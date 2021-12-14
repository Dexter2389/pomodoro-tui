use tui::layout::Alignment;
use tui::style::{Color, Style};
use tui::widgets::{Block, BorderType, Borders, Paragraph};

pub fn render_analytics<'a>() -> Paragraph<'a> {
    let analytics = Paragraph::new("THIS IS A PLACEHOLDER SECTION for Analytics")
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Analytics")
                .border_type(BorderType::Plain),
        );
    
    return analytics;
}