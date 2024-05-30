use crate::todo::TodoItem;
use std::io::Result;
use std::path::Path;

#[derive(Debug)]
pub struct Project {
    pub title: String,
    pub items: Vec<TodoItem>,
    pub path: String,
}

impl Project {
    pub fn load(project_path: &Path) -> Result<Project> {
        let mut items = Vec::new();

        for entry in project_path.read_dir()? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Ok(item) = TodoItem::load(&path) {
                    items.push(item);
                }
            }
        }

        Ok(Project {
            title: project_path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("Unknown Project")
                .to_string(),
            items,
            path: project_path.to_str().unwrap().to_string(),
        })
    }

    pub fn add_item(&mut self, mut item: TodoItem) -> Result<()> {
        item.file_path = format!("{}/{}.md", self.path, item.title);
        item.save()?;
        self.items.push(item);
        Ok(())
    }
}
