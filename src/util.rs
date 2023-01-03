use std::path;

use anyhow::{bail, Context, Result};
use yaml_rust::{yaml, Yaml};

pub static LIB_NAME: &str = "SJML Paper";
pub static LIB_VERSION: &str = env!("CARGO_PKG_VERSION");

// TODO: have this process the git rev
pub fn get_paper_version_stamp() -> String {
    let version = format!("{} v{}", LIB_NAME, LIB_VERSION);
    return version;
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
                    if vs.starts_with("[") && vs.ends_with("]") {
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

pub fn ensure_paper_dir() -> Result<()> {
    let files = vec![path::Path::new("./paper_meta.yml")];
    let dirs = vec![
        path::Path::new("./.paper_resources"),
        path::Path::new("./content"),
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
        if subpaths.len() == 0 {
            bail!("Invalid paper directory; {:?} contains no files.", d);
        }
    }

    Ok(())
}
