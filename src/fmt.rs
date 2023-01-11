use std::fs;

use anyhow::{Context, Result};

use crate::build;
use crate::config::CONFIG;
use crate::subprocess;
use crate::util;

pub fn fmt(wrap: bool, columns: u32) -> Result<()> {
    util::ensure_paper_dir()?;

    let content_files = build::get_content_file_list();

    let col_str = columns.to_string();
    for cf in content_files {
        let mut args = vec![
            "--from",
            &CONFIG.get().pandoc_input_format,
            "--to",
            &CONFIG.get().pandoc_input_format,
        ];
        if wrap {
            args.extend_from_slice(&["--wrap", "auto", "--columns", &col_str]);
        } else {
            args.extend_from_slice(&["--wrap", "preserve"])
        }
        args.push(&cf);
        let md_out = subprocess::run_command("pandoc", &args, None, false)?;
        let md_curr = fs::read_to_string(&cf).context("Couldn't read file to string")?;
        let md_curr = md_curr.trim();
        if md_out.trim() != md_curr.trim() {
            if CONFIG.get().verbose {
                println!("Reformatting {}...", cf);
            }
            fs::write(cf, md_out).context("Couldn't write file")?;
        }
    }

    Ok(())
}
