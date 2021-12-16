use crate::app::Block as FBlock;
use crate::modules::database::*;
use crate::modules::utils::*;

use tui::layout::Constraint;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders, Cell, List, ListItem, ListState, Row, Table};

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

    /// Navigate to the next item in the list.
    pub fn next(&mut self) {
        if let Some(selected) = self.todo_list_selected.selected() {
            let total_todo_task = read_from_db(DB_PATH).expect("Can fetch List from DB").len();
            if selected >= total_todo_task - 1 {
                self.todo_list_selected.select(Some(0));
            } else {
                self.todo_list_selected.select(Some(selected + 1));
            }
        }
    }

    /// Navigate to the previous item in the list.
    pub fn pervious(&mut self) {
        if let Some(selected) = self.todo_list_selected.selected() {
            let total_todo_task = read_from_db(DB_PATH).expect("Can fetch List from DB").len();
            if selected > 0 {
                self.todo_list_selected.select(Some(selected - 1));
            } else {
                self.todo_list_selected.select(Some(total_todo_task - 1));
            }
        }
    }
}

pub fn render_todo_list<'a>(
    focus_block: &'a FBlock,
    sub_block_in_control: &'a SubBlock,
) -> List<'a> {
    let color = check_focus(focus_block, sub_block_in_control, "todo_list");
    let todo_list = read_from_db(DB_PATH).expect("Can fetch List from DB");

    // Need to find a better way to handle do this variable assignment.
    let mut selected_style = Style::default();
    if sub_block_in_control == &SubBlock::TodoList || sub_block_in_control == &SubBlock::TodoDetails
    {
        selected_style = Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD);
    };

    let items: Vec<_> = todo_list
        .iter()
        .map(|todo| {
            ListItem::new(Spans::from(vec![Span::styled(
                todo.title.clone(),
                Style::default(),
            )]))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(color))
                .title("Todo List")
                .border_type(BorderType::Plain),
        )
        .highlight_style(selected_style);

    return list;
}

pub fn render_todo_details<'a>(
    focus_block: &'a FBlock,
    sub_block_in_control: &'a SubBlock,
    todo_list_state: &ListState,
) -> Table<'a> {
    let color = check_focus(focus_block, sub_block_in_control, "todo_details");
    let todo_list = read_from_db(DB_PATH).expect("Can fetch List from DB");

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
            .style(Style::default().fg(color))
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
    return todo_details;
}

fn check_focus(
    focus_block: &FBlock,
    sub_block_in_control: &SubBlock,
    calling_sub_block: &str,
) -> Color {
    if focus_block == &FBlock::AppBlock && sub_block_in_control == &SubBlock::None {
        return Color::Yellow;
    } else if focus_block == &FBlock::AppBlock
        && sub_block_in_control == &SubBlock::TodoList
        && calling_sub_block == "todo_list"
    {
        return Color::Yellow;
    } else if focus_block == &FBlock::AppBlock
        && sub_block_in_control == &SubBlock::TodoDetails
        && calling_sub_block == "todo_details"
    {
        return Color::Yellow;
    } else {
        return Color::White;
    };
}
