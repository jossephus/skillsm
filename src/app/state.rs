use crate::data::{SkillDetail, SkillSummary, ViewKind};
use ratatui::widgets::ListState;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Mode {
    #[default]
    List,
    Detail,
    Search,
    Help,
    Installing,
}

#[derive(Debug, Default)]
pub struct ViewState {
    pub skills: Vec<SkillSummary>,
    pub filtered_indices: Vec<usize>,
    pub list_state: ListState,
    pub loading: bool,
    pub error: Option<String>,
}

impl ViewState {
    pub fn selected_skill(&self) -> Option<&SkillSummary> {
        let idx = self.list_state.selected()?;
        if self.filtered_indices.is_empty() {
            self.skills.get(idx)
        } else {
            self.filtered_indices
                .get(idx)
                .and_then(|&i| self.skills.get(i))
        }
    }

    pub fn visible_skills(&self) -> Vec<&SkillSummary> {
        if self.filtered_indices.is_empty() {
            self.skills.iter().collect()
        } else {
            self.filtered_indices
                .iter()
                .filter_map(|&i| self.skills.get(i))
                .collect()
        }
    }

    pub fn len(&self) -> usize {
        if self.filtered_indices.is_empty() {
            self.skills.len()
        } else {
            self.filtered_indices.len()
        }
    }
}

#[derive(Debug, Default)]
pub struct AppState {
    pub mode: Mode,
    pub current_view: ViewKind,
    pub views: HashMap<ViewKind, ViewState>,
    pub search_query: String,
    pub detail_cache: HashMap<String, SkillDetail>,
    pub detail_loading: bool,
    pub detail_scroll: u16,
    pub status_message: Option<String>,
    pub should_quit: bool,
    pub install_output: String,
    pub install_command: String,
}

impl AppState {
    pub fn new() -> Self {
        let mut views = HashMap::new();
        for view in ViewKind::all() {
            views.insert(view, ViewState::default());
        }
        Self {
            views,
            ..Default::default()
        }
    }

    pub fn current_view_state(&self) -> &ViewState {
        self.views.get(&self.current_view).unwrap()
    }

    pub fn current_view_state_mut(&mut self) -> &mut ViewState {
        self.views.get_mut(&self.current_view).unwrap()
    }

    pub fn selected_skill(&self) -> Option<&SkillSummary> {
        self.current_view_state().selected_skill()
    }
}
