use crate::data::{SkillSummary, ViewKind};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug, Clone)]
pub enum Event {
    Key(KeyEvent),
    ViewLoaded {
        view: ViewKind,
        skills: Vec<SkillSummary>,
    },
    DetailLoaded {
        key: String,
        markdown: String,
    },
    Error(String),
}

#[derive(Debug, Clone)]
pub enum Action {
    FetchView(ViewKind),
    FetchDetail { source: String, skill_id: String },
    InstallInteractive(SkillSummary),
}

pub fn key_to_action(key: KeyEvent) -> Option<KeyAction> {
    let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);

    match key.code {
        KeyCode::Char('q') | KeyCode::Char('Q') => Some(KeyAction::Quit),
        KeyCode::Char('c') if ctrl => Some(KeyAction::Quit),

        KeyCode::Tab => Some(KeyAction::NextTab),
        KeyCode::BackTab => Some(KeyAction::PrevTab),
        KeyCode::Char('1') => Some(KeyAction::SelectTab(0)),
        KeyCode::Char('2') => Some(KeyAction::SelectTab(1)),
        KeyCode::Char('3') => Some(KeyAction::SelectTab(2)),

        KeyCode::Up | KeyCode::Char('k') => Some(KeyAction::Up),
        KeyCode::Down | KeyCode::Char('j') => Some(KeyAction::Down),
        KeyCode::Home | KeyCode::Char('g') => Some(KeyAction::Top),
        KeyCode::End | KeyCode::Char('G') => Some(KeyAction::Bottom),
        KeyCode::PageUp => Some(KeyAction::PageUp),
        KeyCode::PageDown => Some(KeyAction::PageDown),

        KeyCode::Enter => Some(KeyAction::Select),
        KeyCode::Esc => Some(KeyAction::Back),

        KeyCode::Char('/') => Some(KeyAction::StartSearch),
        KeyCode::Char('i') => Some(KeyAction::Install),
        KeyCode::Char('r') => Some(KeyAction::Refresh),
        KeyCode::Char('?') => Some(KeyAction::Help),

        _ => None,
    }
}

#[derive(Debug, Clone, Copy)]
pub enum KeyAction {
    Quit,
    NextTab,
    PrevTab,
    SelectTab(usize),
    Up,
    Down,
    Top,
    Bottom,
    PageUp,
    PageDown,
    Select,
    Back,
    StartSearch,
    Install,
    Refresh,
    Help,
}
