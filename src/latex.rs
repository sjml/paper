use std::path::Path;

use anyhow::{bail, Result};

use crate::config::CONFIG;
use crate::formats::Builder;
use crate::metadata::PaperMeta;
use crate::subprocess;
use crate::util;

pub struct LatexBuilder {}

impl Default for LatexBuilder {
    fn default() -> Self {
        LatexBuilder {}
    }
}

impl Builder for LatexBuilder {
    fn get_output_file_suffix(&self) -> String {
        "tex".to_string()
    }

    fn prepare(&mut self, args: &mut Vec<String>, meta: &PaperMeta) -> Result<()> {
        let cmds = [
            "--to=latex".to_string(),
            "--shift-heading-level-by".to_string(),
            "-1".to_string(),
        ];
        args.extend_from_slice(&cmds);

        if meta.get_bool(&["latex", "fragment"]).unwrap_or(false) {
            if CONFIG.get().verbose {
                println!("Generating LaTeX fragment...")
            }
        } else {
            if CONFIG.get().verbose {
                println!("Generating full LaTeX file...")
            }
            args.extend_from_slice(&[
                "--template".to_string(),
                "./.paper_resources/ChicagoStyle_Template.tex".to_string(),
            ]);
        }

        args.extend_from_slice(&[
            "--variable".to_string(),
            format!("library_name={}", util::LIB_NAME),
            "--variable".to_string(),
            format!("library_version={}", util::LIB_VERSION),
        ]);

        if let Some(data) = meta.get_data_pairs(&["data"]) {
            for (k, mut v) in data {
                if k == "date" {
                    v = util::get_date_string(meta)?;
                }
                // process any markdown inside the variables (italics in a title, for instance)
                let marked_up = subprocess::run_command(
                    "pandoc",
                    &[
                        "--from".to_string(),
                        CONFIG.get().pandoc_input_format.clone(),
                        "--to".to_string(),
                        "latex".to_string(),
                    ],
                    Some(&v),
                )?;

                args.extend_from_slice(&[
                    "--variable".to_string(),
                    format!("{}={{{}}}", k, marked_up.trim()),
                ]);
            }
        } else {
            bail!("Missing data map.");
        }

        if meta.get_bool(&["latex", "ragged"]).unwrap_or(false) {
            args.extend_from_slice(&["--variable".to_string(), "ragged=true".to_string()]);
        }

        if let Some(base_font_override) = meta.get_string(&["base_font_override"]) {
            if CONFIG.get().verbose {
                println!("Changing base font to {}...", base_font_override);
            }
            args.extend_from_slice(&[
                "--variable".to_string(),
                format!("base_font_override={}", base_font_override),
            ]);
        }
        if let Some(mono_font_override) = meta.get_string(&["mono_font_override"]) {
            if CONFIG.get().verbose {
                println!("Changing mono font to {}...", mono_font_override);
            }
            args.extend_from_slice(&[
                "--variable".to_string(),
                format!("mono_font_override={}", mono_font_override),
            ]);
        }

        Ok(())
    }

    fn get_file_list(&self) -> Vec<String> {
        let mut content_files = walkdir::WalkDir::new("./content")
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().is_file())
            .map(|entry| entry.path().as_os_str().to_string_lossy().to_string())
            .collect::<Vec<String>>();
        content_files.sort();

        content_files
    }

    fn finish_file(&self, _output_file_path: &Path, _meta: &PaperMeta) -> Result<Vec<String>> {
        if CONFIG.get().verbose {
            println!("Packinging LaTeX...");
        }

        // no-op
        Ok(vec![])
    }
}
