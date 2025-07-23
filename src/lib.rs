use comfy_table::{Cell, Row, Table};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Result;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Task {
    pub description: String,
    pub completed: bool,
    pub due_date: Option<String>,
}

impl Task {
    pub fn new(description: String, due_date: Option<String>) -> Self {
        Self {
            description,
            completed: false,
            due_date,
        }
    }
}

pub struct TodoList {
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    pub fn add(&mut self, description: String, due: Option<String>) {
        self.tasks.push(Task::new(description, due));
    }

    pub fn complete(&mut self, index: usize) -> bool {
        if index < self.tasks.len() {
            self.tasks[index].completed = true;
            true
        } else {
            false
        }
    }

    pub fn remove(&mut self, index: usize) -> bool {
        if index < self.tasks.len() {
            self.tasks.remove(index);
            true
        } else {
            false
        }
    }

    pub fn load(path: &str) -> Self {
        match fs::read_to_string(path) {
            Ok(contents) => {
                let tasks = serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new());
                Self { tasks }
            }
            Err(_) => Self::new(),
        }
    }

    pub fn save(&self, path: &str) -> Result<()> {
        let json = serde_json::to_string_pretty(&self.tasks)?;
        fs::write(path, json)
    }

    pub fn show(&self) {
        if self.tasks.is_empty() {
            println!("No tasks to display.");
            return;
        }

        let mut table = Table::new();
        table.set_header(vec!["Index", "Description", "Due Date", "Completed"]);

        for (i, task) in self.tasks.iter().enumerate() {
            let due_date_str = match &task.due_date {
                Some(date) => date.clone(),
                None => "None".to_string(),
            };
            let completed_str = if task.completed { "Yes" } else { "No" };
            table.add_row(Row::from(vec![
                Cell::new(i.to_string()),
                Cell::new(&task.description),
                Cell::new(due_date_str),
                Cell::new(completed_str),
            ]));
        }
        println!("{table}");
    }
}

impl Default for TodoList {
    fn default() -> Self {
        TodoList::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_add_task() {
        let mut todo_list = TodoList::new();
        let description = "Test task".to_string();
        let due_date = Some("2025-12-31".to_string());
        todo_list.add(description.clone(), due_date.clone());
        assert_eq!(todo_list.tasks.len(), 1);
        assert_eq!(todo_list.tasks[0].description, description);
        assert_eq!(todo_list.tasks[0].due_date, due_date);
        assert!(!todo_list.tasks[0].completed);
    }

    #[test]
    fn test_complete_task() {
        let mut todo_list = TodoList::new();
        todo_list.add("Test task".to_string(), None);
        todo_list.complete(0);
        assert!(todo_list.tasks[0].completed);
    }

    #[test]
    fn test_remove_task() {
        let mut todo_list = TodoList::new();
        todo_list.add("Test task".to_string(), None);
        todo_list.remove(0);
        assert!(todo_list.tasks.is_empty());
    }

    #[test]
    fn test_save_and_load_tasks() {
        let mut todo_list = TodoList::new();
        todo_list.add("Task 1".to_string(), None);
        todo_list.add("Task 2".to_string(), Some("2025-12-31".to_string()));

        let test_file = "test_tasks.json";
        todo_list.save(test_file).unwrap();

        let loaded_list = TodoList::load(test_file);
        assert_eq!(todo_list.tasks, loaded_list.tasks);

        fs::remove_file(test_file).unwrap();
    }
}
