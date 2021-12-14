use tui::layout::Alignment;
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Paragraph};

pub fn render_title<'a>() -> Paragraph<'a> {
    let title = Paragraph::new("Pomodoro TUI")
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .alignment(Alignment::Center);
    
    return title;
}