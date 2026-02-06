use crate::data::SkillSummary;
use crate::error::{AppError, Result};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawSkill {
    source: String,
    skill_id: String,
    name: String,
    installs: i64,
    #[serde(default)]
    installs_yesterday: Option<i64>,
    #[serde(default)]
    change: Option<i64>,
}

impl From<RawSkill> for SkillSummary {
    fn from(raw: RawSkill) -> Self {
        SkillSummary {
            source: raw.source,
            skill_id: raw.skill_id,
            name: raw.name,
            installs: raw.installs,
            installs_yesterday: raw.installs_yesterday,
            change: raw.change,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SkillsWrapper {
    skills: Vec<RawSkill>,
}

pub fn extract_skills_from_html(html: &str) -> Result<Vec<SkillSummary>> {
    let unescaped = html.replace(r#"\""#, "\"");

    if let Some(skills) = try_parse_wrapper_json(&unescaped) {
        if !skills.is_empty() {
            return Ok(skills);
        }
    }

    if let Some(skills) = try_parse_wrapper_json(html) {
        if !skills.is_empty() {
            return Ok(skills);
        }
    }

    if let Some(skills) = try_parse_skills_array(&unescaped) {
        if !skills.is_empty() {
            return Ok(skills);
        }
    }

    Err(AppError::Parse(
        "Could not extract skills data from HTML".to_string(),
    ))
}

fn try_parse_wrapper_json(html: &str) -> Option<Vec<SkillSummary>> {
    let marker = r#"[{"skills":"#;
    let start_pos = html.find(marker)?;

    let search_area = &html[start_pos..];

    let mut depth = 0;
    let mut end_idx = 0;

    for (i, c) in search_area.char_indices() {
        match c {
            '[' => depth += 1,
            ']' => {
                depth -= 1;
                if depth == 0 {
                    end_idx = i + 1;
                    break;
                }
            }
            _ => {}
        }
    }

    if end_idx == 0 {
        return None;
    }

    let json_str = &search_area[..end_idx];

    if let Ok(wrappers) = serde_json::from_str::<Vec<SkillsWrapper>>(json_str) {
        if let Some(first) = wrappers.first() {
            return Some(first.skills.iter().cloned().map(Into::into).collect());
        }
    }

    None
}

fn try_parse_skills_array(html: &str) -> Option<Vec<SkillSummary>> {
    let marker = r#""source":"#;
    let start_pos = html.find(marker)?;

    let before = &html[..start_pos];
    let array_start = before.rfind('[')?;

    if !before[array_start..].contains("{") {
        return None;
    }

    let search_area = &html[array_start..];
    let mut depth = 0;
    let mut end_idx = 0;

    for (i, c) in search_area.char_indices() {
        match c {
            '[' => depth += 1,
            ']' => {
                depth -= 1;
                if depth == 0 {
                    end_idx = i + 1;
                    break;
                }
            }
            _ => {}
        }
    }

    if end_idx == 0 {
        return None;
    }

    let json_str = &search_area[..end_idx];

    if let Ok(skills) = serde_json::from_str::<Vec<RawSkill>>(json_str) {
        return Some(skills.into_iter().map(Into::into).collect());
    }

    None
}
