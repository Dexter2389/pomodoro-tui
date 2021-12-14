use tui::layout::Alignment;
use tui::style::{Color, Style};
use tui::widgets::{Block, BorderType, Borders, Paragraph};

pub fn render_copyright<'a>() -> Paragraph<'a> {
    let copyright = Paragraph::new("pomodoro-tui 2021 - all rights reserved")
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Copyright")
                .border_type(BorderType::Plain),
        );
    
    return copyright;
}