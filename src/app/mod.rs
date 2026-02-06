pub mod events;
pub mod state;

use crate::data::ViewKind;
use crossterm::event::KeyCode;
use events::{key_to_action, Action, Event, KeyAction};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use state::{AppState, Mode};

pub struct App {
    pub state: AppState,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: AppState::new(),
        }
    }

    pub fn update(&mut self, event: Event) -> Vec<Action> {
        let mut actions = Vec::new();

        match event {
            Event::Key(key) => {
                if self.state.mode == Mode::Search {
                    match key.code {
                        KeyCode::Char(c) => {
                            self.state.search_query.push(c);
                            self.apply_filter();
                            return actions;
                        }
                        KeyCode::Backspace => {
                            self.state.search_query.pop();
                            self.apply_filter();
                            return actions;
                        }
                        _ => {}
                    }
                }
                if let Some(action) = key_to_action(key) {
                    actions.extend(self.handle_key_action(action));
                }
            }
            Event::ViewLoaded { view, skills } => {
                if let Some(view_state) = self.state.views.get_mut(&view) {
                    view_state.skills = skills;
                    view_state.loading = false;
                    view_state.error = None;
                    if view_state.list_state.selected().is_none() && !view_state.skills.is_empty() {
                        view_state.list_state.select(Some(0));
                    }
                }
            }
            Event::DetailLoaded { key, markdown } => {
                self.state.detail_loading = false;
                self.state
                    .detail_cache
                    .insert(key, crate::data::SkillDetail { markdown });
            }
            Event::Error(msg) => {
                self.state.status_message = Some(format!("Error: {}", msg));
                self.state.detail_loading = false;
                if let Some(view_state) = self.state.views.get_mut(&self.state.current_view) {
                    view_state.loading = false;
                    view_state.error = Some(msg);
                }
            }
        }

        actions
    }

    fn handle_key_action(&mut self, action: KeyAction) -> Vec<Action> {
        let mut actions = Vec::new();

        match self.state.mode {
            Mode::List => match action {
                KeyAction::Quit => self.state.should_quit = true,
                KeyAction::NextTab => {
                    self.state.current_view = self.state.current_view.next();
                    if self.state.current_view_state().skills.is_empty() {
                        actions.push(Action::FetchView(self.state.current_view));
                    }
                }
                KeyAction::PrevTab => {
                    self.state.current_view = self.state.current_view.prev();
                    if self.state.current_view_state().skills.is_empty() {
                        actions.push(Action::FetchView(self.state.current_view));
                    }
                }
                KeyAction::SelectTab(idx) => {
                    let views = ViewKind::all();
                    if let Some(&view) = views.get(idx) {
                        self.state.current_view = view;
                        if self.state.current_view_state().skills.is_empty() {
                            actions.push(Action::FetchView(view));
                        }
                    }
                }
                KeyAction::Up => self.move_selection(-1),
                KeyAction::Down => self.move_selection(1),
                KeyAction::Top => self.select_first(),
                KeyAction::Bottom => self.select_last(),
                KeyAction::PageUp => self.move_selection(-10),
                KeyAction::PageDown => self.move_selection(10),
                KeyAction::Select => {
                    if let Some(skill) = self.state.selected_skill().cloned() {
                        self.state.mode = Mode::Detail;
                        if !self.state.detail_cache.contains_key(&skill.skill_id) {
                            self.state.detail_loading = true;
                            actions.push(Action::FetchDetail {
                                source: skill.source.clone(),
                                skill_id: skill.skill_id.clone(),
                            });
                        }
                    }
                }
                KeyAction::StartSearch => {
                    self.state.mode = Mode::Search;
                    self.state.search_query.clear();
                }
                KeyAction::Install => {
                    if let Some(skill) = self.state.selected_skill().cloned() {
                        actions.push(Action::InstallInteractive(skill));
                    }
                }
                KeyAction::Refresh => {
                    let view = self.state.current_view;
                    if let Some(view_state) = self.state.views.get_mut(&view) {
                        view_state.loading = true;
                    }
                    actions.push(Action::FetchView(view));
                }
                KeyAction::Help => {
                    self.state.mode = Mode::Help;
                }
                KeyAction::Back => {}
            },
            Mode::Detail => match action {
                KeyAction::Quit => self.state.should_quit = true,
                KeyAction::Back | KeyAction::Select => {
                    self.state.mode = Mode::List;
                    self.state.detail_scroll = 0;
                }
                KeyAction::Up => {
                    self.state.detail_scroll = self.state.detail_scroll.saturating_sub(1);
                }
                KeyAction::Down => {
                    self.state.detail_scroll = self.state.detail_scroll.saturating_add(1);
                }
                KeyAction::PageUp => {
                    self.state.detail_scroll = self.state.detail_scroll.saturating_sub(10);
                }
                KeyAction::PageDown => {
                    self.state.detail_scroll = self.state.detail_scroll.saturating_add(10);
                }
                KeyAction::Top => {
                    self.state.detail_scroll = 0;
                }
                _ => {}
            },
            Mode::Help => match action {
                KeyAction::Quit => self.state.should_quit = true,
                KeyAction::Back | KeyAction::Help => {
                    self.state.mode = Mode::List;
                }
                _ => {}
            },
            Mode::Search => match action {
                KeyAction::Back => {
                    self.state.mode = Mode::List;
                    self.state.search_query.clear();
                    self.clear_filter();
                }
                KeyAction::Select => {
                    self.state.mode = Mode::List;
                }
                _ => {}
            },
            Mode::Installing => match action {
                KeyAction::Quit => self.state.should_quit = true,
                KeyAction::Back | KeyAction::Select => {
                    self.state.mode = Mode::List;
                    self.state.install_output.clear();
                }
                _ => {}
            },
        }

        actions
    }

    fn move_selection(&mut self, delta: i32) {
        let view_state = self.state.current_view_state_mut();
        let len = view_state.len();
        if len == 0 {
            return;
        }

        let current = view_state.list_state.selected().unwrap_or(0) as i32;
        let new_idx = (current + delta).clamp(0, len as i32 - 1) as usize;
        view_state.list_state.select(Some(new_idx));
    }

    fn select_first(&mut self) {
        let view_state = self.state.current_view_state_mut();
        if !view_state.skills.is_empty() {
            view_state.list_state.select(Some(0));
        }
    }

    fn select_last(&mut self) {
        let view_state = self.state.current_view_state_mut();
        let len = view_state.len();
        if len > 0 {
            view_state.list_state.select(Some(len - 1));
        }
    }

    fn clear_filter(&mut self) {
        let view_state = self.state.current_view_state_mut();
        view_state.filtered_indices.clear();
    }

    fn apply_filter(&mut self) {
        let query = self.state.search_query.clone();
        let view_state = self.state.current_view_state_mut();

        if query.is_empty() {
            view_state.filtered_indices.clear();
            return;
        }

        let matcher = SkimMatcherV2::default();
        let mut scored: Vec<(usize, i64)> = view_state
            .skills
            .iter()
            .enumerate()
            .filter_map(|(idx, skill)| {
                let name_score = matcher.fuzzy_match(&skill.name, &query);
                let id_score = matcher.fuzzy_match(&skill.skill_id, &query);
                let source_score = matcher.fuzzy_match(&skill.source, &query);
                let best = name_score.max(id_score).max(source_score);
                best.map(|score| (idx, score))
            })
            .collect();

        scored.sort_by(|a, b| b.1.cmp(&a.1));
        view_state.filtered_indices = scored.into_iter().map(|(idx, _)| idx).collect();
        view_state
            .list_state
            .select(if view_state.filtered_indices.is_empty() {
                None
            } else {
                Some(0)
            });
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
