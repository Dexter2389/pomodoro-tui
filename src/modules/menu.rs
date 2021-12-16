use crate::app::Block as FBlock;

use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Tabs};

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

fn create_menu<'a>() -> Vec<Spans<'a>> {
    let menu_titles = vec!["Home", "Todo", "Pomodoro", "Analytics", "Settings", "Quit"];
    // Create a list of formated strings from the menu_titles
    let menu = menu_titles
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(
                    first,
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::UNDERLINED),
                ),
                Span::styled(rest, Style::default().fg(Color::White)),
            ])
        })
        .collect();

    return menu;
}

pub fn render_menu<'a>(focus_block: &'a FBlock, active_tab: MenuItems) -> Tabs<'a> {
    let color = check_focus(focus_block);

    let menu = Tabs::new(create_menu())
        .select(active_tab.into())
        .block(Block::default().title("Menu").borders(Borders::ALL))
        .style(Style::default().fg(color))
        .highlight_style(Style::default().fg(Color::Yellow))
        .divider(Span::raw("|"));

    return menu;
}

fn check_focus(focus_block: &FBlock) -> Color {
    if focus_block == &FBlock::MenuBlock {
        return Color::Yellow;
    } else {
        return Color::White;
    };
}
