use std::path::Path;
use crate::todo::TodoItem;

#[derive(Debug)]
pub struct Project {
    pub title: String,
    pub items: Vec<TodoItem>,
}

impl Project {
    pub fn load(project_path: &Path) -> Project {
        let mut items = Vec::new();

        for entry in project_path.read_dir().unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.is_file() {
                let item = TodoItem::load(&path).unwrap();
                items.push(item);
            }
        }

        Project {
            title: project_path.file_name().unwrap().to_str().unwrap().to_string(),
            items,
        }
    }
}
