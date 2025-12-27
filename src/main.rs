mod cli;
mod models;
mod database;

use clap::Parser;
use chrono::{Local, DateTime};
use cli::Cli;
use models::{Task, Priority};
use uuid::Uuid;
use database::Database;
use database::sqlite::SqliteDatabase;

fn main() {
    let db: Box<dyn Database> = Box::new(SqliteDatabase{});
    db.create_database().expect("Failed to create database");
    
    let cli = Cli::parse();
    match &cli.command {
        cli::Commands::Add { title, tag, priority, due } => {
            let priority = match priority.as_ref().map(|p| p.as_str()).unwrap_or("medium").to_lowercase().as_str() {
                    "low" => Priority::Low,
                    "medium" => Priority::Medium,
                    "high" => Priority::High,
                    _ => Priority::Medium,
                };

            let tags = tag
                .as_ref()
                .map(|t| vec![t.clone()])
                .unwrap_or_else(Vec::new);

            let created_at = Local::now();
            let due = due.as_ref().and_then(|d| {
                DateTime::parse_from_rfc3339(d)
                    .ok()
                    .map(|dt| dt.with_timezone(&Local))
            });

            let task = Task {
                id: Uuid::new_v4().to_string(),
                title: title.clone(),
                done: false,
                created_at,
                due,
                tags,
                priority,
            };

            db.add_task(&task).expect("Failed to add task");

            println!("Created task: {:?}", task);
        }
        cli::Commands::List { all } => {
            if *all {
                println!("Listing all tasks:");
            } else {
                println!("Listing pending tasks:");
            }
        }
    }
}