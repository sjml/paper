use anyhow::{bail, Result};
use clap::{arg, value_parser, Command};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

mod build;
mod config;
mod fmt;
mod formats;
pub mod metadata;
mod project_setup;
mod save;
mod subprocess;
mod util;
mod wc;

fn cli() -> Command {
    Command::new("paper")
        .about("Shane’s little paper-{writing|managing|building} utility\n    <https://github.com/sjml/paper>")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .arg(arg!(-v --verbose "Spam the output log").global(true))
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
            Command::new("dev")
                .about("Set up a project for development work on paper itself.\nDeletes the local `.paper_resources` directory and symlinks the template’s version, so changes here affect the actual program.")
                .hide(true)
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
                    .value_parser(value_parser!(i64))
                    .default_value("-1")
                )
        )
        .subcommand(
            Command::new("save")
                .about("Make a git commit with some extra tracking data.")
        )
        .subcommand(
            Command::new("push")
                .about("Push local git changes to the remote repository, creating one if necessary.")
        )
        .subcommand(
            Command::new("web")
                .about("Open the remote repository’s GitHub site.")
        )
        .subcommand(
            Command::new("wc")
                .about("Print word count metrics for the project, stripping out metadata, citations, and footnotes.")
                .arg(arg!(--full "Show full pre-stripped word count of each file as well."))
        )
        .subcommand(
            Command::new("fmt")
                .about("Run an automated formatter on all the local Markdown files.")
                .arg(arg!(--"no-wrap" "Do not add linebreaks to wrap the Markdown text."))
                .arg(arg!(--columns <NUM> "The number of characters that can be in each line before wrapping.")
                    .value_parser(value_parser!(u32)).default_value("80"))
        )
}

fn _main() -> Result<()> {
    let matches = cli().get_matches();

    let pandoc_features = [
        "+bracketed_spans",  // let us put attributes on individual spans
        "+raw_tex",          // allow raw TeX commands (like `\noindent{}`)
        "-auto_identifiers", // don't try to link section headings
    ];

    config::CONFIG.set(config::Configuration {
        verbose: matches.get_flag("verbose"),
        pandoc_input_format: format!("markdown{}", pandoc_features.join("")),
        output_directory_name: "output".to_string(),
    });

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
        Some(("dev", _)) => {
            project_setup::dev()?;
        }
        Some(("build", sub_matches)) => {
            let output_format: formats::OutputFormat = match sub_matches
                .get_one::<String>("output-format")
                .expect("required")
                .as_str()
            {
                // TODO: can this be auto-mapped or derived via clap somehow?
                "docx" => formats::OutputFormat::Docx,
                "docx+pdf" => formats::OutputFormat::DocxPdf,
                "latex" => formats::OutputFormat::LaTeX,
                "latex+pdf" => formats::OutputFormat::LaTeXPdf,
                "json" => formats::OutputFormat::Json,
                _ => bail!("Invalid output format."),
            };

            build::build(
                output_format,
                *sub_matches
                    .get_one::<i64>("docx-revision")
                    .expect("required"),
            )?;
        }
        Some(("save", _)) => {
            save::save()?;
        }
        Some(("push", _)) => {
            save::push()?;
        }
        Some(("web", _)) => {
            save::web()?;
        }
        Some(("wc", sub_matches)) => {
            wc::wc(sub_matches.get_flag("full"))?;
        }
        Some(("fmt", sub_matches)) => {
            fmt::fmt(
                !sub_matches.get_flag("no-wrap"),
                *sub_matches.get_one::<u32>("columns").expect("required"),
            )?;
        }
        _ => unreachable!(),
    }

    Ok(())
}

fn main() {
    if let Err(e) = _main() {
        // eprintln!("\x1B[0;31m\x1b[1merror\x1b[0m: {:?}", e);
        //// more portable version of the above, using the already-included termcolor:
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
