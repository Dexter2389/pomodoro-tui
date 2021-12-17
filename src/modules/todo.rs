use crate::app::Block as FBlock;
use crate::modules::database::*;
use crate::modules::utils::*;

use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{
    Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table, Wrap,
};

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
    let color = check_focus_subblock(focus_block, sub_block_in_control, "list");
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
) -> Block<'a> {
    let color = check_focus_subblock(focus_block, sub_block_in_control, "details");
    let details = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(color))
        .title("Details")
        .border_type(BorderType::Plain);
    return details;
}

pub fn create_todo_sub_sections(area: Rect) -> Vec<Rect> {
    let main_section = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(area);

    let sub_section_1 = Layout::default()
    .direction(Direction::Vertical)
    .margin(1)
    .constraints(
        [
            Constraint::Length(3),
            Constraint::Min(3),
            Constraint::Length(5),
        ]
        .as_ref(),
    )
    .split(main_section[0]);

    let first_sub_sub_section_1 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(sub_section_1[2]);

    let sub_section_2 = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(3),
                Constraint::Min(5),
            ]
            .as_ref(),
        )
        .split(main_section[1]);

    let first_sub_sub_section_2 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(sub_section_2[0]);

    let second_sub_sub_section_2 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(34),
                Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(sub_section_2[1]);

    let list_block = main_section[0];
    let search_area = sub_section_1[0];
    let list_area = sub_section_1[1];
    let add_task_area = first_sub_sub_section_1[0];
    let delete_task_area = first_sub_sub_section_1[1];
    let detail_block = main_section[1];
    let title_area = first_sub_sub_section_2[0];
    let created_at_area = first_sub_sub_section_2[1];
    let category_area = second_sub_sub_section_2[0];
    let label_area = second_sub_sub_section_2[1];
    let progress_area = second_sub_sub_section_2[2];
    let description_area = sub_section_2[2];
    let milestone_area = sub_section_2[3];

    return vec![
        list_block,
        search_area,
        list_area,
        add_task_area,
        delete_task_area,
        detail_block,
        title_area,
        created_at_area,
        category_area,
        label_area,
        progress_area,
        description_area,
        milestone_area,
    ];
}

pub fn render_todo_title<'a>(
    focus_block: &'a FBlock,
    sub_block_in_control: &'a SubBlock,
    sub_sub_block_in_control: &'a SubSubBlock,
    todo_list_state: &ListState,
) -> Paragraph<'a> {
    let color = check_focus_subsubblock(
        focus_block,
        sub_block_in_control,
        sub_sub_block_in_control,
        "title",
    );
    let selected_todo = get_selected_item(todo_list_state);
    let title = Paragraph::new(selected_todo.title)
        .style(Style::default())
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(color))
                .title("Title")
                .border_type(BorderType::Rounded),
        )
        .wrap(Wrap { trim: true });
    return title;
}

pub fn render_todo_created_at<'a>(
    focus_block: &'a FBlock,
    sub_block_in_control: &'a SubBlock,
    sub_sub_block_in_control: &'a SubSubBlock,
    todo_list_state: &ListState,
) -> Paragraph<'a> {
    let color = check_focus_subsubblock(
        focus_block,
        sub_block_in_control,
        sub_sub_block_in_control,
        "create_at",
    );
    let selected_todo = get_selected_item(todo_list_state);
    let created_at = Paragraph::new(selected_todo.created_at.to_string())
        .style(Style::default())
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(color))
                .title("Created At")
                .border_type(BorderType::Rounded),
        )
        .wrap(Wrap { trim: true });
    return created_at;
}

pub fn render_todo_category<'a>(
    focus_block: &'a FBlock,
    sub_block_in_control: &'a SubBlock,
    sub_sub_block_in_control: &'a SubSubBlock,
    todo_list_state: &ListState,
) -> Paragraph<'a> {
    let color = check_focus_subsubblock(
        focus_block,
        sub_block_in_control,
        sub_sub_block_in_control,
        "category",
    );
    let selected_todo = get_selected_item(todo_list_state);
    let category = Paragraph::new(selected_todo.category)
        .style(Style::default())
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(color))
                .title("Category")
                .border_type(BorderType::Rounded),
        )
        .wrap(Wrap { trim: true });
    return category;
}

pub fn render_todo_label<'a>(
    focus_block: &'a FBlock,
    sub_block_in_control: &'a SubBlock,
    sub_sub_block_in_control: &'a SubSubBlock,
    todo_list_state: &ListState,
) -> Paragraph<'a> {
    let color = check_focus_subsubblock(
        focus_block,
        sub_block_in_control,
        sub_sub_block_in_control,
        "label",
    );
    let selected_todo = get_selected_item(todo_list_state);
    let label = Paragraph::new(selected_todo.label)
        .style(Style::default())
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(color))
                .title("Label")
                .border_type(BorderType::Rounded),
        )
        .wrap(Wrap { trim: true });
    return label;
}

pub fn render_todo_progress<'a>(
    focus_block: &'a FBlock,
    sub_block_in_control: &'a SubBlock,
    sub_sub_block_in_control: &'a SubSubBlock,
    todo_list_state: &ListState,
) -> Paragraph<'a> {
    let color = check_focus_subsubblock(
        focus_block,
        sub_block_in_control,
        sub_sub_block_in_control,
        "progress",
    );
    let selected_todo = get_selected_item(todo_list_state);
    let progress = Paragraph::new(selected_todo.status)
        .style(Style::default())
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(color))
                .title("Progress")
                .border_type(BorderType::Rounded),
        )
        .wrap(Wrap { trim: true });
    return progress;
}

pub fn render_todo_description<'a>(
    focus_block: &'a FBlock,
    sub_block_in_control: &'a SubBlock,
    sub_sub_block_in_control: &'a SubSubBlock,
    todo_list_state: &ListState,
) -> Paragraph<'a> {
    let color = check_focus_subsubblock(
        focus_block,
        sub_block_in_control,
        sub_sub_block_in_control,
        "description",
    );
    let selected_todo = get_selected_item(todo_list_state);
    let description = Paragraph::new(selected_todo.description)
        .style(Style::default())
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(color))
                .title("Description")
                .border_type(BorderType::Rounded),
        )
        .wrap(Wrap { trim: true });
    return description;
}

pub fn render_todo_milestone<'a>(
    focus_block: &'a FBlock,
    sub_block_in_control: &'a SubBlock,
    sub_sub_block_in_control: &'a SubSubBlock,
    todo_list_state: &ListState,
) -> Table<'a> {
    let color = check_focus_subsubblock(
        focus_block,
        sub_block_in_control,
        sub_sub_block_in_control,
        "milestones",
    );
    let selected_todo = get_selected_item(todo_list_state);

    let milestone: Vec<Row> = selected_todo
        .milestones
        .iter()
        .enumerate()
        .map(|(i, _)| {
            Row::new(vec![
                Cell::from(Span::raw(selected_todo.milestones[i].subid.to_string())),
                Cell::from(Span::raw(selected_todo.milestones[i].subtitle.to_string())),
                Cell::from(Span::raw(selected_todo.milestones[i].eta.to_string())),
                Cell::from(Span::raw(
                    selected_todo.milestones[i].created_at.to_string(),
                )),
                Cell::from(Span::raw(selected_todo.milestones[i].status.to_string())),
            ])
        })
        .collect();

    let milestones = Table::new(milestone)
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
                "ETA (in min)",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Cell::from(Span::styled(
                "Created At",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Cell::from(Span::styled(
                "Status",
                Style::default().add_modifier(Modifier::BOLD),
            )),
        ]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(color))
                .title("Milestones")
                .border_type(BorderType::Plain),
        )
        .widths(&[
            Constraint::Percentage(10),
            Constraint::Percentage(35),
            Constraint::Percentage(15),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ]);

    return milestones;
}

/// Return the currently seleted item from the todo_list
fn get_selected_item(todo_list_state: &ListState) -> TodoDB {
    let todo_list = read_from_db(DB_PATH).expect("Can fetch List from DB");

    let selected_todo = todo_list
        .get(
            todo_list_state
                .selected()
                .expect("There is always a selected task"),
        )
        .expect("Exists")
        .clone();

    return selected_todo;
}

pub fn todo_detail_subsubblock_next(current_focus_subsubblock: &mut SubSubBlockControl) {
    let current_focus: usize = SubSubBlock::from(current_focus_subsubblock.focus_block);
    let total_focus_blocks = 7;

    if current_focus >= total_focus_blocks {
        current_focus_subsubblock.focus_block = SubSubBlock::from_usize(1);
    } else {
        current_focus_subsubblock.focus_block = SubSubBlock::from_usize(current_focus + 1);
    }
}

pub fn todo_detail_subsubblock_previous(current_focus_subsubblock: &mut SubSubBlockControl) {
    let current_focus: usize = SubSubBlock::from(current_focus_subsubblock.focus_block);
    let total_focus_blocks = 7;

    if current_focus > 1 && current_focus <= total_focus_blocks {
        current_focus_subsubblock.focus_block = SubSubBlock::from_usize(current_focus - 1);
    } else {
        current_focus_subsubblock.focus_block = SubSubBlock::from_usize(total_focus_blocks);
    }
}

fn check_focus_subsubblock(
    focus_block: &FBlock,
    sub_block_in_control: &SubBlock,
    sub_sub_block_in_control: &SubSubBlock,
    calling_sub_block: &str,
) -> Color {
    if focus_block == &FBlock::AppBlock
        && sub_block_in_control == &SubBlock::None
        && sub_sub_block_in_control == &SubSubBlock::None
    {
        return Color::Yellow;
    } else if focus_block != &FBlock::AppBlock
        && sub_block_in_control == &SubBlock::None
        && sub_sub_block_in_control == &SubSubBlock::None
    {
        return Color::White;
    } else if focus_block == &FBlock::AppBlock
        && sub_block_in_control == &SubBlock::TodoList
        && sub_sub_block_in_control == &SubSubBlock::None
    {
        return Color::White;
    } else if focus_block == &FBlock::AppBlock
        && sub_block_in_control == &SubBlock::TodoDetails
        && sub_sub_block_in_control == &SubSubBlock::TodoTitle
        && calling_sub_block == "title"
    {
        return Color::White;
    } else if focus_block == &FBlock::AppBlock
        && sub_block_in_control == &SubBlock::TodoDetails
        && sub_sub_block_in_control == &SubSubBlock::TodoCreatedAt
        && calling_sub_block == "create_at"
    {
        return Color::White;
    } else if focus_block == &FBlock::AppBlock
        && sub_block_in_control == &SubBlock::TodoDetails
        && sub_sub_block_in_control == &SubSubBlock::TodoCategory
        && calling_sub_block == "category"
    {
        return Color::White;
    } else if focus_block == &FBlock::AppBlock
        && sub_block_in_control == &SubBlock::TodoDetails
        && sub_sub_block_in_control == &SubSubBlock::TodoLabel
        && calling_sub_block == "label"
    {
        return Color::White;
    } else if focus_block == &FBlock::AppBlock
        && sub_block_in_control == &SubBlock::TodoDetails
        && sub_sub_block_in_control == &SubSubBlock::TodoStatus
        && calling_sub_block == "progress"
    {
        return Color::White;
    } else if focus_block == &FBlock::AppBlock
        && sub_block_in_control == &SubBlock::TodoDetails
        && sub_sub_block_in_control == &SubSubBlock::TodoDescription
        && calling_sub_block == "description"
    {
        return Color::White;
    } else if focus_block == &FBlock::AppBlock
        && sub_block_in_control == &SubBlock::TodoDetails
        && sub_sub_block_in_control == &SubSubBlock::TodoMilestones
        && calling_sub_block == "milestones"
    {
        return Color::White;
    } else {
        return Color::Yellow;
    }
}

fn check_focus_subblock(
    focus_block: &FBlock,
    sub_block_in_control: &SubBlock,
    calling_sub_block: &str,
) -> Color {
    if focus_block == &FBlock::AppBlock && sub_block_in_control == &SubBlock::None {
        return Color::Yellow;
    } else if focus_block == &FBlock::AppBlock
        && sub_block_in_control == &SubBlock::TodoList
        && calling_sub_block == "list"
    {
        return Color::Yellow;
    } else if focus_block == &FBlock::AppBlock
        && sub_block_in_control == &SubBlock::TodoDetails
        && calling_sub_block == "details"
    {
        return Color::Yellow;
    } else {
        return Color::White;
    };
}
