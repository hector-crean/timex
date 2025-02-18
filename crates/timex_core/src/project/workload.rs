use crate::git::diff::{get_commit_diff, CommitDiff, CommitTreeIterator, Walker};
use crate::git::error::GitError;
use async_openai::config::OpenAIConfig;
use async_openai::types::ChatCompletionRequestUserMessage;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror;
use toml;

use super::{ParseWorkloadError, Project, UserConfig};

use async_openai::{
    types::{ChatCompletionRequestMessage, CreateChatCompletionRequestArgs, Role},
    Client as OpenAIClient,
};

#[derive(Debug, Deserialize)]
pub struct Workload {
    pub user: UserConfig,
    pub projects: Vec<Project>,
}

#[derive(Debug, thiserror::Error)]
pub enum WorkloadError {
    #[error("Git error: {0}")]
    Git(#[from] GitError),
    #[error("OpenAI API error: {0}")]
    OpenAI(#[from] async_openai::error::OpenAIError),
    #[error("Failed to build chat completion request: {0}")]
    RequestBuild(async_openai::error::OpenAIError),
}

impl Workload {
    pub fn from_toml_file(path: &str) -> Result<Workload, ParseWorkloadError> {
        let content = std::fs::read_to_string(path)?;
        let config: Self = toml::from_str(&content)?;
        Ok(config)
    }
    fn walk_commit_diffs(repo_path: &str) -> Result<Vec<CommitDiff>, GitError> {
        let repo = gix::open(repo_path)?;
        let mut walker = CommitTreeIterator::new(&repo)?;
        let mut diffs: Vec<CommitDiff> = Vec::new();

        walker.adjacent_pairs(|old_commit, new_commit| {
            let diff = get_commit_diff(&repo, &old_commit.to_string(), &new_commit.to_string())?;
            diffs.push(diff);
            Ok(())
        })?;

        Ok(diffs)
    }

    pub fn generate_workload(&self) -> Result<HashMap<String, Vec<CommitDiff>>, GitError> {
        let mut all_diffs = HashMap::new();
        for project in &self.projects {
            let diffs = Self::walk_commit_diffs(&project.git_url)?;
            all_diffs.insert(project.name.clone(), diffs);
        }
        Ok(all_diffs)
    }

    pub async fn generate_report(
        &self,
        api_key: &str,
    ) -> Result<HashMap<String, String>, WorkloadError> {
        let config = OpenAIConfig::default().with_api_key(api_key);
        let client = OpenAIClient::with_config(config);
        let workload = self.generate_workload()?;
        let mut reports = HashMap::new();

        for (project_name, diffs) in workload {
            let mut diff_descriptions = String::new();
            for diff in diffs {
                diff_descriptions.push_str(&format!("{}\n", diff));
            }

            let messages = vec![
                ChatCompletionRequestMessage::User(ChatCompletionRequestUserMessage {
                    content: async_openai::types::ChatCompletionRequestUserMessageContent::Text(format!(
                        "Please analyze these git commits and write a brief summary of the work done:\n{}",
                        diff_descriptions
                    )),
                    name: None,
                })
            ];

            let request = CreateChatCompletionRequestArgs::default()
                .model("gpt-4-turbo-preview")
                .messages(messages)
                .build()?;

            let response = client.chat().create(request).await?;
            let summary = response.choices[0]
                .message
                .content
                .clone()
                .unwrap_or_default();

            reports.insert(project_name, summary);
        }

        Ok(reports)
    }
}
