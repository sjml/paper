use std::fs;
use std::io::Write;
use std::path;
use std::str::FromStr;
use std::time::UNIX_EPOCH;

use anyhow::{bail, Context, Result};
use regex::Regex;
use serde_json::{self, Value};
use walkdir::WalkDir;

use crate::config::CONFIG;
use crate::docx;
use crate::formats::{self, OutputFormat};
use crate::latex;
use crate::metadata::PaperMeta;
use crate::pandoc_wrap;
use crate::subprocess;
use crate::util;

pub fn get_content_file_list() -> Vec<String> {
    let mut content_files = WalkDir::new(&CONFIG.get().content_directory_name)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .filter(|entry| entry.path().file_name().unwrap() != ".DS_Store")
        .filter(|entry| entry.path().extension().unwrap_or(std::ffi::OsStr::new("")) == "md")
        .map(|entry| entry.path().as_os_str().to_string_lossy().to_string())
        .collect::<Vec<String>>();
    content_files.sort();

    content_files
}

fn get_content_timestamp() -> Result<u64> {
    // if there are no changes in the content directory, return the last commit time
    let content_status = subprocess::run_command(
        "git",
        &[
            "status",
            &CONFIG.get().content_directory_name,
            "--porcelain",
        ],
        None,
        false,
    )?;
    if content_status.is_empty() {
        let git_commit_time = subprocess::run_command(
            "git",
            &[
                "log",
                "-1",
                "--format=%ct",
                "--",
                &CONFIG.get().content_directory_name,
            ],
            None,
            false,
        )?;
        let commit_time: u64 = git_commit_time
            .trim()
            .parse()
            .context("Could not convert git commit time to u64")?;
        return Ok(commit_time);
    }

    // otherwise return the most recent mod time in the content directory
    let mut most_recent: u64 = 0;
    for entry in WalkDir::new(&CONFIG.get().content_directory_name) {
        let entry = entry.context("Invalid directory entry in walkdir")?;
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

    let whitespace_search = Regex::new(r"\s").context("Could not compile regex")?;

    // pull the first (or only) author's last name
    let author_splits = meta
        .get_string(&["data", "author"])
        .expect("No author in metadata.");
    let authors: Vec<&str> = author_splits.split(',').map(|s| s.trim()).collect();
    let author_label = authors.first().unwrap().split(' ').last().unwrap();
    match meta.get_string(&["data", "class_mnemonic"]) {
        Some(mnemonic) => {
            filename = format!(
                "{}_{}",
                author_label,
                whitespace_search.replace(&mnemonic, "")
            );
        }
        None => {
            filename = author_label.to_string();
        }
    }

    let assignment = util::get_assignment()?;
    let assignment_underscored = whitespace_search.replace(&assignment, "_");
    filename = format!("{}_{}", filename, assignment_underscored);

    Ok(filename)
}

pub fn build(
    output_format: &formats::OutputFormat,
    of_specified: bool,
    docx_revision: i64,
) -> Result<()> {
    util::ensure_paper_dir()?;

    let mut meta = PaperMeta::new()?;

    let mut of = output_format.clone();
    if !of_specified {
        of = match meta.get_string(&["default_format"]) {
            Some(df) => formats::OutputFormat::from_str(&df)?,
            None => of,
        };
    }

    if CONFIG.get().verbose {
        println!("Building for format {:?}.", of);
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

    let mut pandoc_args: Vec<String> = vec![
        "--from".to_string(),
        CONFIG.get().pandoc_input_format.clone(),
        "--metadata-file".to_string(),
        util::find_meta(None)?
            .to_str()
            .context("Meta path could not be converted to string.")?
            .to_string(),
        "--resource-path".to_string(),
        CONFIG.get().content_directory_name.clone(),
    ];

    let mut builder: Box<dyn formats::Builder>;
    match of {
        OutputFormat::Docx => {
            meta.set_int(&["docx", "revision"], docx_revision)?;
            builder = Box::<docx::DocxBuilder>::default();
        }
        OutputFormat::LaTeX => {
            builder = Box::<latex::LatexBuilder>::default();
        }
        OutputFormat::LaTeXPdf => {
            builder = Box::<latex::LatexPdfBuilder>::default();
        }
        _ => {
            // wrong, just leaving here now until the rest of the arms are filled
            builder = Box::<latex::LatexBuilder>::default();
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
            meta.set_string(&["filename"], &generated)?;
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
    let lua_filters = fs::read_dir(&filter_dir)
        .with_context(|| format!("Could not read {:?}", &filter_dir))?
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
        if !(meta.get_bool(&["use_ibid"]).unwrap_or(false)) {
            pandoc_args.push(
                ".paper_resources/chicago-fullnote-bibliography-short-title-subsequent.csl"
                    .to_string(),
            );
        } else {
            pandoc_args
                .push(".paper_resources/chicago-fullnote-bibliography-with-ibid.csl".to_string());
        }
        for bs in bib_sources {
            pandoc_args.push("--bibliography".to_string());

            let mut source = bs.clone();
            if source.starts_with('~') {
                source = source.replacen(
                    '~',
                    std::env::var("HOME")
                        .context("Could not get $HOME env var")?
                        .as_str(),
                    1,
                );
            }
            pandoc_args.push(source);
        }
    } else if CONFIG.get().verbose {
        println!("No citation processing.");
    }

    let post_lua_filters = fs::read_dir(&filter_dir)
        .with_context(|| format!("Could not read {:?}", &filter_dir))?
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

    let sentinels = &meta.get_vec_string(&["sentinels"]).unwrap_or(vec![]);
    for content_file in builder.get_file_list() {
        let content = fs::read_to_string(content_file.clone())
            .with_context(|| format!("ERROR: Could not read content file {:?}", content_file))?;
        for s in sentinels {
            if content.contains(s) {
                eprintln!("[WARNING] {:?} contains sentinel '{}'", content_file, s);
            }
        }
        pandoc_args.push(content_file);
    }

    if CONFIG.get().verbose {
        println!("Invoking pandoc with:");
        println!("\t{}", pandoc_args.join(" "));
    }

    let output = subprocess::run_command(
        &pandoc_wrap::get_pandoc_exe_path()?,
        pandoc_args.as_slice(),
        None,
        true,
    )?;
    if CONFIG.get().verbose {
        println!("{}", output);
    }

    let logs = builder.finish_file(output_file_path.as_path(), &meta)?;

    record_build_data(&logs, &meta)?;

    Ok(())
}

fn record_build_data(log_lines: &[String], meta: &PaperMeta) -> Result<()> {
    util::stamp_local_dir()?;

    if let Some(bib_paths) = meta.get_vec_string(&["sources"]) {
        let mut cited_refence_keys = vec![];

        let lua_path = CONFIG
            .get()
            .resources_path
            .join("scripts")
            .join("ref_list.lua");

        let mut args = vec![
            "--to".to_string(),
            lua_path.to_string_lossy().to_string(),
            "--metadata-file".to_string(),
            util::find_meta(None)?
                .to_str()
                .context("Meta path could not be converted to string.")?
                .to_string(),
            "--citeproc".to_string(),
        ];

        let mut bpp_strings = vec![];
        for bp in bib_paths {
            let mut bp_local = bp.clone();
            if bp.starts_with('~') {
                bp_local = format!(
                    "{}{}",
                    std::env::var("HOME").context("No $HOME variable set.")?,
                    &bp_local[1..]
                );
            }
            bpp_strings.push(bp_local.clone());
            let bpp = path::Path::new(&bp_local);
            if !bpp.exists() {
                bail!("No such file for bibliography source: {}", bp);
            }
            args.extend_from_slice(&["--bibliography".to_string(), bp_local]);
        }
        args.extend_from_slice(&get_content_file_list());

        let ref_str =
            subprocess::run_command(&pandoc_wrap::get_pandoc_exe_path()?, &args, None, false)?;
        let ref_str = ref_str.trim();
        cited_refence_keys.extend(ref_str.split('\n').map(|s| s.to_string()));

        let mut refs: Vec<Value> = vec![];
        for bpps in bpp_strings {
            let bpp = path::Path::new(&bpps);
            let mut csl_args = vec!["--to", "csljson"];
            if bpp.extension().unwrap_or(std::ffi::OsStr::new("")) == "json" {
                csl_args.extend_from_slice(&["--from", "csljson"]);
            }
            csl_args.push(&bpps);
            let source_data_text = subprocess::run_command(
                &pandoc_wrap::get_pandoc_exe_path()?,
                &csl_args,
                None,
                false,
            )?;
            let source_data: Value = serde_json::from_str(&source_data_text)
                .context("Could not parse JSON from sources data")?;
            match source_data {
                Value::Array(source_list) => {
                    for entry in source_list {
                        match entry {
                            Value::Object(entry_obj) => {
                                if let Some(id_val) = entry_obj.get("id") {
                                    match id_val {
                                        Value::String(id_str) => {
                                            if cited_refence_keys.contains(id_str) {
                                                refs.push(Value::Object(entry_obj));
                                            }
                                        }
                                        _ => bail!("Invalid CSL JSON in {}", bpps),
                                    }
                                } else {
                                    bail!("Invalid CSL JSON in {}", bpps);
                                }
                            }
                            _ => bail!("Invalid CSL JSON in {}", bpps),
                        }
                    }
                }
                _ => bail!("Invalid CSL JSON in {}", bpps),
            }
        }
        if !refs.is_empty() {
            let refs_val = Value::Array(refs);
            let refs_str = serde_json::to_string_pretty(&refs_val).with_context(|| {
                format!("Could not make pretty string from JSON {:?}", &refs_val)
            })?;
            let csl_out_path = std::env::current_dir()
                .context("Could not get current directory")?
                .join(".paper_data")
                .join("cited_references.json");
            fs::write(csl_out_path, refs_str)?;
        }
    }

    let mut out_file = fs::File::create(
        std::env::current_dir()
            .context("Could not get current directory")?
            .join(".paper_data")
            .join("build_environment.txt"),
    )
    .context("Could not create build data output file")?;

    let separator = str::repeat("#", 60);

    writeln!(out_file, "{}", util::get_paper_version_stamp())
        .context("Could not write to build data output file")?;
    writeln!(out_file, "{}", separator).context("Could not write to build data output file")?;

    let dep_str = env!("PAPER_RUST_DEPS")
        .split("||||||")
        .collect::<Vec<&str>>()
        .join("\n");
    writeln!(out_file, "{}", dep_str).context("Could not write to build data output file")?;
    writeln!(out_file, "{}", separator).context("Could not write to build data output file")?;

    let pandoc_vers = subprocess::run_command(
        &pandoc_wrap::get_pandoc_exe_path()?,
        &["--version"],
        None,
        false,
    )
    .context("Could not get pandoc version string")?;
    writeln!(out_file, "{}", pandoc_vers).context("Could not write to build data output file")?;

    writeln!(out_file, "{}", separator).context("Could not write to build data output file")?;

    write!(out_file, "{}", log_lines.join("\n"))
        .context("Could not write to build data output file")?;

    Ok(())
}
