use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use final_project::store::Store;
use final_project::task::{Priority, Status};

#[derive(Debug, Parser)]
#[command(
    name = "task-manager",
    version,
    about = "A simple SQLite-backed CLI task manager"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Add a new task.
    Add {
        /// Title of the task.
        title: String,
        /// Priority of the task.
        #[arg(long, default_value = "medium")]
        priority: Priority,
    },
    /// List tasks, optionally filtered.
    List {
        /// Only show tasks with this status.
        #[arg(long)]
        status: Option<Status>,
        /// Only show tasks with this priority.
        #[arg(long)]
        priority: Option<Priority>,
    },
    /// Mark a task as done.
    Done {
        /// Id of the task to complete.
        id: i64,
    },
    /// Delete a task.
    Remove {
        /// Id of the task to delete.
        id: i64,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let store = Store::open("tasks.db").context("failed to open tasks.db")?;

    match cli.command {
        Commands::Add { title, priority } => {
            let task = store
                .add_task(&title, priority)
                .context("failed to add task")?;
            println!("Added task {}: {} [{}]", task.id, task.title, task.priority);
        }
        Commands::List { status, priority } => {
            let tasks = store
                .list_tasks(status, priority)
                .context("failed to list tasks")?;
            if tasks.is_empty() {
                println!("No tasks found.");
            } else {
                for task in tasks {
                    println!("{task}");
                }
            }
        }
        Commands::Done { id } => {
            store.mark_done(id)?;
            println!("Marked task {id} as done.");
        }
        Commands::Remove { id } => {
            store.remove_task(id)?;
            println!("Removed task {id}.");
        }
    }

    Ok(())
}
