use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillSummary {
    pub source: String,
    pub skill_id: String,
    pub name: String,
    pub installs: i64,
    #[serde(default)]
    pub installs_yesterday: Option<i64>,
    #[serde(default)]
    pub change: Option<i64>,
}

impl SkillSummary {
    pub fn install_command(&self) -> String {
        format!(
            "npx skills add https://github.com/{} --skill {}",
            self.source, self.skill_id
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ViewKind {
    #[default]
    AllTime,
    Trending,
    Hot,
}

impl ViewKind {
    pub fn label(&self) -> &'static str {
        match self {
            ViewKind::AllTime => "All Time",
            ViewKind::Trending => "Trending (24h)",
            ViewKind::Hot => "Hot",
        }
    }

    pub fn all() -> [ViewKind; 3] {
        [ViewKind::AllTime, ViewKind::Trending, ViewKind::Hot]
    }

    pub fn next(&self) -> ViewKind {
        match self {
            ViewKind::AllTime => ViewKind::Trending,
            ViewKind::Trending => ViewKind::Hot,
            ViewKind::Hot => ViewKind::AllTime,
        }
    }

    pub fn prev(&self) -> ViewKind {
        match self {
            ViewKind::AllTime => ViewKind::Hot,
            ViewKind::Trending => ViewKind::AllTime,
            ViewKind::Hot => ViewKind::Trending,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SkillDetail {
    pub markdown: String,
}
