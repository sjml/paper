use std::fs;
use std::path;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use chrono::prelude::*;
use yaml_rust::{yaml, Yaml, YamlLoader};

use crate::config::CONFIG;
use crate::metadata;

pub static LIB_NAME: &str = "SJML Paper";
pub static LIB_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn get_paper_version_stamp() -> String {
    let mut version = format!("{} v{}", LIB_NAME, LIB_VERSION);
    let git_rev = env!("PAPER_GIT_REV");
    if !git_rev.is_empty() {
        version = format!("{}\n{}", version, git_rev);
    }

    let mut rustc_ver = env!("PAPER_RUSTC_VERSION_STR");
    if rustc_ver.is_empty() {
        rustc_ver = "<<unknown rustc>>";
    }
    version = format!("{}\nBuilt by {}", version, rustc_ver);

    version
}

pub fn stamp_local_dir() -> Result<()> {
    let data_path = std::env::current_dir()
        .context("Could not get current directory")?
        .join(".paper_data");
    if !data_path.exists() {
        fs::create_dir_all(&data_path)
            .with_context(|| format!("Could not create directory path {:?}", &data_path))?;
    }
    let vers = get_paper_version_stamp();
    fs::write(data_path.join("last_paper_version.txt"), vers)
        .context("Could not stamp local dir")?;

    Ok(())
}

pub fn get_date_string(meta: &metadata::PaperMeta) -> Result<String> {
    let date: DateTime<Local> = match meta.get_string(&["data", "date"]) {
        None => return Ok(String::new()),
        Some(date_string) => {
            let due = NaiveDate::parse_from_str(&date_string, "%Y-%m-%d")
                .with_context(|| format!("Could not parse NaiveDate from {}", &date_string))?;
            let due = due.and_time(NaiveTime::from_hms_opt(0, 1, 0).unwrap());
            match Local.from_local_datetime(&due) {
                chrono::LocalResult::Single(s) => s,
                _ => Local::now(),
            }
        }
    };

    // because one of my example documents has a due date of 33 AD, and what's
    //  the point of making your own system if you can't have a little Easter egg?
    //  :D
    let mut year_str = date.year().to_string();
    if year_str == "33" {
        year_str = "A.U.C. 786".to_string();
    }
    let out_string = format!("{}, {}", date.format("%B %-d"), year_str);
    Ok(out_string)
}

pub fn load_yml_file(path: &PathBuf) -> Result<Yaml> {
    let file_contents = fs::read_to_string(path.clone())
        .with_context(|| format!("Could not read file at {:?}", path))?;
    let yml = YamlLoader::load_from_str(&file_contents)
        .with_context(|| format!("Invalid YAML file at {:?}", path))?;

    let yml = yml
        .into_iter()
        .filter(|y| !y.is_null())
        .collect::<Vec<Yaml>>();

    if yml.is_empty() {
        // is this possible?
        bail!("YAML file at {:?} contains no documents.", path);
    }
    if yml.len() > 1 {
        bail!("YAML file at {:?} contains too many documents.", path);
    }
    let doc = yml[0].clone();

    Ok(doc)
}

// NB: can't do a simple naive merge here because:
//     - want to merge recursively if both values are also hashes
//     - if they're both lists we want to extend, not overwrite
//     - if it's a default value ("[SOMETHING]"), leave it alone
// There are a lot of .clone() calls here, but I'm not sure how else to
//    make the borrow checker happy. :(
pub fn merge_yaml_hash(target: &mut yaml::Hash, new_hash: &yaml::Hash) {
    for (k, v) in new_hash {
        match target.get_mut(k) {
            Some(target_v) => match v {
                Yaml::Hash(vh) => match target_v {
                    Yaml::Hash(tvh) => {
                        let mut new_v = tvh.clone();
                        merge_yaml_hash(&mut new_v, vh);
                        target[k] = Yaml::Hash(new_v);
                    }
                    _ => {}
                },
                Yaml::Array(va) => match target_v {
                    Yaml::Array(tva) => {
                        let mut new_v = tva.clone();
                        new_v.extend_from_slice(va);
                        target[k] = Yaml::Array(new_v);
                    }
                    _ => {
                        target[k] = v.clone();
                    }
                },
                Yaml::String(vs) => {
                    if vs.starts_with('[') && vs.ends_with(']') {
                        continue;
                    } else {
                        target[k] = Yaml::String(vs.to_string());
                    }
                }
                _ => {
                    target.insert(k.clone(), v.clone());
                }
            },
            None => {
                target.insert(k.clone(), v.clone());
            }
        }
    }
}

pub fn find_meta(base: Option<&Path>) -> Result<path::PathBuf> {
    let options = vec![
        path::Path::new("paper_meta.yml"),
        path::Path::new("_paper_meta.yml"),
        path::Path::new(".paper_meta.yml"),
    ];

    let base_path = match base {
        Some(p) => p.to_path_buf(),
        None => std::env::current_dir().context("Could not get current directory")?,
    };

    for try_path in options {
        let joined = base_path.join(try_path);
        if joined.exists() {
            return Ok(joined);
        }
    }
    bail!("Could not find valid meta file.");
}

pub fn ensure_paper_dir() -> Result<()> {
    match find_meta(None) {
        Ok(_) => {}
        Err(_) => {
            bail!("Invalid paper directory; no meta file found.");
        }
    }
    // don't have manual file list anymore since we're checking for the meta file
    //   right above here; leave the machinery in place, though, in case we want
    //   to check something else.
    let files: Vec<&Path> = vec![];
    let dirs = vec![
        path::Path::new(".paper_resources"),
        path::Path::new(&CONFIG.get().content_directory_name),
    ];

    for f in files {
        if !f.exists() {
            bail!(
                "Invalid paper directory; expected file {:?}, which does not exist.",
                f
            );
        }
        if !f.is_file() {
            bail!("Invalid paper directory; {:?} is not a file.", f);
        }
    }

    for d in dirs {
        if !d.exists() {
            bail!(
                "Invalid paper directory; expected directory {:?}, which does not exist.",
                d
            );
        }
        if !d.is_dir() {
            bail!("Invalid paper directory; {:?} is not a directory.", d);
        }
        let subpaths = d
            .read_dir()
            .with_context(|| format!("Could not read contents of {:?}", d))?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .collect::<Vec<_>>();
        if subpaths.is_empty() {
            bail!("Invalid paper directory; {:?} contains no files.", d);
        }
    }

    Ok(())
}

pub fn get_assignment() -> Result<String> {
    let meta = metadata::PaperMeta::new()?;
    match meta.get_string(&["assignment"]) {
        Some(s) => Ok(s),
        None => {
            let cwd = std::env::current_dir().context("Current path is invalid.")?;
            let base = cwd
                .file_name()
                .context("Couldn't get basename of current path.")?
                .to_string_lossy();
            Ok(base.to_string())
        }
    }
}
