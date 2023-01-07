use std::fs;
use std::io::Write;

use anyhow::Result;
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
