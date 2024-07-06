use anyhow::{bail, Result};
use yaml_rust::{yaml, Yaml};

use crate::util;

pub struct PaperMeta {
    _root: Yaml,
}

impl PaperMeta {
    pub fn new() -> Result<Self> {
        let meta_path = util::find_meta(None)?;
        match util::load_yml_file(&meta_path) {
            Ok(data) => {
                let mut me = PaperMeta { _root: data };
                match me.get_string(&["data", "date"]) {
                    Some(s) => {
                        if s == "[DATE]" {
                            me.set_null(&["data", "date"])?
                        }
                    }
                    None => me.set_null(&["data", "date"])?,
                }
                Ok(me)
            }
            Err(e) => Err(e),
        }
    }

    fn fetch_node(&self, keychain: &[&str]) -> Option<Yaml> {
        if keychain.is_empty() {
            return Some(self._root.clone());
        }
        let mut curr = self._root.as_hash().unwrap();
        for (i, k) in keychain.iter().enumerate() {
            let key = Yaml::String(k.to_string());
            match curr.get(&key) {
                None => return None,
                Some(val) => {
                    if i == keychain.len() - 1 {
                        return Some(val.clone());
                    }
                    match val {
                        Yaml::Hash(vh) => {
                            curr = vh;
                        }
                        _ => return None,
                    }
                }
            }
        }
        None
    }

    pub fn contains(&self, keychain: &[&str]) -> bool {
        self.fetch_node(keychain).is_some()
    }

    pub fn get_bool(&self, keychain: &[&str]) -> Option<bool> {
        self.fetch_node(keychain).and_then(|n| n.as_bool())
    }

    pub fn get_int(&self, keychain: &[&str]) -> Option<i64> {
        self.fetch_node(keychain).and_then(|n| n.as_i64())
    }

    pub fn get_float(&self, keychain: &[&str]) -> Option<f64> {
        self.fetch_node(keychain).and_then(|n| n.as_f64())
    }

    pub fn get_string(&self, keychain: &[&str]) -> Option<String> {
        self.fetch_node(keychain).and_then(|n| n.into_string())
    }

    pub fn get_vec_string(&self, keychain: &[&str]) -> Option<Vec<String>> {
        match self.fetch_node(keychain) {
            None => None,
            Some(node) => match node {
                Yaml::Array(narr) => {
                    let mut v: Vec<String> = Vec::new();
                    for el in narr {
                        match el.into_string() {
                            Some(s) => v.push(s),
                            None => return None,
                        }
                    }
                    Some(v)
                }
                _ => None,
            },
        }
    }

    // HACKHACK
    pub fn get_data_pairs(&self, keychain: &[&str]) -> Option<Vec<(String, String)>> {
        match self.fetch_node(keychain) {
            None => None,
            Some(node) => match node {
                Yaml::Hash(nh) => {
                    let mut vec = Vec::new();
                    for (key, val) in nh {
                        match key.into_string() {
                            None => {}
                            Some(k) => match val.into_string() {
                                None => {}
                                Some(v) => {
                                    let el = (k, v);
                                    vec.push(el);
                                }
                            },
                        }
                    }
                    Some(vec)
                }
                _ => None,
            },
        }
    }

    // dives through nested HashMaps following a chain of string keys
    //   returns the end of the chain so something can be inserted to it,
    //   creating new nested maps along the way as needed.
    fn get_end_of_chain<'a>(
        mut data: &'a mut Yaml,
        keychain: impl Iterator<Item = String>,
    ) -> Result<&'a mut Yaml> {
        for key in keychain {
            match data {
                Yaml::Hash(vh) => {
                    data = vh
                        .entry(Yaml::String(key))
                        .or_insert_with(|| Yaml::Hash(yaml::Hash::new()))
                }
                _ => bail!("Non-hash key {} in keychain.", key),
            }
        }
        Ok(data)
    }

    fn set_node(&mut self, keychain: &[&str], value: Yaml) -> Result<()> {
        if keychain.is_empty() {
            bail!("Cannot set value without at least one key.")
        }

        let last = keychain[keychain.len() - 1];
        match PaperMeta::get_end_of_chain(
            &mut self._root,
            keychain
                .iter()
                .copied()
                .take(keychain.len() - 1)
                .map(String::from),
        )? {
            Yaml::Hash(vh) => {
                vh.insert(Yaml::String(last.to_string()), value);
            }
            _ => bail!(""),
        }

        Ok(())
    }

    pub fn set_bool(&mut self, keychain: &[&str], value: bool) -> Result<()> {
        self.set_node(keychain, Yaml::Boolean(value))
    }

    pub fn set_int(&mut self, keychain: &[&str], value: i64) -> Result<()> {
        self.set_node(keychain, Yaml::Integer(value))
    }

    pub fn set_float(&mut self, keychain: &[&str], value: f64) -> Result<()> {
        // yaml_rust stores floats as strings ¯\_(ツ)_/¯
        self.set_node(keychain, Yaml::Real(value.to_string()))
    }

    pub fn set_string(&mut self, keychain: &[&str], value: &str) -> Result<()> {
        self.set_node(keychain, Yaml::String(value.to_string()))
    }

    pub fn set_vec_string(&mut self, keychain: &[&str], value: Vec<String>) -> Result<()> {
        let mut vecval: Vec<Yaml> = Vec::new();
        for s in value {
            vecval.push(Yaml::String(s));
        }
        let arrval = Yaml::Array(vecval);
        self.set_node(keychain, arrval)
    }

    pub fn set_null(&mut self, keychain: &[&str]) -> Result<()> {
        self.set_node(keychain, Yaml::Null)
    }
}
