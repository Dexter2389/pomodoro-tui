use crate::app::{App, AppResult, MenuItems};
use crossterm::event::{MouseEvent};

/// Handles the mouse events and updates the state of [`App`].
pub fn handle_mouse_events(mouse_event: MouseEvent, app: &mut App) -> AppResult<()> {
    match mouse_event.kind {
        _ => {}
    }
    Ok(())
}