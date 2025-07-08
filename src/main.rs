use clap::{Parser, Subcommand};

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

fn main() {
    let mut tasks: Vec<Task> = Vec::new();  // vec to hold all tasks

    // let one_task = Task::new(String::from("Test task"), None);
    // let another_task = Task::new(
    //     String::from("Another task"),
    //     false,
    //     Some(String::from("Tomorrow")),
    // );

    // show_task(&one_task);

    // tasks.push(one_task);
    // tasks.push(another_task);

    // show_task(&tasks[0]);   // show tasks from tasks vec
    // show_task(&tasks[1]);

    let cli = Cli::parse();

    match cli.command {
        Commands::Add { description, due } => {
            tasks.push(Task::new(description, due));
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
            } else {
                println!("No task at index {index}");
            }
        }
        Commands::Remove { index } => {
            if index < tasks.len() {
                tasks.remove(index);
            } else {
                println!("No task at index {index}");
            }
            
        }
    }
}
