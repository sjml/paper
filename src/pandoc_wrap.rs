use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::{bail, Context, Result};
use reqwest;
use tempfile;

use crate::config::CONFIG;
use crate::subprocess;

// locks pandoc to a particular version so we don't have to chase
//   down differences immediately when the system version changes

const PANDOC_LOCKED_VERSION: &str = "3.3";

pub fn get_pandoc_exe_path() -> Result<String> {
    // check if downloaded version exists
    let exe_path = CONFIG
        .get()
        .resources_path
        .join(format!("pandoc-{}", PANDOC_LOCKED_VERSION))
        .join("pandoc");
    if exe_path.exists() && exe_path.is_file() {
        match exe_path.into_os_string().into_string() {
            Ok(p) => return Ok(p),
            Err(orig) => bail!("Could not turn OsString `{:?}` into String", orig),
        }
    }

    // see if we can download it
    let (os_name, ext) = match env::consts::OS {
        "macos" => ("macOS", ".zip"),
        "linux" => ("linux", ".tar.gz"),
        _ => bail!("Unsupported OS!"),
    };
    if os_name != "macOS" {
        eprintln!("Not tested on Linux, but we're going to try...");
    }
    let arch = match env::consts::ARCH {
        "x86_64" => match os_name {
            "macOS" => "x86_64",
            "linux" => "amd64",
            _ => unreachable!(),
        },
        "aarch64" => "arm64",
        _ => bail!("Unsupported architecture!"),
    };
    let file_basename = match os_name {
        "macOS" => format!("pandoc-{}-{}-{}", PANDOC_LOCKED_VERSION, arch, os_name),
        "linux" => format!("pandoc-{}-{}-{}", PANDOC_LOCKED_VERSION, os_name, arch),
        _ => unreachable!(),
    };
    let filename = format!("{}{}", file_basename, ext);
    let dl_path = format!(
        "https://github.com/jgm/pandoc/releases/download/{}/{}",
        PANDOC_LOCKED_VERSION, filename
    );

    println!(
        "Attempting to download Pandoc v{}...",
        PANDOC_LOCKED_VERSION
    );
    let tmp_dir = tempfile::Builder::new().prefix("paper-pandoc").tempdir()?;
    let tmp_dir_str = tmp_dir
        .path()
        .as_os_str()
        .to_str()
        .expect("Could not turn OsStr into str");
    let mut res = reqwest::blocking::get(&dl_path)
        .with_context(|| format!("Could not download `{}`", dl_path))?;
    let output_file_path = tmp_dir.path().join(filename);
    let mut destination = fs::File::create(&output_file_path)?;
    io::copy(&mut res, &mut destination)?;

    let output_file_path_str = match output_file_path.into_os_string().into_string() {
        Ok(str) => str,
        Err(orig) => bail!("Could not turn OsString `{:?}` into String", orig),
    };

    let unzipped_path = match ext {
        ".zip" => {
            let unzip_output = subprocess::run_command(
                "unzip",
                &[
                    output_file_path_str,
                    "-d".to_string(),
                    tmp_dir_str.to_string(),
                ],
                None,
                false,
            )
            .context("Could not unzip downloaded Pandoc. Weird!")?;
            let exe = unzip_output
                .split("\n")
                .into_iter()
                .find(|&line| {
                    line.trim().starts_with("inflating: ") && line.trim().ends_with("/bin/pandoc")
                })
                .expect("Couldn't find executable in zip")
                .trim()
                .strip_prefix("inflating: ")
                .expect("Couldn't strip prefix from exe line");
            PathBuf::from_str(exe)?
        }
        ".tar.gz" => {
            let untar_output = subprocess::run_command(
                "tar",
                &["-zxf", &output_file_path_str, "-C", tmp_dir_str],
                None,
                false,
            )
            .context("Could not unzip downloaded Pandoc. Weird!")?;
            let exe = untar_output
                .split("\n")
                .into_iter()
                .find(|&line| line.trim().starts_with("x ") && line.trim().ends_with("/bin/pandoc"))
                .expect("Couldn't find executable in zip")
                .trim()
                .strip_prefix("x ")
                .expect("Couldn't strip prefix from exe line");
            tmp_dir.path().join(exe)
        }
        _ => {
            bail!("Unsupported extension: {}", ext);
        }
    };
    let parent = match exe_path.parent() {
        Some(p) => p,
        None => bail!("No parent for exe path? `{:?}`", exe_path),
    };
    fs::create_dir_all(parent)?;
    fs::rename(unzipped_path, &exe_path)?;
    let exe_str = match exe_path.into_os_string().into_string() {
        Ok(str) => str,
        Err(orig) => bail!("Could not turn OsString `{:?}` into String", orig),
    };

    Ok(exe_str)
}
