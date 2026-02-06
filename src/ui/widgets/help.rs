use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Widget},
};

pub struct HelpWidget;

impl Widget for HelpWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let popup_area = centered_rect(60, 70, area);

        Clear.render(popup_area, buf);

        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Help ")
            .title_alignment(Alignment::Center)
            .border_style(Style::default().fg(Color::Cyan));

        let inner = block.inner(popup_area);
        block.render(popup_area, buf);

        let keybinds: Vec<(&str, &str)> = vec![
            ("Navigation", ""),
            ("", ""),
            ("j / ↓", "Move down"),
            ("k / ↑", "Move up"),
            ("g / Home", "Go to top"),
            ("G / End", "Go to bottom"),
            ("PgUp/PgDn", "Page up/down"),
            ("", ""),
            ("Views", ""),
            ("", ""),
            ("Tab", "Next view"),
            ("Shift+Tab", "Previous view"),
            ("1/2/3", "Select view directly"),
            ("", ""),
            ("Actions", ""),
            ("", ""),
            ("Enter", "View detail"),
            ("/", "Search"),
            ("i", "Install selected skill"),
            ("r", "Refresh"),
            ("", ""),
            ("General", ""),
            ("", ""),
            ("?", "Toggle help"),
            ("Esc", "Back / Cancel"),
            ("q", "Quit"),
        ];

        let lines: Vec<Line> = keybinds
            .iter()
            .map(|(key, desc)| {
                if desc.is_empty() {
                    if key.is_empty() {
                        Line::default()
                    } else {
                        Line::from(Span::styled(
                            format!("── {} ──", key),
                            Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::BOLD),
                        ))
                    }
                } else {
                    Line::from(vec![
                        Span::styled(
                            format!("{:>12}", key),
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::raw("  "),
                        Span::raw(*desc),
                    ])
                }
            })
            .collect();

        Paragraph::new(lines)
            .alignment(Alignment::Left)
            .render(inner, buf);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);

    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
