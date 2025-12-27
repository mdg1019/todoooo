use super::Database;
use crate::models::Task;
use sqlite::Connection;

pub struct SqliteDatabase {}

impl SqliteDatabase {}

impl Database for SqliteDatabase {
    fn create_database(&self) -> Result<(), String> {
        let conn = Connection::open("todo.db").map_err(|e| e.to_string())?;

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
        ).map_err(|e| e.to_string())?;
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
    
    fn list_tasks(&self) -> Result<Vec<Task>, String> {
        let conn = Connection::open("todo.db").map_err(|e| e.to_string())?;
        let mut statement = conn.prepare(
            "SELECT id, title, done, created_at, due, tags, priority FROM Tasks"
        ).map_err(|e| e.to_string())?;
        let mut tasks = Vec::new();
        while let sqlite::State::Row = statement.next().map_err(|e| e.to_string())? {
            let id: String = statement.read(0).map_err(|e| e.to_string())?;
            let title: String = statement.read(1).map_err(|e| e.to_string())?;
            let done: i64 = statement.read(2).map_err(|e| e.to_string())?;
            let created_at: String = statement.read(3).map_err(|e| e.to_string())?;
            let due: String = statement.read(4).map_err(|e| e.to_string())?;
            let tags: String = statement.read(5).map_err(|e| e.to_string())?;
            let priority: String = statement.read(6).map_err(|e| e.to_string())?;
            let created_at = chrono::DateTime::parse_from_rfc3339(&created_at).map_err(|e| e.to_string())?.with_timezone(&chrono::Local);
            let due = if due.is_empty() {
                None
            } else {
                Some(chrono::DateTime::parse_from_rfc3339(&due).map_err(|e| e.to_string())?.with_timezone(&chrono::Local))
            };
            let tags = if tags.is_empty() {
                Vec::new()
            } else {
                tags.split(',').map(|s| s.to_string()).collect()
            };
            let priority = match priority.as_str() {
                "Low" => crate::models::Priority::Low,
                "Medium" => crate::models::Priority::Medium,
                "High" => crate::models::Priority::High,
                _ => crate::models::Priority::Low,
            };
            tasks.push(Task {
                id,
                title,
                done: done != 0,
                created_at,
                due,
                tags,
                priority,
            });
        }
        Ok(tasks)
    }  
}
