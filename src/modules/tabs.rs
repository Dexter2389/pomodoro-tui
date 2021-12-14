use crate::app::App;

use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Tabs};

fn create_menu<'a>() -> Vec<Spans<'a>> {
    let menu_titles = vec![
        "Home", "Todo", "Pomodoro", "Analytics", "Settings", "Quit",
    ];
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

pub fn render_tabs<'a>() -> Tabs<'a> {
    let menu = create_menu();

    let tabs = Tabs::new(menu)
        .select(App::default().active_tab.into())
        .block(Block::default().title("Menu").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow))
        .divider(Span::raw("|"));

    return tabs;
}