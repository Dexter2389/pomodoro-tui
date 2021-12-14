use crate::app::AppResult;
// use crate::modules::utils::*;

use tui::style::{Color, Modifier, Style};
use tui::layout::{Constraint};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders, Cell, List, ListItem, ListState, Row, Table};

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use std::fs;

// Todo.
#[derive(Debug)]
pub struct Todo {
    /// List item selected of our TODO list.
    pub todo_list_selected: ListState,
}

impl Default for Todo {
    fn default() -> Self {
        // Create ListState
        let mut todo_list_selected = ListState::default();
        // Initialize ListState
        todo_list_selected.select(Some(0));
        Self {
            todo_list_selected: todo_list_selected,
        }
    }
}

impl Todo {
    /// Constructs a new instance of [`Todo`].
    pub fn new() -> Self {
        Self::default()
    }
}

pub fn render_todo<'a>(todo_list_state: &ListState) -> (List<'a>, Table<'a>) {
    let todos = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("Todo List")
        .border_type(BorderType::Plain);

    let todo_list = read_db().expect("Can fetch List from DB");
    let items: Vec<_> = todo_list
        .iter()
        .map(|todo| {
            ListItem::new(Spans::from(vec![Span::styled(
                todo.name.clone(),
                Style::default(),
            )]))
        })
        .collect();

    let selected_todo = todo_list
        .get(
            todo_list_state
                .selected()
                .expect("There is always a selected task"),
        )
        .expect("Exists")
        .clone();

    let list = List::new(items).block(todos).highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    let todo_details = Table::new(vec![Row::new(vec![
        Cell::from(Span::raw(selected_todo.id.to_string())),
        Cell::from(Span::raw(selected_todo.name)),
        Cell::from(Span::raw(selected_todo.category)),
        Cell::from(Span::raw(selected_todo.age.to_string())),
        Cell::from(Span::raw(selected_todo.created_at.to_string())),
    ])])
    .header(Row::new(vec![
        Cell::from(Span::styled(
            "ID",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Name",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Category",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Age",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Created At",
            Style::default().add_modifier(Modifier::BOLD),
        )),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Detail")
            .border_type(BorderType::Plain),
    )
    .widths(&[
        Constraint::Percentage(5),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
        Constraint::Percentage(5),
        Constraint::Percentage(20),
    ]);

    return (list, todo_details);
}

#[derive(Serialize, Deserialize, Clone)]
struct Pet {
    id: usize,
    name: String,
    category: String,
    age: usize,
    created_at: DateTime<Utc>,
}

fn read_db() -> AppResult<Vec<Pet>> {
    const DB_PATH: &str = "./data/db.json";
    let db_content = fs::read_to_string(DB_PATH)?;
    let parsed: Vec<Pet> = serde_json::from_str(&db_content)?;
    Ok(parsed)
}
