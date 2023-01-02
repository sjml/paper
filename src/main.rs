use clap::{arg, Command};

mod errors;
mod project_setup;
mod util;

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

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("new", sub_matches)) => {
            match project_setup::new_project(
                sub_matches
                    .get_one::<String>("PROJECT_NAME")
                    .expect("required"),
            ) {
                Ok(p) => p,
                Err(_) => std::process::exit(1),
            };
        }
        Some(("init", _)) => match project_setup::init_project() {
            Ok(p) => p,
            Err(_) => std::process::exit(1),
        },
        _ => unreachable!(),
    }
}
