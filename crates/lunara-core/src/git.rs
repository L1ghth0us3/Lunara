use std::fmt;
use std::path::PathBuf;
use std::process::Command;

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

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Upstream {
    pub name: Option<String>,
    pub ahead: u32,
    pub behind: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct StatusCounts {
    pub staged: u32,
    pub unstaged: u32,
    pub untracked: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct StatusSummary {
    pub upstream: Upstream,
    pub counts: StatusCounts,
}

fn parse_porcelain_v1_b(input: &str) -> StatusSummary {
    let mut upstream = Upstream::default();
    let mut counts = StatusCounts::default();

    for line in input.lines() {
        if let Some(rest) = line.strip_prefix("## ") {
            // Example: "main...origin/main [ahead 1, behind 2]" or "HEAD (no branch)"
            let mut name = None;
            let mut ahead = 0u32;
            let mut behind = 0u32;
            let mut head = rest;
            if let Some((left, _right)) = rest.split_once(" [") {
                head = left;
                if let Some(brack) = rest.split_once('[').map(|(_, r)| r.trim_end_matches(']')) {
                    for part in brack.split(',') {
                        let part = part.trim();
                        if let Some(n) = part.strip_prefix("ahead ") {
                            ahead = n.parse().unwrap_or(0);
                        } else if let Some(n) = part.strip_prefix("behind ") {
                            behind = n.parse().unwrap_or(0);
                        }
                    }
                }
            }
            if let Some((_local, remote)) = head.split_once("...") {
                name = Some(remote.trim().to_string());
            }
            upstream = Upstream { name, ahead, behind };
            continue;
        }

        // Porcelain v1 XY status, untracked starts with ??
        if line.starts_with("?? ") {
            counts.untracked += 1;
            continue;
        }
        let bytes = line.as_bytes();
        if bytes.len() >= 3 {
            let x = bytes[0] as char;
            let y = bytes[1] as char;
            if x != ' ' { counts.staged += 1; }
            if y != ' ' { counts.unstaged += 1; }
        }
    }

    StatusSummary { upstream, counts }
}

pub fn status_summary(repo_root: &PathBuf) -> Result<StatusSummary, Error> {
    let out = Command::new("git")
        .arg("status")
        .arg("--porcelain=v1")
        .arg("-b")
        .current_dir(repo_root)
        .output()
        .map_err(|e| Error::Other(format!("git status: {}", e)))?;
    if !out.status.success() {
        return Err(Error::Other(format!(
            "git status failed: {}",
            String::from_utf8_lossy(&out.stderr)
        )));
    }
    let text = String::from_utf8_lossy(&out.stdout);
    let mut summary = parse_porcelain_v1_b(&text);
    // If we have an upstream, compute accurate ahead/behind using rev-list
    if summary.upstream.name.is_some() {
        if let Ok((ahead, behind)) = ahead_behind(repo_root) {
            summary.upstream.ahead = ahead;
            summary.upstream.behind = behind;
        }
    }
    Ok(summary)
}

pub fn ahead_behind(repo_root: &PathBuf) -> Result<(u32, u32), Error> {
    let out = Command::new("git")
        .arg("rev-list")
        .arg("--left-right")
        .arg("--count")
        .arg("HEAD...@{u}")
        .current_dir(repo_root)
        .output()
        .map_err(|e| Error::Other(format!("git rev-list: {}", e)))?;
    if !out.status.success() {
        return Err(Error::Other(format!(
            "git rev-list failed: {}",
            String::from_utf8_lossy(&out.stderr)
        )));
    }
    let s = String::from_utf8_lossy(&out.stdout);
    // Output like: "A\tB\n"
    let mut it = s.trim().split(|c| c == '\t' || c == ' ');
    let ahead = it.next().and_then(|x| x.parse::<u32>().ok()).unwrap_or(0);
    let behind = it.next().and_then(|x| x.parse::<u32>().ok()).unwrap_or(0);
    Ok((ahead, behind))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn branch_ref_display() {
        assert_eq!(format!("{}", BranchRef::Named("main".into())), "main");
        assert!(format!("{}", BranchRef::Detached("abc123".into())).starts_with("DETACHED@"));
    }

    #[test]
    fn parse_porcelain_header_and_counts() {
        let sample = "## main...origin/main [ahead 2, behind 1]\n M src/lib.rs\nM  src/main.rs\n?? new.txt\n";
        let s = parse_porcelain_v1_b(sample);
        assert_eq!(s.upstream.name.as_deref(), Some("origin/main"));
        assert_eq!(s.upstream.ahead, 2);
        assert_eq!(s.upstream.behind, 1);
        assert_eq!(s.counts.untracked, 1);
        // One staged (x!=space), one unstaged (y!=space)
        assert_eq!(s.counts.staged, 1);
        assert_eq!(s.counts.unstaged, 1);
    }
}
