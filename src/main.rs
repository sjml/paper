use std::fmt;
use std::fs;
use std::io::Write;
use std::path::Path;

use clap::{arg, Command};
use include_dir::{include_dir, Dir};
use subprocess;
use yaml_rust::yaml;
use yaml_rust::YamlEmitter;
use yaml_rust::{Yaml, YamlLoader};

pub static LIB_NAME: &str = "SJML Paper";
pub static LIB_VERSION: &str = env!("CARGO_PKG_VERSION");

static PROJECT_TEMPLATE: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/resources/project_template");

#[derive(Debug)]
pub enum PaperError {
    DirectoryAlreadyExists,
    DirectoryNotEmpty,
    InvalidYaml,
}
impl fmt::Display for PaperError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PaperError::DirectoryAlreadyExists => write!(f, "directory already exists"),
            PaperError::DirectoryNotEmpty => write!(f, "directory not empty"),
            PaperError::InvalidYaml => write!(f, "YAML file invalid"),
        }
    }
}

fn cli() -> Command {
    Command::new("paper")
        .about("Shane’s little paper-{writing|managing|building} utility\n    <https://github.com/sjml/paper>")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("new")
                .about("Create the scaffolding for a new writing/research project.")
                .arg(arg!(<PROJECT_NAME> "The name of the directory to create for the project."))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("init")
                .about("While in an empty directory, set it up for a project.\n(Called as part of the process for `new`.)")
        )
}

fn load_yml_file(path: &Path) -> Result<Yaml, PaperError> {
    let file_contents = match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(_) => return Err(PaperError::InvalidYaml),
    };
    let yml = match YamlLoader::load_from_str(&file_contents) {
        Ok(parsed) => parsed,
        Err(_) => return Err(PaperError::InvalidYaml),
    };
    let yml = yml
        .into_iter()
        .filter(|y| !y.is_null())
        .collect::<Vec<Yaml>>();
    if yml.len() != 1 {
        return Err(PaperError::InvalidYaml);
    }
    let doc = yml[0].clone();

    Ok(doc)
}

// TODO: have this process the git rev
fn get_paper_version_stamp() -> String {
    let version = format!("{} v{}", LIB_NAME, LIB_VERSION);
    return version;
}

// too many .clone() calls here, but I'm not sure how else to
//    make the borrow checker happy. :(
fn merge_hash(target: &mut yaml::Hash, new_hash: &yaml::Hash) {
    for (k, v) in new_hash {
        if target.contains_key(k) {
            let target_v = target.get_mut(k).unwrap();

            if target_v.as_hash().is_some() && v.as_hash().is_some() {
                let mut new_v = target_v.as_hash().unwrap().clone();
                merge_hash(&mut new_v, v.as_hash().unwrap());
                target[k] = Yaml::Hash(new_v);
            }
            else if v.is_array() {
                if target_v.is_array() {
                    let mut new_v = target_v.as_vec().unwrap().clone();
                    new_v.extend_from_slice(v.as_vec().unwrap().as_slice());
                    target[k] = Yaml::Array(new_v);
                }
                else {
                    target[k] = v.clone();
                }
            }
            else {
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
        }
        else {
            target.insert(k.clone(), v.clone());
        }
    }
}

fn init_project() -> Result<(), PaperError> {
    let proj_path_buf = std::env::current_dir().unwrap();
    if !(proj_path_buf.read_dir().unwrap().next().is_none()) {
        eprintln!("Directory needs to be empty to initialize project.");
        return Err(PaperError::DirectoryNotEmpty);
    }

    PROJECT_TEMPLATE.extract(&proj_path_buf).unwrap();

    let mut meta_chain: Vec<Yaml> = Vec::new();
    let mut current_path: Option<&Path> = Some(proj_path_buf.as_path());
    while current_path.is_some() {
        let meta_path = current_path.unwrap().join("paper_meta.yml");
        if meta_path.exists() {
            let data = load_yml_file(&meta_path).unwrap();
            meta_chain.push(data);
        }
        current_path = current_path.unwrap().parent();
    }
    meta_chain.reverse();

    let mut meta = yaml::Hash::new();
    for m in meta_chain {
        merge_hash(&mut meta, &m.into_hash().unwrap());
    }

    let mut meta_str = String::new();
    let mut yaml_emitter = YamlEmitter::new(&mut meta_str);
    yaml_emitter.dump(&Yaml::Hash(meta)).unwrap();

    let mut meta_output_file = fs::File::create("paper_meta.yml").unwrap();
    write!(meta_output_file, "{}", meta_str).unwrap();

    fs::create_dir("research").unwrap();

    subprocess::Exec::cmd("git")
        .arg("init")
        .stdout(subprocess::NullFile)
        .join()
        .unwrap();
    subprocess::Exec::cmd("git")
        .args(&vec!["add", "."])
        .stdout(subprocess::NullFile)
        .join()
        .unwrap();
    subprocess::Exec::cmd("git")
        .args(&vec![
            "commit",
            "-m",
            &format!(
                "Initial project creation\n---\n{}",
                get_paper_version_stamp()
            ),
        ])
        .stdout(subprocess::NullFile)
        .join()
        .unwrap();

    Ok(())
}

fn new_project(project_name: &str) -> Result<(), PaperError> {
    let project_path = Path::new(project_name);
    if project_path.exists() {
        eprintln!("Project path already exists: {}", project_name);
        return Err(PaperError::DirectoryAlreadyExists);
    }

    println!("Starting new project called '{}'...", project_name);
    fs::create_dir(project_path).unwrap();
    std::env::set_current_dir(project_path).unwrap();

    return init_project();
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("new", sub_matches)) => {
            match new_project(
                sub_matches
                    .get_one::<String>("PROJECT_NAME")
                    .expect("required"),
            ) {
                Ok(p) => p,
                Err(_) => std::process::exit(1),
            };
        }
        Some(("init", _)) => match init_project() {
            Ok(p) => p,
            Err(_) => std::process::exit(1),
        },
        _ => unreachable!(),
    }
}
