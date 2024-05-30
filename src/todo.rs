use std::fs::File;
use std::io::{Error, Read, Write};
use std::path::Path;

#[derive(Debug)]
pub struct TodoItem {
    pub title: String,
    pub status: bool,
    pub priority: char,
    pub order: u16,
    pub creation_date: String,
    pub due_date: String,
    pub tags: Vec<String>,
    pub description: String,
}

impl TodoItem {
    /// load the data from the file path into a todo item
    pub fn load(file_path: &Path) -> Result<TodoItem, Error> {
        let mut file = File::open(file_path).unwrap();
        let mut buffer = String::new();

        file.read_to_string(&mut buffer).unwrap();

        let mut title = String::new();
        let mut status = false;
        let mut priority = String::new();
        let mut order = 0;
        let mut creation_date = String::new();
        let mut due_date = String::new();
        let mut tags = Vec::new();
        let mut description = String::new();

        let mut end = false;

        buffer.lines().for_each(|line| {
            if end {
                description.push_str(line);
                description.push_str("\n");
                return;
            }

            if line.is_empty() {
                end = true;
                return;
            }

            let mut parts = line.split(": ");
            let key = parts.next().unwrap();
            let value = parts.next().unwrap();

            match key {
                "Title" => title = value.to_string(),
                "Status" => status = value == "true",
                "Priority" => priority = value.to_string(),
                "Order" => order = value.parse().unwrap(),
                "Creation Date" => creation_date = value.to_string(),
                "Due Date" => due_date = value.to_string(),
                "Tags" => tags = value.split(", ").map(|s| s.to_string()).collect(),
                "" => (),
                _ => description = value.to_string(),
            }
        });

        let todo_item = TodoItem {
            title,
            status,
            priority: priority.chars().next().unwrap(),
            order,
            creation_date,
            due_date,
            tags,
            description,
        };

        Ok(todo_item)
    }

    /// Validate the data in the struct is the same as in the file.
    pub fn validate(&self) -> bool {
        let filename = format!("{}.md", self.title);
        let file_path = Path::new(filename.as_str());

        let data = TodoItem::load(file_path).unwrap();

        self.title == data.title
            && self.status == data.status
            && self.priority == data.priority
            && self.order == data.order
            && self.creation_date == data.creation_date
            && self.due_date == data.due_date
            && self.tags == data.tags
            && self.description == data.description
    }

    /// Save the data in the struct to the file
    pub fn save(&self) -> Result<(), Error> {
        let filename = format!("{}.md", self.title);
        let file_path = Path::new(filename.as_str());

        let mut file = File::create(file_path)?;

        writeln!(file, "Title: {}", self.title)?;
        writeln!(file, "Status: {}", self.status)?;
        writeln!(file, "Priority: {}", self.priority)?;
        writeln!(file, "Order: {}", self.order)?;
        writeln!(file, "Creation Date: {}", self.creation_date)?;
        writeln!(file, "Due Date: {}", self.due_date)?;
        writeln!(file, "Tags: {}", self.tags.join(", "))?;
        writeln!(file)?;
        writeln!(file, "Description: {}", self.description)?;

        file.flush()?;

        Ok(())
    }

    pub fn delete(&self) -> Result<(), Error> {
        let filename = format!("{}.md", self.title);
        let file_path = Path::new(filename.as_str());

        std::fs::remove_file(file_path)?;

        Ok(())
    }
}
