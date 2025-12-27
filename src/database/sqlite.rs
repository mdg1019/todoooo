use super::Database;
use crate::models::Task;
use sqlite::Connection;
use std::error::Error;

pub struct SqliteDatabase {}

impl SqliteDatabase {}

impl Database for SqliteDatabase {
    fn create_database(&self) -> Result<(), Box<dyn Error>> {
        let conn = Connection::open("todo.db")?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS Tasks (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                done INTEGER NOT NULL,
                created_at TEXT NOT NULL,
                due TEXT,
                tags TEXT,
                priority TEXT NOT NULL
            );",
        )?;
        Ok(())
    }

    fn add_task(&self, task: &Task) -> Result<(), String> {
        let conn = Connection::open("todo.db").map_err(|e| e.to_string())?;

        let tags = if task.tags.is_empty() {
            None
        } else {
            Some(task.tags.join(","))
        };

        let mut statement = conn
            .prepare(
                "INSERT INTO Tasks (id, title, done, created_at, due, tags, priority) VALUES (?, ?, ?, ?, ?, ?, ?)",
            )
            .map_err(|e| e.to_string())?;

                statement
                    .bind((1, &task.id as &str))
                    .map_err(|e| e.to_string())?;
                statement
                    .bind((2, &task.title as &str))
                    .map_err(|e| e.to_string())?;
                statement
                    .bind((3, task.done as i64))
                    .map_err(|e| e.to_string())?;
                statement
                    .bind((4, &task.created_at.to_rfc3339() as &str))
                    .map_err(|e| e.to_string())?;
                statement
                    .bind((
                        5,
                        &task.due.map(|d| d.to_rfc3339()).unwrap_or_default() as &str,
                    ))
                    .map_err(|e| e.to_string())?;
                statement
                    .bind((6, &tags.unwrap_or_default() as &str))
                    .map_err(|e| e.to_string())?;
                statement
                    .bind((7, &format!("{:?}", task.priority) as &str))
                    .map_err(|e| e.to_string())?;

                statement.next().map_err(|e| e.to_string())?;

        Ok(())
    }
}
