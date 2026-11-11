use std::{error::Error, fmt, fs};

use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

fn file_path() -> Result<PathBuf, Box<dyn Error>> {
    let home = dirs::home_dir().ok_or("could not find home directory")?;
    Ok(home.join(".task-cli").join("tasks.json"))
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TaskStatus {
    Todo,
    Done,
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskStatus::Todo => write!(f, "Todo"),
            TaskStatus::Done => write!(f, "Done"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub status: TaskStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Task {
    fn new(id: i32, title: String) -> Self {
        let now = Utc::now();
        Task {
            id,
            title,
            status: TaskStatus::Todo,
            created_at: now,
            updated_at: now,
        }
    }

    fn touch(&mut self) {
        self.updated_at = Utc::now();
    }
}

pub struct TaskRepository;

impl TaskRepository {
    pub fn load_tasks() -> Result<Vec<Task>, Box<dyn Error>> {
        let path = file_path()?;
        if !path.exists() {
            return Ok(Vec::new());
        }
        let data = fs::read_to_string(path)?;
        let tasks = serde_json::from_str(&data)?;
        Ok(tasks)
    }

    pub fn add_task(title: String) -> Result<Task, Box<dyn Error>> {
        let mut tasks = Self::load_tasks()?;

        let next_id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;

        let task = Task::new(next_id, title);
        tasks.push(task.clone());

        Self::save_tasks(&tasks)?;
        Ok(task)
    }

    pub fn delete_task(id: i32) -> Result<(), Box<dyn Error>> {
        let mut tasks = Self::load_tasks()?;

        tasks.retain(|task| task.id != id);

        Self::save_tasks(&tasks)?;
        Ok(())
    }

    pub fn mark_task_as_done(id: i32) -> Result<Task, Box<dyn Error>> {
        let mut tasks = Self::load_tasks()?;

        let task = tasks
            .iter_mut()
            .find(|t| t.id == id)
            .ok_or(format!("task #{} not found", id))?;

        task.status = TaskStatus::Done;
        task.touch(); // bump updated_at
        let updated = task.clone(); // copy out before the borrow ends

        Self::save_tasks(&tasks)?;
        Ok(updated)
    }

    pub fn save_tasks(tasks: &Vec<Task>) -> Result<(), Box<dyn Error>> {
        let path = file_path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(tasks)?;
        fs::write(path, json)?;
        Ok(())
    }
}
