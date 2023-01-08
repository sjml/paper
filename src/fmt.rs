use std::fs;

use anyhow::Result;
use walkdir;

use crate::config::CONFIG;
use crate::subprocess;
use crate::util;

pub fn fmt(wrap: bool, columns: u32) -> Result<()> {
    util::ensure_paper_dir()?;

    let content_files = walkdir::WalkDir::new("./content")
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .map(|entry| entry.path().as_os_str().to_string_lossy().to_string())
        .collect::<Vec<String>>();

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
        let md_out = subprocess::run_command("pandoc", &args)?;
        let md_curr = fs::read_to_string(&cf)?;
        let md_curr = md_curr.trim();
        if md_out.trim() != md_curr.trim() {
            if CONFIG.get().verbose {
                println!("Reformatting {}...", cf);
            }
            fs::write(cf, md_out)?;
        }
    }

    Ok(())
}
