use std::fs;
use std::path;
use std::time::UNIX_EPOCH;

use anyhow::{Context, Result};
use regex::Regex;
use walkdir;

use crate::config::CONFIG;
use crate::formats::{self, OutputFormat};
use crate::metadata::PaperMeta;
use crate::subprocess;
use crate::util;

fn get_content_timestamp() -> Result<u64> {
    // if there are no changes in the content directory, return the last commit time
    let content_status = subprocess::run_command("git", &["status", "./content", "--porcelain"])?;
    if content_status.len() == 0 {
        let git_commit_time = subprocess::run_command("git", &["log", "-1", "--format=%ct"])?;
        let commit_time: u64 = git_commit_time.trim().parse()?;
        return Ok(commit_time);
    }

    // otherwise return the most recent mod time in the content directory
    let mut most_recent: u64 = 0;
    for entry in walkdir::WalkDir::new("./content") {
        let entry = entry?;
        let md = entry
            .metadata()
            .with_context(|| format!("Could not get metadata for {:?}", entry.path()))?;
        let modified = md
            .modified()
            .unwrap()
            .duration_since(UNIX_EPOCH)
            .context("Invalid modification time or *very* old file.")?;
        most_recent = std::cmp::max(most_recent, modified.as_secs());
    }
    Ok(most_recent)
}

fn generate_filename(meta: &PaperMeta) -> Result<String> {
    let mut filename;

    let whitespace_search = Regex::new(r"\s")?;

    // pull the first (or only) author's last name
    let author_splits = meta
        .get_string(&["data", "author"])
        .expect("No author in metadata.");
    let authors: Vec<&str> = author_splits.split(",").map(|s| s.trim()).collect();
    let author_label = authors.first().unwrap().split(" ").last().unwrap();
    match meta.get_string(&["data", "class_mnemonic"]) {
        Some(mnemonic) => {
            filename = format!(
                "{}_{}",
                author_label,
                whitespace_search.replace(&mnemonic, "")
            );
        }
        None => {
            filename = format!("{}", author_label);
        }
    }

    let assignment = util::get_assignment()?;
    let assignment_underscored = whitespace_search.replace(&assignment, "_");
    filename = format!("{}_{}", filename, assignment_underscored);

    Ok(filename)
}

pub fn build(output_format: formats::OutputFormat, docx_revision: i64) -> Result<()> {
    util::ensure_paper_dir()?;

    let mut meta = PaperMeta::new()?;

    if CONFIG.get().verbose {
        println!("Building for format {:?}.", output_format);
    }

    let content_timestamp = get_content_timestamp()?;
    if CONFIG.get().verbose {
        println!("Setting source epoch to {}.", content_timestamp);
    }
    std::env::set_var("SOURCE_DATE_EPOCH", content_timestamp.to_string());

    let out_path = path::Path::new(&CONFIG.get().output_directory_name);
    if !out_path.exists() {
        fs::create_dir(out_path).context("Could not create ouptput directory.")?;
    }

    #[rustfmt::skip]
    let mut pandoc_args: Vec<String> = vec![
        "--from".to_string(), CONFIG.get().pandoc_input_format.clone(),
        "--metadata-file".to_string(), "./paper_meta.yml".to_string(),
        "--resource-path".to_string(), "./content".to_string(),
    ];

    let mut builder: Box<dyn formats::Builder>;
    match output_format {
        OutputFormat::Docx | OutputFormat::DocxPdf => {
            meta.set_int(&["docx", "revision"], docx_revision)?;
            builder = Box::new(formats::DocXBuilder::default());
        }
        _ => {
            // wrong, just leaving here now until the rest of the arms are filled
            builder = Box::new(formats::DocXBuilder::default());
        }
    }

    builder.prepare(&mut pandoc_args, &meta)?;

    let filename = match meta.get_string(&["filename"]) {
        Some(fname) => fname,
        None => {
            let generated = generate_filename(&meta)?;
            if CONFIG.get().verbose {
                println!("No filename given; using generated \"{}\".", generated);
            }
            generated
        }
    };
    let output_file_path =
        out_path.join(format!("{}.{}", filename, builder.get_output_file_suffix()));
    pandoc_args.push("--output".to_string());
    pandoc_args.push(
        output_file_path
            .as_path()
            .to_str()
            .context("Can't unwrap output file path.")
            .unwrap()
            .to_string(),
    );

    let filter_dir = path::Path::new(".paper_resources").join("filters");
    let lua_filters = fs::read_dir(&filter_dir)?
        .filter_map(|lf| lf.ok())
        .filter(|lf| {
            lf.file_name()
                .as_os_str()
                .to_string_lossy()
                .starts_with("filter-")
        })
        .collect::<Vec<fs::DirEntry>>();

    for lf in lua_filters {
        pandoc_args.push("--lua-filter".to_string());
        pandoc_args.push(lf.path().to_string_lossy().to_string());
    }

    if let Some(bib_sources) = meta.get_vec_string(&["sources"]) {
        if CONFIG.get().verbose {
            println!("Processing citations...");
        }
        pandoc_args.push("--citeproc".to_string());
        pandoc_args.push("--csl".to_string());
        if !(meta.get_bool(&["use_ibid"]).unwrap_or_else(|| false)) {
            pandoc_args.push(
                "./.paper_resources/chicago-fullnote-bibliography-short-title-subsequent.csl"
                    .to_string(),
            );
        } else {
            pandoc_args
                .push("./.paper_resources/chicago-fullnote-bibliography-with-ibid.csl".to_string());
        }
        for bs in bib_sources {
            pandoc_args.push("--bibliography".to_string());

            let mut source = bs.clone();
            if source.starts_with("~") {
                source = source.replacen("~", std::env::var("HOME")?.as_str(), 1);
            }
            pandoc_args.push(source);
        }

        let post_lua_filters = fs::read_dir(filter_dir)?
            .filter_map(|lf| lf.ok())
            .filter(|lf| {
                lf.file_name()
                    .as_os_str()
                    .to_string_lossy()
                    .starts_with("post-filter-")
            })
            .collect::<Vec<fs::DirEntry>>();

        for lf in post_lua_filters {
            pandoc_args.push("--lua-filter".to_string());
            pandoc_args.push(lf.path().to_string_lossy().to_string());
        }
    } else if CONFIG.get().verbose {
        println!("No citation processing.");
    }

    for content_file in builder.get_file_list() {
        pandoc_args.push(content_file);
    }

    // println!("{:?}", pandoc_args);

    if CONFIG.get().verbose {
        println!("Invoking pandoc with:");
        println!("\t{}", pandoc_args.join(" "));
    }

    subprocess::run_command("pandoc", pandoc_args.as_slice())?;

    builder.finish_file(output_file_path.as_path())?;

    Ok(())
}
