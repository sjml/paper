use std::path::Path;

use anyhow::{bail, Result};

use crate::build;
use crate::config::CONFIG;
use crate::formats::Builder;
use crate::metadata::PaperMeta;
use crate::subprocess;
use crate::subprocess::RunCommandError;
use crate::util;

#[derive(Default)]
pub struct LatexBuilder {}

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
                    false,
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
        build::get_content_file_list()
    }

    fn finish_file(&self, _output_file_path: &Path, _meta: &PaperMeta) -> Result<Vec<String>> {
        if CONFIG.get().verbose {
            println!("Packinging LaTeX...");
        }

        // no-op
        Ok(vec![])
    }
}

#[derive(Default)]
pub struct LatexPdfBuilder {
    delegate: LatexBuilder,
}

impl Builder for LatexPdfBuilder {
    fn get_output_file_suffix(&self) -> String {
        self.delegate.get_output_file_suffix()
    }

    fn prepare(&mut self, args: &mut Vec<String>, meta: &PaperMeta) -> Result<()> {
        self.delegate.prepare(args, meta)
    }

    fn get_file_list(&self) -> Vec<String> {
        self.delegate.get_file_list()
    }

    fn finish_file(&self, output_file_path: &Path, _meta: &PaperMeta) -> Result<Vec<String>> {
        let args = &["--color", "never", &output_file_path.to_string_lossy()];
        if CONFIG.get().verbose {
            println!("Running LaTeX build command:");
            println!("\t{}", args.join(" "));
        }

        let output = subprocess::run_command("tectonic", args, None, true);
        match output {
            Ok(stdout) => Ok(stdout.split('\n').map(|s| s.to_string()).collect()),
            Err(e) => match e {
                RunCommandError::IoErr(ioe) => Err(ioe.into()),
                RunCommandError::RuntimeErr(out) => {
                    let stderr = String::from_utf8(out.stderr)?;
                    let tex_err: Vec<&str> = stderr
                        .split('\n')
                        .filter(|s| s.starts_with("error: "))
                        .collect();
                    bail!("TeX runtime errors: \n{}", tex_err.join("\n"));
                }
            },
        }
    }
}
