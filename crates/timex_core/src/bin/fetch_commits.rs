use timex_core::git::{GitCommit, error::GitError};

const REPO_PATH: &str = "/Users/hectorcrean/rust/timex";

pub fn main() -> Result<(), GitError> {
    let commits = GitCommit::from_repo(REPO_PATH)?;
    for commit in commits {
        println!("{}", commit);
    }
    Ok(())
}

