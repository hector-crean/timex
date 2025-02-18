pub mod diff;
pub mod error;
use error::GitError;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use gix::{self};
use colored::*;

// Each git commit has a unique hash



#[derive(Debug, Serialize, Deserialize)]
pub struct GitCommit {
    pub hash: String,
    pub title: String,
    pub body: Option<String>,
    pub author: String,
    pub date: DateTime<Utc>,
    #[serde(skip)]
    diff: Option<String>, // We'll skip serializing the diff as it can be large
}


impl<'repo> TryFrom<&gix::Commit<'repo>> for GitCommit {
    type Error = GitError;

    fn try_from(commit: &gix::Commit) -> Result<Self, Self::Error> {
        let hash = commit.id.to_hex().to_string();
        let author = commit.author()?.name.to_string();
        let time = commit.time()?;
        let msg = commit.message()?;
        let title = msg.title.to_string();
        let body = msg.body().map(|b| b.to_string());
        
        
        Ok(GitCommit {
            hash,
            title,
            body,
            author,
            date: DateTime::from_timestamp(time.seconds, 0).unwrap_or_default(),
            diff: None,
        })
    }
}

impl GitCommit {
    /// Returns all commits in the repository at the given path
    pub fn from_repo(repo_path:impl Into<std::path::PathBuf>) -> Result<Vec<Self>, GitError> {
        let repo = gix::open(repo_path)?;
        let mut commits = Vec::new();
        
        for info in repo.head_commit()?.ancestors().all()? {
            let info = info?;
            let commit = info.object()?;
            commits.push(GitCommit::try_from(&commit)?);
        }
        
        Ok(commits)
    }

    // pub fn get_diff(&self, repo_path: impl Into<std::path::PathBuf>) -> Result<String, GitError> {
    //     let repo = gix::open(repo_path)?;
    //     let commit = repo.find_object(gix::hash::ObjectId::from_hex(self.hash.as_bytes())?.into())?;
    //     let commit = commit.peel_to_commit()?;
        
    //     let parent = commit.parent(0)?;
    //     let parent_tree = parent.tree()?;
    //     let current_tree = commit.tree()?;

    //     let diff = current_tree.changes()?.for_each_to_obtain_tree(&parent_tree, |_|{})?;
    //     Ok(format!("{}", diff))
    // }
}

impl std::fmt::Display for GitCommit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {}", "commit".bright_yellow(), self.hash.bright_white())?;
        
        if !self.author.is_empty() {
            writeln!(f, "{}: {}", "Author".bright_blue(), self.author)?;
        }
        
        writeln!(f, "{}: {}", "Date".bright_blue(), self.date)?;
        
        if !self.title.is_empty() {
            writeln!(f, "{}: {}", "Title".bright_blue(), self.title.bright_green())?;
        }
        
        if let Some(body) = &self.body {
            if !body.is_empty() {
                writeln!(f, "{}: {}", "Body".bright_blue(), body.bright_green())?;
                // for line in body.lines() {
                //     writeln!(f, "    {}", line)?;
                // }
            }
        }
        
        // Add an extra newline to separate commits when displaying multiple
        writeln!(f)
    }
}




