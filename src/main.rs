use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Result;

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A simple to-do list", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        description: String,
        #[arg(short, long)]
        due: Option<String>,
    },
    List,
    Complete {
        index: usize,
    },
    Remove {
        index: usize,
    },
}

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
struct Task {
    description: String,
    completed: bool,
    due_date: Option<String>,
}

impl Task {
    fn new(description: String, due_date: Option<String>) -> Self {
        Self {
            description,
            completed: false,
            due_date,
        }
    }
}

fn show_task(task: &Task) {
    println!("Description: {}", task.description);
    println!("Completed: {}", task.completed);
    match &task.due_date {
        Some(date) => println!("Due date: {}", date),
        None => println!("Due date: None"),
    }
    println!();
}

fn load_tasks(path: &str) -> Vec<Task> {
    match fs::read_to_string(path) {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new()),
        Err(_) => Vec::new(),
    }
}

fn save_tasks(tasks: &Vec<Task>, path: &str) -> Result<()> {
    let json = serde_json::to_string_pretty(tasks)?;
    fs::write(path, json)
}

fn main() {
    const TASKS_FILE: &str = ".to_do.json";
    let mut tasks: Vec<Task> = load_tasks(TASKS_FILE);

    let cli = Cli::parse();

    match cli.command {
        Commands::Add { description, due } => {
            tasks.push(Task::new(description, due));
            if let Err(e) = save_tasks(&tasks, TASKS_FILE) {
                eprintln!("Failed to save tasks: {e}");
                tasks.pop();    // pop task just saved
            }
        }
        Commands::List => {
            for (i, task) in tasks.iter().enumerate() {
                println!("Task {i}");
                show_task(task);
            }
            
        }
        Commands::Complete { index } => {
            if index < tasks.len() {
                tasks[index].completed = true;
                if let Err(e) = save_tasks(&tasks, TASKS_FILE) {
                    eprintln!("Failed to save tasks: {e}");
                    tasks[index].completed = false;
                }
            } else {
                println!("No task at index {index}");
            }
        }
        Commands::Remove { index } => {
            if index >= tasks.len() {
                println!("No task at index {index}");
                return;
            }

            let removed_task = tasks.remove(index);

            if let Err(e) = save_tasks(&tasks, TASKS_FILE) {
                eprintln!("Failed to save tasks: {e}");
                tasks.insert(index, removed_task);
            }
        }
    }
}
