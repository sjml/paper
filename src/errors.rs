use std::fmt;

#[derive(Debug)]
pub enum PaperError {
    DirectoryAlreadyExists,
    DirectoryNotEmpty,
    InvalidYaml,
}
impl fmt::Display for PaperError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PaperError::DirectoryAlreadyExists => write!(f, "directory already exists"),
            PaperError::DirectoryNotEmpty => write!(f, "directory not empty"),
            PaperError::InvalidYaml => write!(f, "YAML file invalid"),
        }
    }
}
