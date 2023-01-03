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
