pub mod workload;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror;
use toml;

// Each git commit has a unique hash

#[derive(Debug, thiserror::Error)]
pub enum ParseWorkloadError {
    #[error("Failed to read config file: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Failed to parse TOML: {0}")]
    TomlError(#[from] toml::de::Error),
}



#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct Project {
    pub name: String,
    pub code: String,
    pub description: String,
    pub git_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeEntry {
    pub project: Project,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkConfig {
    pub user: UserConfig,
    pub project: Vec<Project>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserConfig {
    pub name: String,
    pub email: String,
}

#[cfg(test)]
mod tests {
    use crate::project::workload::Workload;

    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[test]
    fn test_workload_from_valid_toml() {
        let config_content = r#"
            [user]
            name = "John Doe"
            email = "john@example.com"

            [[projects]]
            name = "Project 1"
            code = "P1"
            description = "Test Project 1"
            git_url = "https://github.com/test/project1"
        "#;

        let mut temp_file = NamedTempFile::new().unwrap();
        write!(temp_file, "{}", config_content).unwrap();
        
        let workload = Workload::from_toml_file(temp_file.path().to_str().unwrap()).unwrap();
        
        assert_eq!(workload.user.name, "John Doe");
        assert_eq!(workload.user.email, "john@example.com");
        assert_eq!(workload.projects.len(), 1);
        assert_eq!(workload.projects[0].name, "Project 1");
        assert_eq!(workload.projects[0].code, "P1");
    }

    #[test]
    fn test_workload_from_invalid_toml() {
        let invalid_content = r#"
            [user]
            name = "Invalid
        "#;

        let mut temp_file = NamedTempFile::new().unwrap();
        write!(temp_file, "{}", invalid_content).unwrap();
        
        let result = Workload::from_toml_file(temp_file.path().to_str().unwrap());
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ParseWorkloadError::TomlError(_)));
    }

    #[test]
    fn test_workload_from_nonexistent_file() {
        let result = Workload::from_toml_file("nonexistent_file.toml");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ParseWorkloadError::IoError(_)));
    }
}
