use clap::{Args, Parser, Subcommand};
use comfy_table::{Cell, Color, Table, presets::UTF8_FULL};
use console::{Style, Term, style};
use indicatif::ProgressBar;
use std::{error::Error, io::Write, thread, time::Duration};

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
    let term = Term::stdout();

    term.clear_screen()?;
    println!("Hello {}", style("Hazhir").bold().cyan());

    match cli.command {
        Commands::Add(args) => {
            let task = TaskRepository::add_task(args.title).unwrap();
            println!(
                "{} {}",
                style("Task successfully created:").green(),
                style(task.title).green().bold()
            );
        }
        Commands::List => {
            let tasks = TaskRepository::load_tasks().unwrap();
            if tasks.is_empty() {
                println!("Currently you have no tasks you can rest or you can add a new task")
            } else {
                let mut table = Table::new();
                table
                    .load_style(UTF8_FULL.with_rounded_corners())
                    .set_header(vec!["Id", "Title", "Status"]);
                for task in tasks {
                    table.add_row(vec![
                        Cell::new(task.id.to_string()),
                        Cell::new(task.title),
                        Cell::new(task.status.to_string()).fg(Color::Blue),
                    ]);
                }
                term.write_line(&table.to_string())?;
            }
        }
        Commands::Done(args) => {
            let task = TaskRepository::mark_task_as_done(args.id).unwrap();
            println!(
                "{} {} {}",
                style("Task").cyan(),
                style(&task.title).cyan().bold(),
                style("completed well done!").cyan()
            );
        }
        Commands::Delete(args) => {
            TaskRepository::delete_task(args.id).unwrap();
            println!("{}", style("Task removed successfuly").red())
        }
    }

    Ok(())
}
