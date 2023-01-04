use std::fs;
use std::path;

use anyhow::{Context, Result};

use crate::metadata;
use crate::util;
use crate::config::CONFIG;

const OUTPUT_DIRECTORY_NAME: &str = "output";

pub fn build(output_format: &str, docx_revision: i64) -> Result<()> {
    util::ensure_paper_dir()?;

    let mut meta = metadata::PaperMeta::new()?;

    if output_format.contains("docx") {
        meta.set_int(&["docx", "revision"], docx_revision)?;
    }

    if CONFIG.get().verbose {
        println!("Building for format {}.", output_format);
    }

    let content_timestamp = util::get_content_timestamp()?;
    if CONFIG.get().verbose {
        println!("Setting source epoch to {}.", content_timestamp);
    }
    std::env::set_var("SOURCE_DATE_EPOCH", content_timestamp.to_string());

    let out_path = path::Path::new(OUTPUT_DIRECTORY_NAME);
    if !out_path.exists() {
        fs::create_dir(OUTPUT_DIRECTORY_NAME)
            .context("Could not create ouptput directory.")?;
    }

    Ok(())
}
