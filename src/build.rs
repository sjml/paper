use anyhow::Result;

use crate::util::ensure_paper_dir;

pub fn build(output_format: &str, docx_revision: i128) -> Result<()> {
    ensure_paper_dir()?;

    println!("Building {} at rev {}...", output_format, docx_revision);
    Ok(())
}
