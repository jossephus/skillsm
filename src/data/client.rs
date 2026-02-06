use crate::data::{parse, SkillSummary, ViewKind};
use crate::error::{AppError, Result};
use reqwest::Client;
use std::time::Duration;

#[derive(Clone)]
pub struct SkillsClient {
    client: Client,
    base_url: String,
}

impl SkillsClient {
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("skillsm/0.1.0")
            .build()?;

        Ok(Self {
            client,
            base_url: "https://skills.sh".to_string(),
        })
    }

    pub async fn fetch_skills(&self, view: ViewKind) -> Result<Vec<SkillSummary>> {
        let url = match view {
            ViewKind::AllTime => format!("{}/?view=all-time", self.base_url),
            ViewKind::Trending => format!("{}/?view=trending", self.base_url),
            ViewKind::Hot => format!("{}/?view=hot", self.base_url),
        };

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(AppError::Parse(format!(
                "HTTP error: {}",
                response.status()
            )));
        }

        let html = response.text().await?;
        let mut skills = parse::extract_skills_from_html(&html)?;

        match view {
            ViewKind::AllTime => {
                skills.sort_by(|a, b| b.installs.cmp(&a.installs));
            }
            ViewKind::Trending | ViewKind::Hot => {
                skills.sort_by(|a, b| {
                    let a_change = a.change.unwrap_or(0);
                    let b_change = b.change.unwrap_or(0);
                    b_change.cmp(&a_change)
                });
            }
        }

        Ok(skills)
    }

    pub async fn fetch_readme(&self, source: &str, skill_id: &str) -> Result<String> {
        // First try direct path with skill_id as folder name
        let branches = ["main", "master"];
        let path_templates = [
            "skills/{}/SKILL.md",
            ".skills/{}/SKILL.md",
            ".claude/skills/{}/SKILL.md",
        ];

        for template in &path_templates {
            for branch in &branches {
                let path = template.replace("{}", skill_id);
                let url = format!(
                    "https://raw.githubusercontent.com/{}/{}/{}",
                    source, branch, path
                );
                let response = self.client.get(&url).send().await?;
                if response.status().is_success() {
                    return Ok(response.text().await?);
                }
            }
        }

        // If not found, scan the skills directory for matching SKILL.md with name field
        for skills_dir in ["skills", ".skills", ".claude/skills"] {
            for branch in &branches {
                if let Ok(folders) = self.list_skill_folders(source, branch, skills_dir).await {
                    for folder in folders {
                        let url = format!(
                            "https://raw.githubusercontent.com/{}/{}/{}/{}/SKILL.md",
                            source, branch, skills_dir, folder
                        );
                        let response = self.client.get(&url).send().await?;
                        if response.status().is_success() {
                            let content = response.text().await?;
                            // Check if this SKILL.md has matching name in frontmatter
                            if self.skill_matches_name(&content, skill_id) {
                                return Ok(content);
                            }
                        }
                    }
                }
            }
        }

        // Try searching in plugins (e.g., plugins/*/skills/{skill_id}/SKILL.md)
        for branch in &branches {
            if let Ok(content) = self
                .search_plugins_for_skill(source, branch, skill_id)
                .await
            {
                return Ok(content);
            }
        }

        Err(AppError::Parse(format!(
            "SKILL.md not found for {}/{}",
            source, skill_id
        )))
    }

    async fn search_plugins_for_skill(
        &self,
        source: &str,
        branch: &str,
        skill_id: &str,
    ) -> Result<String> {
        // Try plugins directory (e.g., plugins/expo-app-design/skills/{skill_id})
        if let Ok(plugins) = self.list_skill_folders(source, branch, "plugins").await {
            for plugin in &plugins {
                let skills_path = format!("plugins/{}/skills", plugin);
                if let Ok(folders) = self.list_skill_folders(source, branch, &skills_path).await {
                    for folder in folders {
                        if folder == skill_id {
                            let url = format!(
                                "https://raw.githubusercontent.com/{}/{}/{}/{}/SKILL.md",
                                source, branch, skills_path, folder
                            );
                            let response = self.client.get(&url).send().await?;
                            if response.status().is_success() {
                                return Ok(response.text().await?);
                            }
                        }
                    }
                }
            }
        }

        // Try plugins/claude/*/skills/{skill_id} pattern (e.g., plugins/claude/prompts.chat/skills/skill-lookup)
        if let Ok(claude_plugins) = self
            .list_skill_folders(source, branch, "plugins/claude")
            .await
        {
            for plugin in claude_plugins {
                let skills_path = format!("plugins/claude/{}/skills", plugin);
                if let Ok(folders) = self.list_skill_folders(source, branch, &skills_path).await {
                    for folder in folders {
                        if folder == skill_id {
                            let url = format!(
                                "https://raw.githubusercontent.com/{}/{}/{}/{}/SKILL.md",
                                source, branch, skills_path, folder
                            );
                            let response = self.client.get(&url).send().await?;
                            if response.status().is_success() {
                                return Ok(response.text().await?);
                            }
                        }
                    }
                }
            }
        }

        Err(AppError::Parse("Not found in plugins".to_string()))
    }

    async fn list_skill_folders(
        &self,
        source: &str,
        branch: &str,
        skills_dir: &str,
    ) -> Result<Vec<String>> {
        let url = format!(
            "https://api.github.com/repos/{}/contents/{}?ref={}",
            source, skills_dir, branch
        );
        let response = self
            .client
            .get(&url)
            .header("Accept", "application/vnd.github.v3+json")
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(AppError::Parse(
                "Failed to list skills directory".to_string(),
            ));
        }

        let items: Vec<serde_json::Value> = response
            .json()
            .await
            .map_err(|e| AppError::Parse(e.to_string()))?;

        Ok(items
            .iter()
            .filter(|item| item["type"].as_str() == Some("dir"))
            .filter_map(|item| item["name"].as_str().map(String::from))
            .collect())
    }

    fn skill_matches_name(&self, content: &str, skill_id: &str) -> bool {
        // Parse YAML frontmatter to check name field
        if let Some(start) = content.find("---") {
            if let Some(end) = content[start + 3..].find("---") {
                let frontmatter = &content[start + 3..start + 3 + end];
                for line in frontmatter.lines() {
                    let line = line.trim();
                    if line.starts_with("name:") {
                        let name = line.trim_start_matches("name:").trim();
                        return name == skill_id;
                    }
                }
            }
        }
        false
    }
}

impl Default for SkillsClient {
    fn default() -> Self {
        Self::new().expect("Failed to create HTTP client")
    }
}
