use tui::layout::Alignment;
use tui::style::{Color, Style};
use tui::widgets::{Block, BorderType, Borders, Paragraph, Wrap};

pub fn render_pomodoro<'a>() -> Paragraph<'a> {
    let pomodoro = Paragraph::new("THIS IS A PLACEHOLDER SECTION for Pomodoro")
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Pomodoro")
                .border_type(BorderType::Plain),
        )
        .wrap(Wrap { trim: true });
    
    return pomodoro;
}