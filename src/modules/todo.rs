use crate::modules::database::*;

use tui::layout::Constraint;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders, Cell, List, ListItem, ListState, Row, Table};

const DB_PATH: &str = "./data/db.json";

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

    let todo_list = read_from_db(DB_PATH).expect("Can fetch List from DB");
    let items: Vec<_> = todo_list
        .iter()
        .map(|todo| {
            ListItem::new(Spans::from(vec![Span::styled(
                todo.title.clone(),
                Style::default(),
            )]))
        })
        .collect();

    let list = List::new(items).block(todos).highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    let selected_todo = todo_list
        .get(
            todo_list_state
                .selected()
                .expect("There is always a selected task"),
        )
        .expect("Exists")
        .clone();

    let todo_details = Table::new(vec![Row::new(vec![
        Cell::from(Span::raw(selected_todo.id.to_string())),
        Cell::from(Span::raw(selected_todo.title)),
        Cell::from(Span::raw(selected_todo.category)),
        Cell::from(Span::raw(selected_todo.description)),
        Cell::from(Span::raw(selected_todo.created_at.to_string())),
    ])])
    .header(Row::new(vec![
        Cell::from(Span::styled(
            "ID",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Title",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Category",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Description",
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
        Constraint::Percentage(10),
        Constraint::Percentage(45),
        Constraint::Percentage(10),
    ]);

    return (list, todo_details);
}
