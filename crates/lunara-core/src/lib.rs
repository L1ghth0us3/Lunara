//! Lunara core library

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn banner() -> String {
    format!("Lunara core ready (v{})", VERSION)
}

pub mod config;
