use crate::app::state::ViewState;
use crate::data::ViewKind;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Row, StatefulWidget, Table, TableState},
};

pub struct SkillListWidget<'a> {
    view_state: &'a ViewState,
    view_kind: ViewKind,
}

impl<'a> SkillListWidget<'a> {
    pub fn new(view_state: &'a ViewState, view_kind: ViewKind) -> Self {
        Self {
            view_state,
            view_kind,
        }
    }
}

impl<'a> StatefulWidget for SkillListWidget<'a> {
    type State = TableState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let skills = self.view_state.visible_skills();

        let header = Row::new(vec!["Name", "Source", "Installs"])
            .style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
            .bottom_margin(1);

        let rows: Vec<Row> = skills
            .iter()
            .map(|skill| {
                Row::new(vec![
                    skill.name.clone(),
                    skill.source.clone(),
                    skill.installs.to_string(),
                ])
                .style(Style::default())
                .height(1)
            })
            .collect();

        let widths = [
            Constraint::Percentage(45),
            Constraint::Percentage(40),
            Constraint::Percentage(15),
        ];

        let title = format!(" {} ({} skills) ", self.view_kind.label(), skills.len());

        let table = Table::new(rows, widths)
            .header(header)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(title)
                    .border_style(Style::default().fg(Color::White)),
            )
            .row_highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("▶ ");

        StatefulWidget::render(table, area, buf, state);
    }
}
