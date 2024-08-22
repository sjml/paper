use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

use anyhow::{anyhow, bail, Context, Result};
use reqwest;
use tempfile::tempdir;

use crate::config::CONFIG;
use crate::subprocess;

const PANDOC_LOCKED_VERSION: &str = "3.3";

pub fn get_pandoc_exe_path() -> Result<PathBuf> {
    // check if downloaded version exists
    let exe_path = CONFIG
        .get()
        .resources_path
        .join(format!("pandoc-{}", PANDOC_LOCKED_VERSION))
        .join("pandoc");

    if exe_path.is_file() {
        return Ok(exe_path);
    }

    // Determine the OS and architecture
    let (os_name, ext) = match env::consts::OS {
        "macos" => ("macOS", ".zip"),
        "linux" => ("linux", ".tar.gz"),
        _ => bail!("Unsupported OS!"),
    };

    if os_name != "macOS" {
        eprintln!("Not tested on Linux, but we're going to try...");
    }

    let arch = match env::consts::ARCH {
        "x86_64" => {
            if os_name == "macOS" {
                "x86_64"
            } else {
                "amd64"
            }
        }
        "aarch64" => "arm64",
        _ => bail!("Unsupported architecture!"),
    };
    let file_basename = match os_name {
        "macOS" => format!("pandoc-{}-{}-{}", PANDOC_LOCKED_VERSION, arch, os_name),
        "linux" => format!("pandoc-{}-{}-{}", PANDOC_LOCKED_VERSION, os_name, arch),
        _ => unreachable!(),
    };
    let filename = format!("{}{}", file_basename, ext);
    let dl_url = format!(
        "https://github.com/jgm/pandoc/releases/download/{}/{}",
        PANDOC_LOCKED_VERSION, filename
    );

    println!(
        "Attempting to download Pandoc v{}...",
        PANDOC_LOCKED_VERSION
    );

    let tmp_dir = tempdir().context("Could not create temporary directory")?;
    let output_file_path = tmp_dir.path().join(&filename);

    let mut res = reqwest::blocking::get(&dl_url)
        .with_context(|| format!("Could not download `{}`", dl_url))?;
    let mut destination = fs::File::create(&output_file_path)?;
    io::copy(&mut res, &mut destination)?;

    // Unzip or untar the file
    let unzipped_path = match ext {
        ".zip" => {
            let unzip_output = subprocess::run_command(
                "unzip",
                &[
                    output_file_path.to_string_lossy().to_string(),
                    "-d".to_string(),
                    tmp_dir.path().to_string_lossy().to_string(),
                ],
                None,
                false,
            )
            .context("Could not unzip downloaded Pandoc. Weird!")?;

            let exe = unzip_output
                .lines()
                .find(|&line| {
                    line.trim().starts_with("inflating: ") && line.trim().ends_with("/bin/pandoc")
                })
                .and_then(|line| line.trim().strip_prefix("inflating: "))
                .ok_or_else(|| anyhow!("Couldn't find executable in zip"))?;

            PathBuf::from(exe)
        }
        ".tar.gz" => {
            let untar_output = subprocess::run_command(
                "tar",
                &[
                    "-zxf",
                    &output_file_path.to_string_lossy(),
                    "-C",
                    tmp_dir.path().to_string_lossy().as_ref(),
                ],
                None,
                false,
            )
            .context("Could not untar downloaded Pandoc. Weird!")?;

            let exe = untar_output
                .lines()
                .find(|&line| line.starts_with("x ") && line.trim().ends_with("/bin/pandoc"))
                .and_then(|line| line.trim().strip_prefix("x "))
                .ok_or_else(|| anyhow!("Couldn't find executable in tar"))?;

            tmp_dir.path().join(exe)
        }
        _ => bail!("Unsupported extension: {}", ext),
    };

    // Ensure the directory exists and move the file
    if let Some(parent) = exe_path.parent() {
        fs::create_dir_all(parent)?;
    } else {
        bail!("No parent for exe path: {:?}", exe_path);
    }

    fs::rename(&unzipped_path, &exe_path)
        .with_context(|| format!("Failed to move Pandoc executable to {:?}", exe_path))?;

    Ok(exe_path)
}
