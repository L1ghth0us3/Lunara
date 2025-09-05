use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub enum Error {
    NotRepository,
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NotRepository => write!(f, "not a git repository"),
            Error::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BranchRef {
    Named(String),
    Detached(String), // short commit id
}

impl fmt::Display for BranchRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BranchRef::Named(name) => write!(f, "{}", name),
            BranchRef::Detached(sha) => write!(f, "DETACHED@{}", sha),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepoInfo {
    pub root: PathBuf,
    pub branch: BranchRef,
}

pub fn repo_info() -> Result<RepoInfo, Error> {
    let repo = gix::discover(std::env::current_dir().unwrap_or_default())
        .map_err(|_| Error::NotRepository)?;

    let git_dir = repo.git_dir().to_path_buf();
    let root = git_dir
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| git_dir.clone());

    // Branch name or detached: read HEAD file
    let head_content = std::fs::read_to_string(git_dir.join("HEAD"))
        .map_err(|e| Error::Other(format!("read HEAD: {}", e)))?;
    let head_line = head_content.trim();
    let branch = if let Some(rest) = head_line.strip_prefix("ref: ") {
        let name = rest.strip_prefix("refs/heads/").unwrap_or(rest).to_string();
        BranchRef::Named(name)
    } else {
        let sha = head_line.chars().take(7).collect::<String>();
        BranchRef::Detached(sha)
    };

    Ok(RepoInfo { root, branch })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn branch_ref_display() {
        assert_eq!(format!("{}", BranchRef::Named("main".into())), "main");
        assert!(format!("{}", BranchRef::Detached("abc123".into())).starts_with("DETACHED@"));
    }
}
