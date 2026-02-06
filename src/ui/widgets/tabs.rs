use crate::data::ViewKind;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Tabs as RataTabs, Widget},
};

pub struct TabsWidget {
    current: ViewKind,
}

impl TabsWidget {
    pub fn new(current: ViewKind) -> Self {
        Self { current }
    }
}

impl Widget for TabsWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let titles: Vec<Line> = ViewKind::all()
            .iter()
            .enumerate()
            .map(|(i, view)| {
                let num = format!("[{}] ", i + 1);
                let label = view.label();
                Line::from(vec![
                    Span::styled(num, Style::default().fg(Color::DarkGray)),
                    Span::raw(label),
                ])
            })
            .collect();

        let selected = match self.current {
            ViewKind::AllTime => 0,
            ViewKind::Trending => 1,
            ViewKind::Hot => 2,
        };

        let tabs = RataTabs::new(titles)
            .block(Block::default().borders(Borders::BOTTOM))
            .select(selected)
            .style(Style::default().fg(Color::White))
            .highlight_style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
            .divider(" │ ");

        tabs.render(area, buf);
    }
}
