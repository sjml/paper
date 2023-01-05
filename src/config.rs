use state;

pub static CONFIG: state::Storage<Configuration> = state::Storage::new();

pub struct Configuration {
    // runtime
    pub verbose: bool,

    // more static/constant stuffs
    pub pandoc_input_format: String,
    pub output_directory_name: String,
}
