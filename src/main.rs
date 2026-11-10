use clap::{Args, Parser, Subcommand};
use std::error::Error;

mod task_repository;
use task_repository::TaskRepository;

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Add a new task
    Add(AddArgs),
    /// List all remaining tasks
    List,
    /// Mark a task as done
    Done(DeleteAndDoneArgs),
    /// Delete all tasks
    Delete(DeleteAndDoneArgs),
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Args, Debug, Clone)]
struct AddArgs {
    /// Task title
    title: String,
}

#[derive(Args, Debug, Clone)]
struct DeleteAndDoneArgs {
    /// Task id
    id: i32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add(args) => {
            let task = TaskRepository::add_task(args.title).unwrap();
            println!("Task successfuly created: {}", task.title)
        }
        Commands::List => {
            let tasks = TaskRepository::load_tasks().unwrap();
            if tasks.is_empty() {
                println!("Currently you have no tasks you can rest or you can add a new task")
            } else {
                for task in tasks {
                    println!("{}, {} , {:?}", task.id, task.title, task.status)
                }
            }
        }
        Commands::Done(args) => {
            let task = TaskRepository::mark_task_as_done(args.id).unwrap();
            println!("Task {} completed well done!", task.title)
        }
        Commands::Delete(args) => {}
    }

    Ok(())
}
