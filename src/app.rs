use crate::modules::analytics::*;
use crate::modules::copyright::*;
use crate::modules::home::*;
use crate::modules::menu::*;
use crate::modules::pomodoro::*;
use crate::modules::settings::*;
use crate::modules::title::*;
use crate::modules::todo::*;
use crate::modules::utils::*;

use std::error;

use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::terminal::Frame;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// A list of sections in the Application.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Block {
    TitleBlock,
    MenuBlock,
    AppBlock,
    CopyrightBlock,
}

impl Block {
    /// Convert Block into a integer
    fn from(items: Block) -> usize {
        match items {
            Block::TitleBlock => 0,
            Block::MenuBlock => 1,
            Block::AppBlock => 2,
            Block::CopyrightBlock => 3,
        }
    }
    /// Convert integer into a Block
    fn from_usize(value: usize) -> Block {
        match value {
            0 => Block::TitleBlock,
            1 => Block::MenuBlock,
            2 => Block::AppBlock,
            3 => Block::CopyrightBlock,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

/// Current Focus Block in the Application
#[derive(Debug)]
pub struct Focus {
    /// Which block is on focus currently?
    pub focus_block: Block,
    /// Is the control with Application or with the Block?
    pub control_with_block: bool,
    /// Which sub-block has control?
    pub sub_block_in_control: SubBlock,
    /// Which sub-sub-block has control?
    pub sub_sub_block_in_control: SubSubBlockControl,
}

impl Default for Focus {
    fn default() -> Self {
        Self {
            focus_block: Block::MenuBlock,
            control_with_block: false,
            sub_block_in_control: SubBlock::None,
            sub_sub_block_in_control: SubSubBlockControl::new(),
        }
    }
}

impl Focus {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Which tab currently you are on.
    pub active_tab: MenuItems,
    /// What is your current index in your todo list?
    pub todo_list_position: Todo,
    /// What is the current focus block?
    pub focus_block: Focus,
    /// Is the control with Application or with the Block?
    pub control_with_app: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            active_tab: MenuItems::Home,
            todo_list_position: Todo::new(),
            focus_block: Focus::new(),
            control_with_app: true,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    pub fn next(&mut self) {
        let control = self.control_with_app;
        if control == true {
            let current_focus = Block::from(self.focus_block.focus_block);
            let total_focus_blocks = 4;
            if current_focus >= total_focus_blocks - 1 {
                self.focus_block.focus_block = Block::from_usize(1);
            } else {
                self.focus_block.focus_block = Block::from_usize(current_focus + 1);
            }
        }
    }

    pub fn previous(&mut self) {
        let control = self.control_with_app;
        if control == true {
            let current_focus = Block::from(self.focus_block.focus_block);
            let total_focus_blocks = 4;
            if current_focus > 1 {
                self.focus_block.focus_block = Block::from_usize(current_focus - 1);
            } else {
                self.focus_block.focus_block = Block::from_usize(total_focus_blocks - 1);
            }
        }
    }

    /// Renders the user interface widgets.
    pub fn render<B: Backend>(&mut self, frame: &mut Frame<'_, B>) {
        // Creating different sections on the screen
        let size = frame.size();
        let section = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Min(2),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(size);

        // Assembling complete UI
        frame.render_widget(render_title(&self.focus_block.focus_block), section[0]);
        frame.render_widget(
            render_menu(&self.focus_block.focus_block, self.active_tab),
            section[1],
        );
        match self.active_tab {
            MenuItems::Home => {
                frame.render_widget(render_home(&self.focus_block.focus_block), section[2])
            }
            MenuItems::Todo => {
                let sub_section = create_todo_sub_sections(section[2]);
                frame.render_widget(
                    render_todo_list(
                        &self.focus_block.focus_block,
                        &self.focus_block.sub_block_in_control,
                    ),
                    sub_section[0],
                );
                frame.render_widget(
                    render_todo_search(
                        &self.focus_block.focus_block,
                        &self.focus_block.sub_block_in_control,
                        &self.focus_block.sub_sub_block_in_control.focus_block,
                    ),
                    sub_section[1],
                );
                frame.render_stateful_widget(
                    render_todo_list_area(
                        &self.focus_block.focus_block,
                        &self.focus_block.sub_block_in_control,
                        &self.focus_block.sub_sub_block_in_control.focus_block,
                    ),
                    sub_section[2],
                    &mut self.todo_list_position.todo_list_selected,
                );
                frame.render_widget(
                    render_todo_add_task(
                        &self.focus_block.focus_block,
                        &self.focus_block.sub_block_in_control,
                        &self.focus_block.sub_sub_block_in_control.focus_block,
                    ),
                    sub_section[3],
                );
                frame.render_widget(
                    render_todo_remove_task(
                        &self.focus_block.focus_block,
                        &self.focus_block.sub_block_in_control,
                        &self.focus_block.sub_sub_block_in_control.focus_block,
                    ),
                    sub_section[4],
                );
                frame.render_widget(
                    render_todo_details(
                        &self.focus_block.focus_block,
                        &self.focus_block.sub_block_in_control,
                    ),
                    sub_section[5],
                );
                frame.render_widget(
                    render_todo_title(
                        &self.focus_block.focus_block,
                        &self.focus_block.sub_block_in_control,
                        &self.focus_block.sub_sub_block_in_control.focus_block,
                        &self.todo_list_position.todo_list_selected,
                    ),
                    sub_section[6],
                );
                frame.render_widget(
                    render_todo_created_at(
                        &self.focus_block.focus_block,
                        &self.focus_block.sub_block_in_control,
                        &self.focus_block.sub_sub_block_in_control.focus_block,
                        &self.todo_list_position.todo_list_selected,
                    ),
                    sub_section[7],
                );
                frame.render_widget(
                    render_todo_category(
                        &self.focus_block.focus_block,
                        &self.focus_block.sub_block_in_control,
                        &self.focus_block.sub_sub_block_in_control.focus_block,
                        &self.todo_list_position.todo_list_selected,
                    ),
                    sub_section[8],
                );
                frame.render_widget(
                    render_todo_label(
                        &self.focus_block.focus_block,
                        &self.focus_block.sub_block_in_control,
                        &self.focus_block.sub_sub_block_in_control.focus_block,
                        &self.todo_list_position.todo_list_selected,
                    ),
                    sub_section[9],
                );
                frame.render_widget(
                    render_todo_progress(
                        &self.focus_block.focus_block,
                        &self.focus_block.sub_block_in_control,
                        &self.focus_block.sub_sub_block_in_control.focus_block,
                        &self.todo_list_position.todo_list_selected,
                    ),
                    sub_section[10],
                );
                frame.render_widget(
                    render_todo_description(
                        &self.focus_block.focus_block,
                        &self.focus_block.sub_block_in_control,
                        &self.focus_block.sub_sub_block_in_control.focus_block,
                        &self.todo_list_position.todo_list_selected,
                    ),
                    sub_section[11],
                );
                frame.render_widget(
                    render_todo_milestone(
                        &self.focus_block.focus_block,
                        &self.focus_block.sub_block_in_control,
                        &self.focus_block.sub_sub_block_in_control.focus_block,
                        &self.todo_list_position.todo_list_selected,
                    ),
                    sub_section[12],
                );
            }
            MenuItems::Pomodoro => {
                frame.render_widget(render_pomodoro(&self.focus_block.focus_block), section[2])
            }
            MenuItems::Analytics => {
                frame.render_widget(render_analytics(&self.focus_block.focus_block), section[2])
            }
            MenuItems::Settings => {
                frame.render_widget(render_settings(&self.focus_block.focus_block), section[2])
            }
        }

        frame.render_widget(render_copyright(&self.focus_block.focus_block), section[3]);
    }
}
