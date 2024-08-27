use std::error::Error;

use std::{fmt, io};

use crate::git::GitError::GitCommandFailed;
use std::process::Command;

#[derive(Debug)]
pub enum GitError {
    IoError(io::Error),
    GitCommandFailed(String),
    EmptyDiff,
}

impl fmt::Display for GitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GitError::IoError(err) => write!(f, "IO error: {}", err),
            GitError::GitCommandFailed(err) => write!(f, "Git command failed: {}", err),
            GitError::EmptyDiff => write!(f, "The diff is empty"),
        }
    }
}

impl Error for GitError {}

impl From<io::Error> for GitError {
    fn from(err: io::Error) -> GitError {
        GitError::IoError(err)
    }
}

pub fn get_diff() -> Result<String, GitError> {
    let output = Command::new("git").args(&["diff", "--staged"]).output()?;

    if output.status.success() {
        if output.stdout.is_empty() {
            Err(GitError::EmptyDiff)
        } else {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        }
    } else {
        Err(GitCommandFailed("Failed to get diff".to_string()))
    }
}

pub fn commit(message: &str) -> Result<(), GitError> {
    let output = Command::new("git")
        .args(&["commit", "-m", message])
        .status()?;

    if output.success() {
        Ok(())
    } else {
        Err(GitCommandFailed("Failed to commit changes".to_string()))
    }
}
