use crate::modules::analytics::*;
use crate::modules::copyright::*;
use crate::modules::home::*;
use crate::modules::misc::*;
use crate::modules::pomodoro::*;
use crate::modules::settings::*;
use crate::modules::tabs::*;
use crate::modules::todo::*;

use std::error;

use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::terminal::Frame;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application Menu Items
#[derive(Copy, Clone, Debug)]
pub enum MenuItems {
    Home,
    Todo,
    Pomodoro,
    Analytics,
    Settings,
}

impl From<MenuItems> for usize {
    /// Convert MenuItems into a integer
    fn from(items: MenuItems) -> usize {
        match items {
            MenuItems::Home => 0,
            MenuItems::Todo => 1,
            MenuItems::Pomodoro => 2,
            MenuItems::Analytics => 3,
            MenuItems::Settings => 4,
        }
    }
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub active_tab: MenuItems,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            active_tab: MenuItems::Home,
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

    /// Renders the user interface widgets.
    pub fn render<B: Backend>(&mut self, frame: &mut Frame<'_, B>) {
        // Creating different sections on the screen
        let size = frame.size();
        let section = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
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
        frame.render_widget(render_title(), section[0]);
        frame.render_widget(render_tabs(), section[1]);
        match self.active_tab {
            MenuItems::Home => frame.render_widget(render_home(), section[2]),
            MenuItems::Todo => {
                let todo_section = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
                    .split(section[2]);
                let (list, table) = render_todo(&Todo::default().todo_list_selected);
                frame.render_stateful_widget(
                    list,
                    todo_section[0],
                    &mut Todo::default().todo_list_selected,
                );
                frame.render_widget(table, todo_section[1]);
            }
            MenuItems::Pomodoro => frame.render_widget(render_pomodoro(), section[2]),
            MenuItems::Analytics => frame.render_widget(render_analytics(), section[2]),
            MenuItems::Settings => frame.render_widget(render_settings(), section[2]),
        }

        frame.render_widget(render_copyright(), section[3]);
    }
}
