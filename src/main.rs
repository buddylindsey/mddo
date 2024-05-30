use std::path::Path;

mod config;
mod projects;
mod todo;

use config::load_config;
use projects::Project;

fn main() {
    let config = load_config().unwrap();
    let project_path = Path::new(&config.data_location);

    if project_path.try_exists().is_err() {
        panic!("Data location does not exist");
    }

    let mut all_projects = vec![];

    for projects in project_path.read_dir().expect("Failed to read directory") {
        if let Ok(project) = projects {
            let project = Project::load(&project.path());
            all_projects.push(project);
        }
    }
}
