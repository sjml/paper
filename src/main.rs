use std::fs;
use std::fmt;
use std::path::Path;
use std::io::Write;

use clap::{Command, arg};
use include_dir::{include_dir, Dir};

#[derive(Debug)]
pub enum PaperError {
    DirectoryAlreadyExists,
    DirectoryNotEmpty,
}
impl fmt::Display for PaperError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PaperError::DirectoryAlreadyExists => write!(f, "directory already exists"),
            PaperError::DirectoryNotEmpty => write!(f, "directory not empty"),
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
}

// fn push_args() -> Vec<clap::Arg> {
//     vec![arg!(-m --message <MESSAGE>)]
// }

static PROJECT_TEMPLATE: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/resources/project_template");

fn init_project() -> Result<(), PaperError> {
    let proj_path = std::env::current_dir().unwrap();
    if !(proj_path.read_dir().unwrap().next().is_none()) {
        eprintln!("Directory needs to be empty to initialize project.");
        return Err(PaperError::DirectoryNotEmpty);
    }

    PROJECT_TEMPLATE.extract(proj_path).unwrap();

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

    return init_project()
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("new", sub_matches)) => {
            // println!(
                // "Creating new project called {}",
                // sub_matches.get_one::<String>("PROJECT_NAME").expect("required")
            // );
            // new_project(sub_matches.get_one::<String>("PROJECT_NAME").expect("required"))
            match new_project(sub_matches.get_one::<String>("PROJECT_NAME").expect("required")) {
                Ok(p) => p,
                Err(_) => std::process::exit(1)
            };

        }
        _ => unreachable!(),
    }
}
