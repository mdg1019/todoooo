pub mod sqlite;

use std::error::Error;
use crate::models::Task;

pub trait Database {
	fn create_database(&self) -> Result<(), Box<dyn Error>>;
    fn add_task(&self, task: &Task) -> Result<(), String>;
}
