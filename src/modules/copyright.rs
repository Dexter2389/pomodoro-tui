use crate::app::Block as FBlock;

use tui::layout::Alignment;
use tui::style::{Color, Style};
use tui::widgets::{Block, BorderType, Borders, Paragraph, Wrap};

pub fn render_copyright<'a>(focus_block: &'a FBlock) -> Paragraph<'a> {
    let color = check_focus(focus_block);

    let copyright = Paragraph::new("pomodoro-tui 2021 - all rights reserved")
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(color))
                .title("Copyright")
                .border_type(BorderType::Plain),
        )
        .wrap(Wrap { trim: true });
    return copyright;
}

fn check_focus(focus_block: &FBlock) -> Color {
    if focus_block == &FBlock::CopyrightBlock {
        return Color::Yellow;
    } else {
        return Color::White;
    };
}
