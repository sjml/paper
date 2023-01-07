use std::fs;
use std::io::Write;

use anyhow::{bail, Result};
use dialoguer;

use crate::metadata::PaperMeta;
use crate::subprocess;
use crate::util;
use crate::wc;

const METADATA_START_SENTINEL: &str = "<!-- begin paper metadata -->";
const METADATA_END_SENTINEL: &str = "<!-- end paper metadata -->";

pub fn save() -> Result<()> {
    util::ensure_paper_dir()?;

    let message: String = dialoguer::Input::new()
        .with_prompt("Commit message?")
        .interact_text()?;

    util::stamp_local_dir()?;

    let meta = PaperMeta::new()?;

    let readme_path = std::env::current_dir()?.join("README.md");
    if !readme_path.exists() {
        let mut readme_file = fs::File::create(&readme_path)?;
        match meta.get_string(&["data", "class_mnemonic"]) {
            Some(mnemonic) => {
                writeln!(readme_file, "# {}: {}\n", mnemonic, util::get_assignment()?)?;
            }
            None => {
                writeln!(readme_file, "# {}\n", util::get_assignment()?)?;
            }
        }
        writeln!(readme_file, "{}", METADATA_START_SENTINEL)?;
        writeln!(readme_file, "{}", METADATA_END_SENTINEL)?;
    }

    let readme_text = fs::read_to_string(&readme_path)?;
    let readme_meta_start_idx = readme_text.find(METADATA_START_SENTINEL);
    let readme_meta_end_idx = readme_text.find(METADATA_END_SENTINEL);

    if readme_meta_start_idx.is_some() && readme_meta_end_idx.is_some() {
        let readme_meta_start_idx = readme_meta_start_idx.unwrap();
        let readme_meta_end_idx = readme_meta_end_idx.unwrap() + METADATA_END_SENTINEL.len();

        let readme_before = &readme_text[0..readme_meta_start_idx];
        let readme_after = &readme_text[readme_meta_end_idx..];

        let readme_out_text = format!(
            "{}{}\n{}\n{}{}",
            readme_before,
            METADATA_START_SENTINEL,
            wc::wc_string(false)?,
            METADATA_END_SENTINEL,
            readme_after
        );

        fs::write(readme_path, readme_out_text)?;
    }

    let message = format!("{}\n\nPAPER_DATA\n{}", message, wc::wc_json()?);

    subprocess::run_command("git", &["add", "."])?;
    subprocess::run_command("git", &["commit", "-m", &message])?;

    Ok(())
}

pub fn push() -> Result<()> {
    util::ensure_paper_dir()?;

    let remote = subprocess::run_command("git", &["remote", "-v"])?;
    if remote.is_empty() {
        let meta = PaperMeta::new()?;
        // default_repo = f"{meta['data']['class_mnemonic'].replace(' ', '')} {get_assignment()}"
        let mnemonic = meta.get_string(&["data", "class_mnemonic"]).unwrap_or_default();
        let default_name = format!("{} {}", mnemonic, util::get_assignment()?);
        let default_name = default_name.trim();

        println!("(Note that GitHub will do some mild renaming, so it may not be this exact string.)");
        let repo_name: String = dialoguer::Input::new()
            .with_prompt("What should be the repository name?")
            .default(default_name.into())
            .interact_text()?;
        let is_private = dialoguer::Confirm::new()
            .with_prompt("Private repository?")
            .default(true)
            .interact()?;

        let mut args = vec!["repo", "create", &repo_name, "--source=.", "--push"];
        if is_private {
            args.push("--private");
        }
        subprocess::run_command("gh", &args)?;
    }
    else {
        subprocess::run_command("git", &["push"])?;
    }

    Ok(())
}

pub fn web() -> Result<()> {
    util::ensure_paper_dir()?;

    let remote = subprocess::run_command("git", &["remote", "-v"])?;
    if remote.is_empty() {
        bail!("No remote repository set up.")
    }

    let origin_url = subprocess::run_command("git", &["remote", "get-url", "origin"])?;
    if !origin_url.contains("github.com") {
        // not entirely reliable as you could have a different remote repository containing a string
        //   referencing "github.com" but this is already error-check-y enough for my purposes.
        bail!("This repository is not on GitHub.");
    }

    subprocess::run_command("gh", &["repo", "view", "--web"])?;

    Ok(())
}