use std::error::Error;
use std::path::Path;

use anyhow::Result;

use crate::metadata::PaperMeta;

#[derive(PartialEq, Debug)]
pub enum OutputFormat {
    Docx,
    LaTeX,
    LaTeXPdf,
    Json,
}

impl ToString for OutputFormat {
    fn to_string(&self) -> String {
        match self {
            OutputFormat::Docx => "docx".to_string(),
            OutputFormat::LaTeX => "latex".to_string(),
            OutputFormat::LaTeXPdf => "latex+pdf".to_string(),
            OutputFormat::Json => "json".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseFormatError {}

impl Error for ParseFormatError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "Invalid format identifier string"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

impl std::fmt::Display for ParseFormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Invalid format identifier string")
    }
}

impl std::str::FromStr for OutputFormat {
    type Err = ParseFormatError;
    fn from_str(input: &str) -> Result<OutputFormat, Self::Err> {
        match input {
            "docx" => Ok(OutputFormat::Docx),
            "latex" => Ok(OutputFormat::LaTeX),
            "latex+pdf" => Ok(OutputFormat::LaTeXPdf),
            "json" => Ok(OutputFormat::Json),
            _ => Err(ParseFormatError {}),
        }
    }
}

pub trait Builder {
    fn prepare(&mut self, args: &mut Vec<String>, meta: &PaperMeta) -> Result<()>;
    fn get_file_list(&self) -> Vec<String>;
    fn get_output_file_suffix(&self) -> String;
    fn finish_file(&self, output_file_path: &Path, meta: &PaperMeta) -> Result<Vec<String>>;
}
