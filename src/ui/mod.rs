pub mod widgets;

use crate::app::state::Mode;
use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::TableState,
    Frame,
};
use widgets::{DetailWidget, HelpWidget, InstallModal, SkillListWidget, StatusBar, TabsWidget};

pub fn draw(frame: &mut Frame, app: &App) {
    if app.state.mode == Mode::Detail {
        draw_detail_screen(frame, app);
        return;
    }

    if app.state.mode == Mode::Installing {
        draw_list_screen(frame, app);
        let modal = InstallModal::new(&app.state.install_command, &app.state.install_output);
        frame.render_widget(modal, frame.area());
        return;
    }

    draw_list_screen(frame, app);

    if app.state.mode == Mode::Help {
        frame.render_widget(HelpWidget, frame.area());
    }
}

fn draw_list_screen(frame: &mut Frame, app: &App) {
    let view_state = app.state.current_view_state();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(10),
            Constraint::Length(1),
        ])
        .split(frame.area());

    let tabs = TabsWidget::new(app.state.current_view);
    frame.render_widget(tabs, chunks[0]);

    let skill_list = SkillListWidget::new(view_state, app.state.current_view);

    let mut table_state = TableState::default();
    table_state.select(view_state.list_state.selected());
    frame.render_stateful_widget(skill_list, chunks[1], &mut table_state);

    let status = StatusBar::new(
        app.state.mode,
        app.state.status_message.clone(),
        view_state.loading,
        app.state.search_query.clone(),
    );
    frame.render_widget(status, chunks[2]);
}

fn draw_detail_screen(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10), Constraint::Length(1)])
        .split(frame.area());

    let view_state = app.state.current_view_state();
    let selected_skill = view_state.selected_skill();
    let markdown = selected_skill.and_then(|s| {
        app.state
            .detail_cache
            .get(&s.skill_id)
            .map(|d| d.markdown.as_str())
    });
    let detail = DetailWidget::new(
        selected_skill,
        markdown,
        app.state.detail_loading,
        app.state.detail_scroll,
    );
    frame.render_widget(detail, chunks[0]);

    let status = StatusBar::new(
        app.state.mode,
        app.state.status_message.clone(),
        app.state.detail_loading,
        String::new(),
    );
    frame.render_widget(status, chunks[1]);
}
