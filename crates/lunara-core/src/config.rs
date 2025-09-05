use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum Error {
    NotFound,
    Io(std::io::Error),
    Parse(serde_yaml::Error),
    Invalid(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NotFound => write!(f, "no lunara.yml(.yaml) found"),
            Error::Io(e) => write!(f, "io error: {}", e),
            Error::Parse(e) => write!(f, "yaml parse error: {}", e),
            Error::Invalid(msg) => write!(f, "invalid config: {}", msg),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            Error::Parse(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self { Error::Io(e) }
}

impl From<serde_yaml::Error> for Error {
    fn from(e: serde_yaml::Error) -> Self { Error::Parse(e) }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Config {
    #[serde(default = "default_version")] 
    pub version: String,
    pub pipeline: Pipeline,
    pub policies: Option<Policies>,
    #[serde(default)]
    pub intent: Intent,
}

fn default_version() -> String { "1".to_string() }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Pipeline {
    pub languages: Vec<Language>,
    pub gates: Vec<Gate>,
    #[serde(default)]
    pub timeouts: Option<Timeouts>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    Rust,
    Js,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Gate {
    Build,
    Lint,
    Test,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Intent {
    #[serde(default = "default_intent_path")] 
    pub path: String,
}

fn default_intent_path() -> String { ".lunara/intent.json".to_string() }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Policies {
    #[serde(default)]
    pub docs_required: bool,
    #[serde(default)]
    pub protected_branches: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Timeouts {
    pub per_step_secs: Option<u64>,
    pub overall_secs: Option<u64>,
}

impl Config {
    pub fn validate(&self) -> Result<(), Error> {
        if self.pipeline.languages.is_empty() {
            return Err(Error::Invalid("pipeline.languages must not be empty".into()));
        }
        if self.pipeline.gates.is_empty() {
            return Err(Error::Invalid("pipeline.gates must not be empty".into()));
        }
        Ok(())
    }
}

pub fn parse_str(yaml: &str) -> Result<Config, Error> {
    let cfg: Config = serde_yaml::from_str(yaml)?;
    cfg.validate()?;
    Ok(cfg)
}

pub fn load_from(path: &Path) -> Result<Config, Error> {
    let text = fs::read_to_string(path)?;
    parse_str(&text)
}

pub fn discover_path(start: &Path) -> Option<PathBuf> {
    for name in &["lunara.yml", "lunara.yaml"] {
        let p = start.join(name);
        if p.exists() { return Some(p); }
    }
    None
}

pub fn load() -> Result<Config, Error> {
    let here = std::env::current_dir().map_err(Error::Io)?;
    let Some(path) = discover_path(&here) else { return Err(Error::NotFound) };
    load_from(&path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_minimal_ok() {
        let y = r#"
version: "1"
pipeline:
  languages: [rust]
  gates: [build, lint, test]
policies:
  docs_required: false
  protected_branches: ["main"]
intent:
  path: ".lunara/intent.json"
"#;
        let cfg = parse_str(y).expect("parse ok");
        assert_eq!(cfg.version, "1");
        assert_eq!(cfg.pipeline.languages, vec![Language::Rust]);
        assert_eq!(cfg.pipeline.gates, vec![Gate::Build, Gate::Lint, Gate::Test]);
        assert_eq!(cfg.intent.path, ".lunara/intent.json");
    }

    #[test]
    fn invalid_empty_languages() {
        let y = r#"
version: "1"
pipeline:
  languages: []
  gates: [build]
"#;
        let err = parse_str(y).unwrap_err();
        match err { Error::Invalid(msg) => assert!(msg.contains("languages")), _ => panic!("wrong err") }
    }

    #[test]
    fn invalid_unknown_enum() {
        let y = r#"
version: "1"
pipeline:
  languages: [python]
  gates: [build]
"#;
        let err = parse_str(y).unwrap_err();
        match err { Error::Parse(_) => {}, _ => panic!("expected parse error") }
    }
}

