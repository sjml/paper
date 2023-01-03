use anyhow::Result;
use clap::{arg, value_parser, Command};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

mod build;
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
        )
        .subcommand(
            Command::new("init")
                .about("While in an empty directory, set it up for a project.\n(Called as part of the process for `new`.)")
        )
        .subcommand(
            Command::new("build")
                .about("Generate versions of the paper ready for submission.")
                .arg(
                    arg!(-t --"output-format" <FORMAT> "The desired format of the output file")
                    .value_parser(["docx", "docx+pdf", "latex", "latex+pdf", "json"])
                    .default_value("docx")
                )
                .arg(
                    arg!(--"docx-revision" <NUM> "Revision number for docx output format; if unset or negative, will use the number of times the project was saved.")
                    .value_parser(value_parser!(i128))
                    .default_value("-1")
                )
        )
}

fn _main() -> Result<()> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("new", sub_matches)) => {
            project_setup::new_project(
                sub_matches
                    .get_one::<String>("PROJECT_NAME")
                    .expect("required"),
            )?;
        }
        Some(("init", _)) => {
            project_setup::init_project()?;
        }
        Some(("build", sub_matches)) => {
            build::build(
                sub_matches
                    .get_one::<String>("output-format")
                    .expect("required"),
                *sub_matches
                    .get_one::<i128>("docx-revision")
                    .expect("required"),
            )?;
        }
        _ => unreachable!(),
    }

    Ok(())
}

fn main() {
    if let Err(e) = _main() {
        //// more portable version of this, but using the already-included termcolor:
        // eprintln!("\x1B[0;31m\x1b[1merror\x1b[0m: {:?}", e);
        let mut err_format = ColorSpec::new();
        err_format.set_fg(Some(Color::Red)).set_bold(true);
        let mut stderr = StandardStream::stderr(ColorChoice::Auto);
        let _ = stderr.set_color(&err_format);
        eprint!("error: ");
        let _ = stderr.reset();
        eprintln!("{:?}", e);

        std::process::exit(1);
    }
}
