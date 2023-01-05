use std::fs;
use std::io::Write;
use std::path::Path;

use anyhow::{Context, Result};
use tempfile::{self, NamedTempFile};
use walkdir;

use crate::config::CONFIG;
use crate::metadata::PaperMeta;
use crate::util;

#[derive(PartialEq, Debug)]
pub enum OutputFormat {
    Docx,
    DocxPdf,
    LaTeX,
    LaTeXPdf,
    Json,
}

pub trait Builder {
    fn prepare(&mut self, args: &mut Vec<String>, meta: &PaperMeta) -> Result<()>;
    fn get_file_list(&self) -> Vec<String>;
    fn get_output_file_suffix(&self) -> String;
    fn finish_file(&self, output_file_path: &Path) -> Result<Vec<String>>;
}

pub struct DocXBuilder {
    tmp_prefix_files: Vec<NamedTempFile>,
}

impl Default for DocXBuilder {
    fn default() -> Self {
        DocXBuilder {
            tmp_prefix_files: vec![],
        }
    }
}

impl Builder for DocXBuilder {
    fn get_output_file_suffix(&self) -> String {
        "docx".to_string()
    }

    fn prepare(&mut self, args: &mut Vec<String>, meta: &PaperMeta) -> Result<()> {
        #[rustfmt::skip]
        let cmds = [
            "--to=docx".to_string(),
            "--reference-doc".to_string(), "./.paper_resources/ChicagoStyle_Template.docx".to_string(),
        ];
        args.extend_from_slice(&cmds);

        if !meta.get_bool(&["no_title_page"]).unwrap_or_else(|| false) {
            let outpath = Path::new(&CONFIG.get().output_directory_name);
            let mut title_page_file = tempfile::Builder::new()
                .prefix("title-page")
                .suffix(".md")
                .tempfile_in(outpath)
                .context("Could not create temporary title page file.")?;

            if CONFIG.get().verbose {
                println!("Generating title page into {:?}...", title_page_file.path());
            }

            let mut title_string_coll: Vec<String> = vec![];

            let title = meta.get_string(&["data", "title"]);
            let subtitle = meta.get_string(&["data", "subtitle"]);

            if title.is_some() || subtitle.is_some() {
                title_string_coll.push("::: {custom-style=\"Title\"}\n".to_string());
                match title {
                    Some(title_str) => {
                        title_string_coll.push(title_str);
                        match subtitle {
                            Some(subtitle_str) => {
                                title_string_coll.push(":\\\n".to_string());
                                title_string_coll.push(subtitle_str);
                                title_string_coll.push("\n".to_string());
                            }
                            None => title_string_coll.push("\n".to_string()),
                        }
                    }
                    None => {
                        if let Some(subtitle_str) = subtitle {
                            title_string_coll.push(subtitle_str);
                            title_string_coll.push("\n".to_string());
                        }
                    }
                }
                title_string_coll.push(":::\n".to_string());
            }
            title_string_coll.push("::: {custom-style=\"Author\"}\nby\n:::\n".to_string());
            title_string_coll.push("::: {custom-style=\"Author\"}\n".to_string());
            if let Some(author_str) = meta.get_string(&["data", "author"]) {
                title_string_coll.push(author_str);
                title_string_coll.push("\n".to_string());
            }
            title_string_coll.push(":::\n".to_string());

            title_string_coll.push("::: {custom-style=\"Author\"}\n".to_string());

            meta.get_string(&["data", "professor"])
                .map(|prof_str| title_string_coll.push(format!("{}\\\n", prof_str)));

            // I wonder if there's some more elegant way of handling this, but it's a pretty awkward
            //    bit of logic no matter what
            match meta.get_string(&["data", "class_mnemonic"]) {
                Some(mnemonic_str) => {
                    title_string_coll.push(mnemonic_str);
                    if let Some(classname_str) = meta.get_string(&["data", "class_name"]) {
                        title_string_coll.push(format!(" --- {}", classname_str));
                    }
                }
                None => {
                    if let Some(classname_str) = meta.get_string(&["data", "class_name"]) {
                        title_string_coll.push(classname_str);
                    }
                }
            }

            title_string_coll.push("\\\n".to_string());
            title_string_coll.push(util::get_date_string(&meta)?);

            title_string_coll.push("\n:::\n".to_string());

            write!(title_page_file, "{}", title_string_coll.join(""))
                .context("Could not write to temporary title page file.")?;

            self.tmp_prefix_files.push(title_page_file);
        }

        Ok(())
    }

    fn get_file_list(&self) -> Vec<String> {
        let mut file_list = vec![];

        file_list.extend(
            self.tmp_prefix_files
                .iter()
                .map(|ntf| ntf.path().to_string_lossy().to_string()),
        );

        let mut content_files = walkdir::WalkDir::new("./content")
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().is_file())
            .map(|entry| entry.path().as_os_str().to_string_lossy().to_string())
            .collect::<Vec<String>>();
        content_files.sort();
        file_list.extend(content_files);

        return file_list;
    }

    fn finish_file(&self, output_file_path: &Path) -> Result<Vec<String>> {
        if CONFIG.get().verbose {
            println!("Packinging docx...");
        }

        // let docx_buffer = fs::read(output_file_path)?;


        Ok(vec![])
    }
}
