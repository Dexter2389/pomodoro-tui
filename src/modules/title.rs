use crate::app::Block as FBlock;

use tui::layout::Alignment;
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Paragraph};

pub fn render_title<'a>(focus_block: &'a FBlock) -> Paragraph<'a> {
    let color = check_focus(focus_block);

    let title = Paragraph::new("Pomodoro TUI")
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(color).bg(Color::Black))
        .alignment(Alignment::Center);
    return title;
}

fn check_focus(focus_block: &FBlock) -> Color {
    if focus_block == &FBlock::TitleBlock {
        return Color::Yellow;
    } else {
        return Color::White;
    };
}
