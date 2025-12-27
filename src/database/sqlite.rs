use super::Database;
use crate::models::Task;
use sqlite::Connection;

#[macro_export]
macro_rules! statement_bind {
    ($stmt:expr, $args:expr) => {
        $stmt.bind($args).map_err(|e| e.to_string())?
    };
}

#[macro_export]
macro_rules! statement_read {
    ($stmt:expr, $idx:expr) => {
        $stmt.read($idx).map_err(|e| e.to_string())?
    };
}

pub struct SqliteDatabase {}

impl SqliteDatabase {
    fn tags_to_string(&self, tags: &[String]) -> String {
        tags.join(",")
    }

    fn string_to_tags(&self, tags: &str) -> Vec<String> {
        if tags.is_empty() {
            Vec::new()
        } else {
            tags.split(',').map(|s| s.to_string()).collect()
        }
    }

    fn parse_optional_date(&self,date: &str) -> Result<Option<chrono::DateTime<chrono::Local>>, String> {
        if date.is_empty() {
            Ok(None)
        } else {
            chrono::DateTime::parse_from_rfc3339(date)
                .map(|dt| Some(dt.with_timezone(&chrono::Local)))
                .map_err(|e| e.to_string())
        }
    }

    fn format_optional_date(&self, date: &Option<chrono::DateTime<chrono::Local>>) -> String {
        date.map(|d| d.to_rfc3339()).unwrap_or_default()
    }
}

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

        let tags = self.tags_to_string(&task.tags);
        
        let mut statement = conn
            .prepare(
                "INSERT INTO Tasks (id, title, done, created_at, due, tags, priority) VALUES (?, ?, ?, ?, ?, ?, ?)",
            )
            .map_err(|e| e.to_string())?;

        statement_bind!(statement, (1, &task.id as &str));
        statement_bind!(statement, (2, &task.title as &str));
        statement_bind!(statement, (3, task.done as i64));
        statement_bind!(statement, (4, &task.created_at.to_rfc3339() as &str));
        statement_bind!(statement, (5, &self.format_optional_date(&task.due) as &str));
        statement_bind!(statement, (6, &tags as &str));
        statement_bind!(statement, (7, &format!("{:?}", task.priority) as &str));

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
            let id: String = statement_read!(statement, 0);
            let title: String = statement_read!(statement, 1);
            let done: i64 = statement_read!(statement, 2);
            let created_at: String = statement_read!(statement, 3);
            let due: String = statement_read!(statement, 4);
            let tags: String = statement_read!(statement, 5);
            let priority: String = statement_read!(statement, 6);
            let created_at = chrono::DateTime::parse_from_rfc3339(&created_at).map_err(|e| e.to_string())?.with_timezone(&chrono::Local);
            let due = self.parse_optional_date(&due)?;
            let tags = self.string_to_tags(&tags);
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
