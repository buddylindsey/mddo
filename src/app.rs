use crate::config::{Config,load_config};
use crate::projects::{Project, initialize_projects};

pub struct App {
    pub projects: Vec<Project>,
    pub config: Config,
}

impl Default for App {
    fn default() -> Self {
        let config = load_config().unwrap();
        let projects = initialize_projects(&config.data_location).unwrap();

        App {
            projects,
            config
        }
    }
}
