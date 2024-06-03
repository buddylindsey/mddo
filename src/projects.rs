use crate::todo::TodoItem;
use ratatui::{
    backend::Backend,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    terminal::Terminal,
    widgets::{Block, Paragraph, Widget},
};
use std::io::Result;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Project {
    pub title: String,
    pub items: Vec<TodoItem>,
    pub path: PathBuf,
}

pub fn initialize_projects(project_path: &Path) -> Result<Vec<Project>> {
    let mut all_projects = vec![];

    for projects in project_path.read_dir()? {
        if let Ok(project) = projects {
            let project = Project::load(&project.path())?;
            all_projects.push(project);
        }
    }

    Ok(all_projects)
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
            path: project_path.to_path_buf(),
        })
    }

    pub fn add_item(&mut self, mut item: TodoItem) -> Result<()> {
        let mut filename = self.path.clone();
        filename.push(format!("{}.md", item.title));

        item.file_path = filename;
        item.save()?;

        self.items.push(item);
        Ok(())
    }
}

impl Widget for Project {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([Constraint::Min(1)]);
        let [body] = vertical.areas(area);

        let project_block = Block::bordered().title(self.title.as_str());
        let project_text = Paragraph::new(
            self.items
                .iter()
                .map(|item| item.title.as_str())
                .collect::<Vec<&str>>()
                .join("\n"),
        )
        .block(project_block);
        project_text.render(body, buf);
    }
}
