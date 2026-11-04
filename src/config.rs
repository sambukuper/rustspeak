//! Config file (TOML subset, zero-dependency parser).
//!
//! Layout mirrors the TS3 config directory idea (`rustspeak.toml` next to
//! the binary, same as TS3's lockable `settings`). The parser below only
//! needs `key = value` plus `[section]` headers, so no TOML crate is
//! required and `cargo build` stays offline.

use std::path::Path;

#[derive(Debug, Clone)]
pub struct Config {
    pub nickname: String,
    pub server: String,
    pub ptt_key: String,
    pub channel: String,
    pub plugin_api: u32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            nickname: "Player".to_string(),
            server: "localhost:9987".to_string(),
            ptt_key: "RightShift".to_string(),
            channel: "Lobby".to_string(),
            plugin_api: crate::branding::PLUGIN_API_VERSION,
        }
    }
}

impl Config {
    pub fn load(path: &Path) -> Result<Config, String> {
        let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {}", path.display(), e))?;
        Config::parse(&text)
    }

    pub fn parse(text: &str) -> Result<Config, String> {
        let mut cfg = Config::default();
        for raw in text.lines() {
            let line = raw.trim();
            if line.is_empty() || line.starts_with('#') || (line.starts_with('[') && line.ends_with(']')) {
                continue;
            }
            let (k, v) = line.split_once('=').ok_or_else(|| format!("bad line: {}", line))?;
            let v = v.trim().trim_matches('"').trim_matches('\'');
            match k.trim() {
                "nickname" => cfg.nickname = v.to_string(),
                "server" => cfg.server = v.to_string(),
                "ptt_key" => cfg.ptt_key = v.to_string(),
                "channel" => cfg.channel = v.to_string(),
                "plugin_api" => cfg.plugin_api = v.parse().map_err(|_| "bad plugin_api")?,
                _ => {}
            }
        }
        Ok(cfg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_basic() {
        let c = Config::parse("nickname = \"Bob\"\nserver = \"s:9987\"\nplugin_api = 25\n").unwrap();
        assert_eq!(c.nickname, "Bob");
        assert_eq!(c.plugin_api, 25);
    }
}
