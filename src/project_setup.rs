use std::fs;
use std::io::Write;
use std::path::Path;

use include_dir::{include_dir, Dir};
use subprocess;
use yaml_rust::{yaml, Yaml, YamlEmitter, YamlLoader};

use crate::errors::PaperError;
use crate::util;

static PROJECT_TEMPLATE: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/resources/project_template");

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

pub fn init_project() -> Result<(), PaperError> {
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
        util::merge_yaml_hash(&mut meta, &m.into_hash().unwrap());
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
                util::get_paper_version_stamp()
            ),
        ])
        .stdout(subprocess::NullFile)
        .join()
        .unwrap();

    Ok(())
}

pub fn new_project(project_name: &str) -> Result<(), PaperError> {
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
