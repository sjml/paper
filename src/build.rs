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
    let mut pandoc_args = vec![
        "--from", &CONFIG.get().pandoc_input_format,
        "--metadata-file", "./paper_meta.yml",
        "--resource-path", "./content",
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
    pandoc_args.extend(&[
        "--output",
        output_file_path
            .as_path()
            .to_str()
            .context("Can't unwrap output file path.")
            .unwrap(),
    ]);

    Ok(())
}
