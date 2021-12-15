use crate::app::{App, AppResult, MenuItems};
use crate::modules::database::*;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // exit application on ESC
        KeyCode::Esc => {
            app.running = false;
        }
        // exit application on Ctrl-D
        KeyCode::Char('d') | KeyCode::Char('D') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.running = false;
            }
        }
        // navigate to `todo` on T
        KeyCode::Char('t') | KeyCode::Char('T') => {
            app.active_tab = MenuItems::Todo;
        }
        //navigate to `home` on H
        KeyCode::Char('h') | KeyCode::Char('H') => {
            app.active_tab = MenuItems::Home;
        }
        //navigate to `pomodoro` on P
        KeyCode::Char('p') | KeyCode::Char('P') => {
            app.active_tab = MenuItems::Pomodoro;
        }
        //navigate to `analytics` on A
        KeyCode::Char('a') | KeyCode::Char('A') => {
            app.active_tab = MenuItems::Analytics;
        }
        //navigate to `settings` on S
        KeyCode::Char('s') | KeyCode::Char('S') => {
            app.active_tab = MenuItems::Settings;
        }
        _ => {}
    }
    Ok(())
}