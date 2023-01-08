use std::path::Path;

use anyhow::Result;

use crate::metadata::PaperMeta;

#[derive(PartialEq, Debug)]
pub enum OutputFormat {
    Docx,
    DocxPdf,
    LaTeX,
    LaTeXPdf,
    Json,
}

impl ToString for OutputFormat {
    fn to_string(&self) -> String {
        match self {
            OutputFormat::Docx => "docx".to_string(),
            OutputFormat::DocxPdf => "docx+pdf".to_string(),
            OutputFormat::LaTeX => "latex".to_string(),
            OutputFormat::LaTeXPdf => "latex+pdf".to_string(),
            OutputFormat::Json => "json".to_string(),
        }
    }
}

impl std::str::FromStr for OutputFormat {
    type Err = ();
    fn from_str(input: &str) -> Result<OutputFormat, Self::Err> {
        match input {
            "docx" => Ok(OutputFormat::Docx),
            "docx+pdf" => Ok(OutputFormat::DocxPdf),
            "latex" => Ok(OutputFormat::LaTeX),
            "latex+pdf" => Ok(OutputFormat::LaTeXPdf),
            "json" => Ok(OutputFormat::Json),
            _ => Err(()),
        }
    }
}

pub trait Builder {
    fn prepare(&mut self, args: &mut Vec<String>, meta: &PaperMeta) -> Result<()>;
    fn get_file_list(&self) -> Vec<String>;
    fn get_output_file_suffix(&self) -> String;
    fn finish_file(&self, output_file_path: &Path, meta: &PaperMeta) -> Result<Vec<String>>;
}
