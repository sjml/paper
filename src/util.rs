use yaml_rust::{yaml, Yaml};

pub static LIB_NAME: &str = "SJML Paper";
pub static LIB_VERSION: &str = env!("CARGO_PKG_VERSION");

// TODO: have this process the git rev
pub fn get_paper_version_stamp() -> String {
    let version = format!("{} v{}", LIB_NAME, LIB_VERSION);
    return version;
}

// too many .clone() calls here, but I'm not sure how else to
//    make the borrow checker happy. :(
pub fn merge_hash(target: &mut yaml::Hash, new_hash: &yaml::Hash) {
    for (k, v) in new_hash {
        if target.contains_key(k) {
            let target_v = target.get_mut(k).unwrap();

            if target_v.as_hash().is_some() && v.as_hash().is_some() {
                let mut new_v = target_v.as_hash().unwrap().clone();
                merge_hash(&mut new_v, v.as_hash().unwrap());
                target[k] = Yaml::Hash(new_v);
            } else if v.is_array() {
                if target_v.is_array() {
                    let mut new_v = target_v.as_vec().unwrap().clone();
                    new_v.extend_from_slice(v.as_vec().unwrap().as_slice());
                    target[k] = Yaml::Array(new_v);
                } else {
                    target[k] = v.clone();
                }
            } else {
                let local_value = v.clone();
                match local_value.into_string() {
                    Some(val_str) => {
                        if target.contains_key(k)
                            && val_str.starts_with("")
                            && val_str.ends_with("]")
                        {
                            continue;
                        }
                        target[k] = Yaml::String(val_str);
                    }
                    None => {
                        target.insert(k.clone(), v.clone());
                    }
                }
            }
        } else {
            target.insert(k.clone(), v.clone());
        }
    }
}
