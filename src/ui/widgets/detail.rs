use crate::data::SkillSummary;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

pub struct DetailWidget<'a> {
    skill: Option<&'a SkillSummary>,
    markdown: Option<&'a str>,
    loading: bool,
    scroll: u16,
}

impl<'a> DetailWidget<'a> {
    pub fn new(
        skill: Option<&'a SkillSummary>,
        markdown: Option<&'a str>,
        loading: bool,
        scroll: u16,
    ) -> Self {
        Self {
            skill,
            markdown,
            loading,
            scroll,
        }
    }
}

impl<'a> Widget for DetailWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Skill Detail ")
            .border_style(Style::default().fg(Color::Cyan));

        let inner = block.inner(area);
        block.render(area, buf);

        let Some(skill) = self.skill else {
            let placeholder =
                Paragraph::new("No skill selected").style(Style::default().fg(Color::DarkGray));
            placeholder.render(inner, buf);
            return;
        };

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(6), Constraint::Min(1)])
            .split(inner);

        let header_lines = vec![
            Line::from(vec![
                Span::styled("Name: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::styled(&skill.name, Style::default().fg(Color::White)),
            ]),
            Line::from(vec![
                Span::styled("Source: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::styled(&skill.source, Style::default().fg(Color::Cyan)),
            ]),
            Line::from(vec![
                Span::styled("Installs: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(skill.installs.to_string()),
            ]),
            Line::default(),
            Line::from(vec![
                Span::styled("Install: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::styled(skill.install_command(), Style::default().fg(Color::Green)),
            ]),
        ];

        Paragraph::new(header_lines).render(chunks[0], buf);

        if self.loading {
            let loading_msg =
                Paragraph::new("⟳ Loading README...").style(Style::default().fg(Color::Yellow));
            loading_msg.render(chunks[1], buf);
        } else if let Some(md) = self.markdown {
            let lines: Vec<Line> = md.lines().map(|l| Line::from(l.to_string())).collect();
            Paragraph::new(lines)
                .scroll((self.scroll, 0))
                .render(chunks[1], buf);
        } else {
            let hint =
                Paragraph::new("No README available").style(Style::default().fg(Color::DarkGray));
            hint.render(chunks[1], buf);
        }
    }
}
