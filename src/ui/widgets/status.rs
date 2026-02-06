use crate::app::state::Mode;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

pub struct StatusBar {
    mode: Mode,
    message: Option<String>,
    loading: bool,
    search_query: String,
}

impl StatusBar {
    pub fn new(mode: Mode, message: Option<String>, loading: bool, search_query: String) -> Self {
        Self {
            mode,
            message,
            loading,
            search_query,
        }
    }
}

impl Widget for StatusBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mode_span = match self.mode {
            Mode::List => Span::styled(
                " LIST ",
                Style::default()
                    .bg(Color::Yellow)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            ),
            Mode::Detail => Span::styled(
                " DETAIL ",
                Style::default()
                    .bg(Color::Green)
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Mode::Search => Span::styled(
                format!(" SEARCH: {}█ ", self.search_query),
                Style::default()
                    .bg(Color::Yellow)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            ),
            Mode::Help => Span::styled(
                " HELP ",
                Style::default()
                    .bg(Color::Magenta)
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Mode::Installing => Span::styled(
                " INSTALL ",
                Style::default()
                    .bg(Color::Yellow)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            ),
        };

        let loading_span = if self.loading {
            Span::styled(" ⟳ Loading... ", Style::default().fg(Color::Yellow))
        } else {
            Span::raw("")
        };

        let message_span = self.message.map_or(Span::raw(""), |msg| {
            Span::styled(format!(" {} ", msg), Style::default().fg(Color::White))
        });

        let hints = match self.mode {
            Mode::List => " q:quit  /:search  i:install  r:refresh  ?:help  Tab:switch view ",
            Mode::Detail => " Esc:back  j/k:navigate ",
            Mode::Search => " Esc:cancel  Enter:confirm ",
            Mode::Help => " Esc/?:close ",
            Mode::Installing => " Enter/Esc:close ",
        };

        let hints_span = Span::styled(hints, Style::default().fg(Color::White));

        let line = Line::from(vec![mode_span, loading_span, message_span, hints_span]);

        Paragraph::new(line)
            .style(Style::default().bg(Color::Black))
            .render(area, buf);
    }
}
