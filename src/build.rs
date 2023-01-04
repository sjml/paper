use std::fs;
use std::path;
use std::time::UNIX_EPOCH;

use anyhow::{Context, Result};
use regex::Regex;
use subprocess;
use walkdir;

use crate::config::CONFIG;
use crate::metadata;
use crate::metadata::PaperMeta;
use crate::util;

const OUTPUT_DIRECTORY_NAME: &str = "output";

fn get_content_timestamp() -> Result<u64> {
    // if there are no changes in the content directory, return the last commit time
    let content_status = subprocess::Exec::cmd("git")
        .args(&vec!["status", "./content", "--porcelain"])
        .capture()
        .context("Could not run git status for timestamp.")?;
    if content_status.stdout.len() == 0 {
        let git_commit_time = subprocess::Exec::cmd("git")
            .args(&vec!["log", "-1", "--format=%ct"])
            .capture()
            .context("Could not run git log for timestamp.")?;
        let git_commit_time_str = std::str::from_utf8(&git_commit_time.stdout)
            .context("Invalid UTF-8 sequence in git log for timestamp.")?;
        let commit_time: u64 = git_commit_time_str.trim().parse()?;
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
            filename = format!("{}_{}", author_label, whitespace_search.replace(&mnemonic, ""));
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

pub fn build(output_format: &str, docx_revision: i64) -> Result<()> {
    util::ensure_paper_dir()?;

    let mut meta = metadata::PaperMeta::new()?;

    if output_format.contains("docx") {
        meta.set_int(&["docx", "revision"], docx_revision)?;
    }

    if CONFIG.get().verbose {
        println!("Building for format {}.", output_format);
    }

    let content_timestamp = get_content_timestamp()?;
    if CONFIG.get().verbose {
        println!("Setting source epoch to {}.", content_timestamp);
    }
    std::env::set_var("SOURCE_DATE_EPOCH", content_timestamp.to_string());

    let out_path = path::Path::new(OUTPUT_DIRECTORY_NAME);
    if !out_path.exists() {
        fs::create_dir(OUTPUT_DIRECTORY_NAME).context("Could not create ouptput directory.")?;
    }

    let fn_data_path = ["filename"];
    if !meta.contains(&fn_data_path) {
        let generated = generate_filename(&meta)?;
        meta.set_string(&fn_data_path, &generated)?;
        if CONFIG.get().verbose {
            println!("No filename given; using generated \"{}\".", generated);
        }
    }

    #[rustfmt::skip]
    let cmd = vec!["pandoc",
        "--from", &CONFIG.get().pandoc_input_format,
        "--metadata-file", "./paper_meta.yml",
        "--resource-path", "./content",
    ];

    Ok(())
}
