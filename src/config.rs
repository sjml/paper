use state;

pub static CONFIG: state::Storage<Configuration> = state::Storage::new();

pub struct Configuration {
    pub verbose: bool,
}

