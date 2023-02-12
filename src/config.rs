use state::Storage;
use std::path::PathBuf;

pub static CONFIG: Storage<Configuration> = Storage::new();

pub struct Configuration {
    // runtime
    pub verbose: bool,

    // more static/constant stuffs
    pub pandoc_input_format: String,
    pub output_directory_name: String,
    pub content_directory_name: String,
    pub resources_path: PathBuf,
}
