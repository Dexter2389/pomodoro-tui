use crate::app::Block as FBlock;

use tui::layout::Alignment;
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders, Paragraph, Wrap};

/// Home Blocks.
#[derive(Copy, Clone, Debug)]
pub enum HomeBlock {
    Main,
}

impl From<HomeBlock> for usize {
    /// Convert HomeBlock into a integer
    fn from(items: HomeBlock) -> usize {
        match items {
            HomeBlock::Main => 0,
        }
    }
}

pub fn render_home<'a>(focus_block: &'a FBlock) -> Paragraph<'a> {
    let color = check_focus(focus_block);

    let home = Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Welcome")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("to")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::styled(
            "Pomodoro TUI",
            Style::default().fg(Color::LightBlue),
        )]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("There has to be a better way to write text in an TUI. I cannot keep writting stuff in these spans.")]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(color))
            .title("Home")
            .border_type(BorderType::Plain),
    )
    .wrap(Wrap { trim: true });

    return home;
}

fn check_focus(focus_block: &FBlock) -> Color {
    if focus_block == &FBlock::AppBlock {
        return Color::Yellow;
    } else {
        return Color::White;
    };
}
