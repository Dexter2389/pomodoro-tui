use crate::app::AppResult;

use tui::widgets::{ListState};

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use std::fs;

#[derive(Serialize, Deserialize, Clone)]
pub struct TodoDB {
    pub id: usize,
    pub title: String,
    pub label: String,
    pub category: String,
    pub description: String,
    pub milestones: Vec<Milestone>,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Milestone {
    pub subid: usize,
    pub subtitle: String,
    pub description: String,
    pub eta: usize,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

pub fn read_from_db(db_path: &str) -> AppResult<Vec<TodoDB>> {
    // Interface to read data from the database.
    let db_content = fs::read_to_string(db_path)?;
    let parsed: Vec<TodoDB> = serde_json::from_str(&db_content)?;
    Ok(parsed)
}

pub fn write_to_db(db_path: &str, data: TodoDB) -> AppResult<()> {
    // Interface to write data to the database.
    let db_content = fs::read_to_string(db_path)?;
    let mut parsed: Vec<TodoDB> = serde_json::from_str(&db_content)?;

    parsed.push(data);
    fs::write(db_path, &serde_json::to_vec(&parsed)?)?;
    Ok(())
}

pub fn delete_from_db(db_path: &str, todo_list_state: &mut ListState) -> AppResult<()> {
    // Interface to delete entry from a specific index from the database.
    if let Some(selected) = todo_list_state.selected() {
        let db_content = fs::read_to_string(db_path)?;
        let mut parsed: Vec<TodoDB> = serde_json::from_str(&db_content)?;
        parsed.remove(selected);
        fs::write(db_path, &serde_json::to_vec(&parsed)?)?;
        todo_list_state.select(Some(selected - 1));
    }
    Ok(())
}