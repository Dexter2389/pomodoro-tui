use crate::app::{Block as FBlock};

use tui::layout::Alignment;
use tui::style::{Color, Style};
use tui::widgets::{Block, BorderType, Borders, Paragraph, Wrap};

pub fn render_analytics<'a>(focus_block: &'a FBlock) -> Paragraph<'a> {
    let color = check_focus(focus_block);

    let analytics = Paragraph::new("THIS IS A PLACEHOLDER SECTION for Analytics")
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(color))
                .title("Analytics")
                .border_type(BorderType::Plain),
        )
        .wrap(Wrap { trim: true });
    
    return analytics;
}

fn check_focus(focus_block: &FBlock) -> Color {
    if focus_block == &FBlock::AppBlock {
        return Color::Yellow;
    } else {
        return Color::White;
    };
}