//! Command-line interface — a TS3-style client surface in Rust.

use crate::branding;

pub const USAGE: &str = "RustSpeak 0.1.0 — a Rust-native voice client (TeamSpeak 3 inspired)\n\
\n\
Usage:\n\
  rustspeak [OPTIONS]\n\
  rustspeak connect <SERVER> [NICKNAME]\n\
  rustspeak test-connection [HOST]\n\
  rustspeak --version\n\
\nOptions:\n\
  --config <PATH>   Load rustspeak.toml (default: ./rustspeak.toml if present)\n\
  -h, --help        Show this help\n\
  -V, --version     Show RustSpeak version\n\
\nExamples:\n\
  rustspeak\n\
  rustspeak connect play.rustspeak.gg Alice\n\
  rustspeak test-connection you.are.bot\n";

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Demo,
    Connect { server: String, nickname: String },
    TestConnection { host: String },
    Version,
    Help,
}

pub fn parse_args(args: &[String]) -> Command {
    let mut iter = args.iter().skip(1).peekable();
    match iter.peek().map(|s| s.as_str()) {
        Some("-h") | Some("--help") | Some("help") => Command::Help,
        Some("-V") | Some("--version") => Command::Version,
        Some("connect") => {
            iter.next();
            let server = iter.next().map(|s| s.clone()).unwrap_or_else(|| "localhost:9987".to_string());
            let nickname = iter.next().map(|s| s.clone()).unwrap_or_else(|| {
                std::env::var("RUSTSPEAK_NICK").unwrap_or_else(|_| "Player".to_string())
            });
            Command::Connect { server, nickname }
        }
        Some("test-connection") => {
            iter.next();
            let host = iter.next().map(|s| s.clone()).unwrap_or_else(|| "you.are.bot".to_string());
            Command::TestConnection { host }
        }
        Some("--config") => {
            iter.next();
            let _ = iter.next();
            Command::Demo
        }
        _ => Command::Demo,
    }
}

pub fn version_string() -> String {
    format!("{} {} ({}; plugin API v{})", branding::APP_NAME, branding::APP_VERSION, branding::TS3_REFERENCE_VERSION, branding::PLUGIN_API_VERSION)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn v(s: &[&str]) -> Vec<String> { s.iter().map(|x| x.to_string()).collect() }
    #[test]
    fn connect_parses() {
        assert_eq!(
            parse_args(&v(&["rustspeak", "connect", "s:1", "Bob"])),
            Command::Connect { server: "s:1".into(), nickname: "Bob".into() }
        );
    }
}
