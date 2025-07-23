use clap::{Parser, Subcommand};
use std::path::PathBuf;
use todo::TodoList;

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A simple to-do list", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, global = true, help = "Sets a custom tasks file")]
    file: Option<PathBuf>,
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

fn main() {
    let cli = Cli::parse();

    let tasks_file = cli.file.unwrap_or_else(|| {
        std::env::var("TASKS_FILE")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from(".to_do.json"))
    });

    let mut todo_list = TodoList::load(tasks_file.to_str().unwrap());

    match cli.command {
        Commands::Add { description, due } => {
            todo_list.add(description, due);
            if let Err(e) = todo_list.save(tasks_file.to_str().unwrap()) {
                eprintln!("Failed to save tasks: {e}");
            }
        }
        Commands::List => {
            todo_list.show();
        }
        Commands::Complete { index } => {
            if todo_list.complete(index) {
                if let Err(e) = todo_list.save(tasks_file.to_str().unwrap()) {
                    eprintln!("Failed to save tasks: {e}");
                }
            } else {
                println!("No task at index {index}");
            }
        }
        Commands::Remove { index } => {
            if todo_list.remove(index) {
                if let Err(e) = todo_list.save(tasks_file.to_str().unwrap()) {
                    eprintln!("Failed to save tasks: {e}");
                }
            } else {
                println!("No task at index {index}");
            }
        }
    }
}