use std::path::Path;

use anyhow::Result;
use chrono::prelude::*;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};

use crate::build;
use crate::config::CONFIG;
use crate::formats;
use crate::util;
use crate::wc;

fn respond_to_event(
    show_full: bool,
    should_build: bool,
    output_format: &formats::OutputFormat,
    of_specified: bool,
    docx_revision: i64,
) -> Result<()> {
    let now = Local::now();
    let now_str = now.format("%Y-%m-%d %I:%M:%S %p").to_string();
    println!("{}", now_str);

    if should_build {
        build::build(output_format, of_specified, docx_revision)?;
    }

    wc::wc(show_full)
}

pub fn watch(
    show_full: bool,
    should_build: bool,
    output_format: formats::OutputFormat,
    of_specified: bool,
    docx_revision: i64,
) -> Result<()> {
    util::ensure_paper_dir()?;

    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    let content_path = Path::new(&CONFIG.get().content_directory_name);
    let config_path_buf = util::find_meta(None)?;
    let config_path = config_path_buf.as_path();
    watcher.watch(content_path, RecursiveMode::Recursive)?;
    watcher.watch(config_path, RecursiveMode::NonRecursive)?;

    println!("Watching `{}` directory...", content_path.display());
    println!("(Press Ctrl-C to exit.)");
    respond_to_event(
        show_full,
        should_build,
        &output_format,
        of_specified,
        docx_revision,
    )?;

    for res in rx {
        match res {
            Err(e) => {
                return Err(e.into());
            }
            Ok(event) => match event.kind {
                notify::EventKind::Create(_) => {
                    respond_to_event(
                        show_full,
                        should_build,
                        &output_format,
                        of_specified,
                        docx_revision,
                    )?;
                }
                notify::EventKind::Modify(content) => match content {
                    notify::event::ModifyKind::Metadata(_) => {}
                    _ => {
                        respond_to_event(
                            show_full,
                            should_build,
                            &output_format,
                            of_specified,
                            docx_revision,
                        )?;
                    }
                },
                notify::EventKind::Remove(_) => {
                    respond_to_event(
                        show_full,
                        should_build,
                        &output_format,
                        of_specified,
                        docx_revision,
                    )?;
                }
                _ => {}
            },
        }
    }

    Ok(())
}
