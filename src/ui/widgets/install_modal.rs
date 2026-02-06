use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Text},
    widgets::{Block, Borders, Clear, Paragraph, Widget, Wrap},
};

pub struct InstallModal<'a> {
    command: &'a str,
    output: &'a str,
}

impl<'a> InstallModal<'a> {
    pub fn new(command: &'a str, output: &'a str) -> Self {
        Self { command, output }
    }
}

impl<'a> Widget for InstallModal<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Fixed width modal (80 chars max, or 90% of screen)
        let modal_width = (area.width as f32 * 0.9).min(80.0) as u16;
        let modal_height = (area.height as f32 * 0.7).min(25.0) as u16;

        let area = center_rect(area, modal_width, modal_height);

        // Clear background
        Clear.render(area, buf);

        // Main block with cyan border
        let block = Block::default()
            .title(" Installing Skill ")
            .title_style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .style(Style::default().bg(Color::Black));

        let inner = block.inner(area);
        block.render(area, buf);

        // Layout inside modal
        let chunks = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                Constraint::Length(2), // Command section
                Constraint::Min(3),    // Output area
                Constraint::Length(1), // Hint
            ])
            .margin(1)
            .split(inner);

        // Command section - no box, just text
        let command_text = Text::from(vec![
            Line::from("Command:").style(Style::default().fg(Color::Gray)),
            Line::from(self.command).style(Style::default().fg(Color::Green)),
        ]);
        Paragraph::new(command_text).render(chunks[0], buf);

        // Output area - single border with title, simpler styling
        let output_block = Block::default()
            .title(" Output ")
            .title_style(Style::default().fg(Color::Gray))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray))
            .style(Style::default().bg(Color::Black));

        let output_inner = output_block.inner(chunks[1]);
        output_block.render(chunks[1], buf);

        let output_text = if self.output.is_empty() {
            Text::from("Running...").style(Style::default().fg(Color::Yellow))
        } else {
            Text::from(self.output)
        };

        Paragraph::new(output_text)
            .wrap(Wrap::default())
            .render(output_inner, buf);

        // Hint at bottom
        let hint = Line::from("Press Enter or Esc to close")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::DarkGray));
        Paragraph::new(hint).render(chunks[2], buf);
    }
}

fn center_rect(area: Rect, width: u16, height: u16) -> Rect {
    let horizontal = Layout::horizontal([Constraint::Length(width)]).flex(Flex::Center);
    let vertical = Layout::vertical([Constraint::Length(height)]).flex(Flex::Center);

    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
