use tui::layout::Alignment;
use tui::style::{Color, Style};
use tui::widgets::{Block, BorderType, Borders, Paragraph};

pub fn render_settings<'a>() -> Paragraph<'a> {
    let settings = Paragraph::new("THIS IS A PLACEHOLDER SECTION for Settings")
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Settings")
                .border_type(BorderType::Plain),
        );
    
    return settings;
}