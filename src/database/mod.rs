pub mod sqlite;

use crate::models::Task;

pub trait Database {
	fn create_database(&self) -> Result<(), String>;
    fn add_task(&self, task: &Task) -> Result<(), String>;
    fn list_tasks(&self) -> Result<Vec<Task>, String>;
}
