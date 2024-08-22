use std::path::Path;
use std::str::FromStr;

use anyhow::Result;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

mod build;
mod cli;
mod config;
mod docx;
mod fmt;
mod formats;
mod latex;
pub mod metadata;
mod pandoc_wrap;
mod project_setup;
mod save;
mod subprocess;
mod util;
mod watcher;
mod wc;

fn _main() -> Result<()> {
    let matches = cli::cli().get_matches();

    let pandoc_features = [
        "+bracketed_spans",  // let us put attributes on individual spans
        "+raw_tex",          // allow raw TeX commands (like `\noindent{}`)
        "-auto_identifiers", // don't try to link section headings
    ];

    config::CONFIG.set(config::Configuration {
        verbose: matches.get_flag("verbose"),
        pandoc_input_format: format!("markdown{}", pandoc_features.join("")),
        output_directory_name: "output".to_string(),
        content_directory_name: "content".to_string(),
        resources_path: match option_env!("PAPER_RESOURCES_DIR") {
            Some(res_str) => Path::new(res_str).to_path_buf(),
            None => Path::new(env!("CARGO_MANIFEST_DIR")).join("resources"),
        },
    });

    if matches.get_flag("version") {
        if config::CONFIG.get().verbose {
            println!("{}", util::get_paper_version_stamp());
        } else {
            println!("{} v{}", util::LIB_NAME, util::LIB_VERSION);
        }
        std::process::exit(0);
    }

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
            let output_format = formats::OutputFormat::from_str(
                sub_matches
                    .get_one::<String>("output-format")
                    .expect("required"),
            )?;
            let of_specified = match sub_matches.value_source("output-format").expect("required") {
                clap::parser::ValueSource::DefaultValue => false,
                _ => true,
            };

            build::build(
                &output_format,
                of_specified,
                *sub_matches
                    .get_one::<i64>("docx-revision")
                    .expect("required"),
            )?;
        }
        Some(("save", sub_matches)) => {
            let msg = sub_matches.get_one::<String>("message");
            save::save(msg)?;
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
        Some(("watch", sub_matches)) => {
            let output_format = formats::OutputFormat::from_str(
                sub_matches
                    .get_one::<String>("output-format")
                    .expect("required"),
            )?;
            let of_specified = match sub_matches.value_source("output-format").expect("required") {
                clap::parser::ValueSource::DefaultValue => false,
                _ => true,
            };

            watcher::watch(
                sub_matches.get_flag("full"),
                sub_matches.get_flag("build"),
                output_format,
                of_specified,
                *sub_matches
                    .get_one::<i64>("docx-revision")
                    .expect("required"),
            )?;
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
