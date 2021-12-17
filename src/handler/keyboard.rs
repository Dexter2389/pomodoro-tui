use crate::app::{App, AppResult, Block};
use crate::modules::menu::*;
use crate::modules::utils::*;
use crate::modules::todo::*;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // exit application on Ctrl-D
        KeyCode::Char('d') | KeyCode::Char('D') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.running = false;
            }
        }
        // exit application on ESC if the control is with the app or else give control back to application
        KeyCode::Esc => match app.control_with_app {
            // Check if the control is with the app.
            true => {
                app.running = false;
            }
            false => match app.focus_block.sub_block_in_control {
                // Check which sub block the control is in and pass the control back to the application or other sub block based on the current state.
                SubBlock::TodoDetails => {
                    app.focus_block.sub_block_in_control = SubBlock::TodoList;
                    app.focus_block.sub_sub_block_in_control.focus_block = SubSubBlock::None;
                }
                _ => {
                    app.focus_block.control_with_block = false;
                    app.control_with_app = true;
                    app.focus_block.sub_block_in_control = SubBlock::None;
                    app.focus_block.sub_sub_block_in_control.focus_block = SubSubBlock::None;
                }
            },
        },
        // navigate to `todo` on T
        KeyCode::Char('t') | KeyCode::Char('T') => match app.control_with_app {
            // Check if the control is with the app.
            true => {
                app.active_tab = MenuItems::Todo;
                app.focus_block.focus_block = Block::MenuBlock;
            }
            _ => {}
        },
        //navigate to `home` on H
        KeyCode::Char('h') | KeyCode::Char('H') => match app.control_with_app {
            // Check if the control is with the app.
            true => {
                app.active_tab = MenuItems::Home;
            }
            _ => {}
        },
        //navigate to `pomodoro` on P
        KeyCode::Char('p') | KeyCode::Char('P') => match app.control_with_app {
            // Check if the control is with the app.
            true => {
                app.active_tab = MenuItems::Pomodoro;
            }
            _ => {}
        },
        //navigate to `analytics` on A
        KeyCode::Char('a') | KeyCode::Char('A') => match app.control_with_app {
            // Check if the control is with the app.
            true => {
                app.active_tab = MenuItems::Analytics;
            }
            _ => {}
        },
        //navigate to `settings` on S
        KeyCode::Char('s') | KeyCode::Char('S') => match app.control_with_app {
            // Check if the control is with the app.
            true => {
                app.active_tab = MenuItems::Settings;
            }
            _ => {}
        },
        //navigate to down on DOWN
        KeyCode::Down => match app.focus_block.focus_block {
            // Check which block is in focus.
            Block::AppBlock => match app.active_tab {
                // Check which tab is active.
                MenuItems::Todo => match app.focus_block.control_with_block {
                    // Check if the control is with the block.
                    true => match app.focus_block.sub_block_in_control {
                        // Check which sub block has the control.
                        SubBlock::TodoList => {
                            app.todo_list_position.next();
                        }
                        _ => app.next(),
                    },
                    _ => app.next(),
                },
                _ => app.next(),
            },
            _ => app.next(),
        },
        //navigate to up on UP
        KeyCode::Up => match app.focus_block.focus_block {
            // Check which block is in focus.
            Block::AppBlock => match app.active_tab {
                // Check which tab is active.
                MenuItems::Todo => match app.focus_block.control_with_block {
                    // Check if the control is with the block.
                    true => match app.focus_block.sub_block_in_control {
                        // Check which sub block has the control.
                        SubBlock::TodoList => {
                            app.todo_list_position.pervious();
                        }
                        _ => app.previous(),
                    },
                    _ => app.previous(),
                },
                _ => app.previous(),
            },
            _ => app.previous(),
        },
        //navigate to right on RIGHT
        KeyCode::Right => match app.focus_block.focus_block {
            // Check which block is in focus.
            Block::AppBlock => match app.active_tab {
                // Check which tab is active.
                MenuItems::Todo => match app.focus_block.control_with_block {
                    // Check if the control is with the block.
                    true => match app.focus_block.sub_block_in_control {
                        // Check which sub block has the control.
                        SubBlock::TodoDetails => {
                            todo_detail_subsubblock_next(&mut app.focus_block.sub_sub_block_in_control);
                        }
                        _ => {},
                    },
                    _ => {},
                },
                _ => {},
            },
            _ => {},
        },
        //navigate to left on LEFT
        KeyCode::Left => match app.focus_block.focus_block {
            // Check which block is in focus.
            Block::AppBlock => match app.active_tab {
                // Check which tab is active.
                MenuItems::Todo => match app.focus_block.control_with_block {
                    // Check if the control is with the block.
                    true => match app.focus_block.sub_block_in_control {
                        // Check which sub block has the control.
                        SubBlock::TodoDetails => {
                            todo_detail_subsubblock_previous(&mut app.focus_block.sub_sub_block_in_control);
                        }
                        _ => {},
                    },
                    _ => {},
                },
                _ => {},
            },
            _ => {},
        },
        // Give control to a specific block
        KeyCode::Enter => match app.focus_block.focus_block {
            // Only MenuBlock, AppBlock and CopyrightBlock can request control
            Block::MenuBlock => match app.control_with_app {
                true => {
                    app.focus_block.control_with_block = true;
                    app.control_with_app = false;
                }
                _ => {}
            },
            Block::AppBlock => match app.active_tab {
                // Check if the control is with the App or not
                MenuItems::Todo => match app.control_with_app {
                    true => {
                        app.focus_block.control_with_block = true;
                        app.control_with_app = false;
                        app.focus_block.sub_block_in_control = SubBlock::TodoList;
                    }
                    false => match app.focus_block.sub_block_in_control {
                        // Check which sub block the control is in and pass the control according to the current state to other sub blocks.
                        SubBlock::TodoList => {
                            app.focus_block.sub_block_in_control = SubBlock::TodoDetails;
                            app.focus_block.sub_sub_block_in_control.focus_block = SubSubBlock::TodoTitle;
                        }
                        _ => {}
                    },
                },
                _ => {}
            },
            Block::CopyrightBlock => match app.control_with_app {
                true => {
                    app.focus_block.control_with_block = true;
                    app.control_with_app = false;
                }
                _ => {}
            },
            _ => {}
        },
        _ => {}
    }
    Ok(())
}
