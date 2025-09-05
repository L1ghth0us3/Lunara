use std::env;

/// Exit codes used by the CLI
const EXIT_OK: i32 = 0;
const EXIT_USAGE: i32 = 2;
const EXIT_CHECK_FAILED: i32 = 10;
const EXIT_ERROR: i32 = 1;

fn print_usage() {
    let version = lunara_core::VERSION;
    eprintln!(
        "Lunara v{}\n\nUsage:\n  lunara <command> [options]\n\nCommands:\n  help                 Show this help message\n  version              Print version information\n  init                 Initialize a Lunara config in this repo\n  status               Show current repo status and gates\n  check                Run configured checks (lint/format/test)\n  run                  Execute the configured pipeline\n\nNotes:\n  - Exit codes: 0=ok, 1=error, 2=usage, 10=checks failed",
        version
    );
}

fn cmd_version() -> i32 {
    println!("{}", lunara_core::VERSION);
    EXIT_OK
}

fn cmd_init() -> i32 {
    // Placeholder: in v0.1, we'll scaffold a minimal lunara.yml
    eprintln!("init: not implemented yet (LUN-5)");
    EXIT_ERROR
}

fn cmd_status() -> i32 {
    // Attempt to load config and report status
    match lunara_core::config::load() {
        Ok(cfg) => {
            println!(
                "Lunara v{} — config loaded (v{})",
                lunara_core::VERSION, cfg.version
            );
            EXIT_OK
        }
        Err(e) => {
            eprintln!("Lunara v{} — config not loaded: {}", lunara_core::VERSION, e);
            EXIT_ERROR
        }
    }
}

fn cmd_check() -> i32 {
    // Placeholder: will dispatch to language plugins
    eprintln!("check: not implemented yet (LUN-11/LUN-12/LUN-13)");
    EXIT_CHECK_FAILED
}

fn cmd_run() -> i32 {
    // Placeholder: will execute configured pipeline with policies
    eprintln!("run: not implemented yet");
    EXIT_ERROR
}

fn parse_and_run<I>(mut args: I) -> i32
where
    I: Iterator<Item = String>,
{
    // Skip program name
    let _ = args.next();

    match args.next().as_deref() {
        Some("help") | None => {
            print_usage();
            EXIT_OK
        }
        Some("version") => cmd_version(),
        Some("init") => cmd_init(),
        Some("status") => cmd_status(),
        Some("check") => cmd_check(),
        Some("run") => cmd_run(),
        Some(_) => {
            print_usage();
            EXIT_USAGE
        }
    }
}

fn main() {
    let code = parse_and_run(env::args());
    std::process::exit(code);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn call(args: &[&str]) -> i32 {
        let v = args.iter().map(|s| s.to_string());
        parse_and_run(v)
    }

    #[test]
    fn usage_on_no_args() {
        assert_eq!(call(&["lunara"]), EXIT_OK);
    }

    #[test]
    fn version_ok() {
        assert_eq!(call(&["lunara", "version"]), EXIT_OK);
    }

    #[test]
    fn unknown_is_usage() {
        assert_eq!(call(&["lunara", "wat"]), EXIT_USAGE);
    }
}
