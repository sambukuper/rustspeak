//! RustSpeak branding — mapped from TeamSpeak 3 Client 3.5.6 evidence.
//!
//! Source of truth for names, versions, and plugin API level.
//! See `docs/MAPPING.md` for the full TS3 → RustSpeak table.

/// Public product name. Replaces `TeamSpeak 3`.
pub const APP_NAME: &str = "RustSpeak";
/// Short name shown in window title, taskbar, and `--version`.
pub const APP_SHORT: &str = "RustSpeak";
/// Executable name (Windows `.exe` appended by the OS).
pub const EXE_NAME: &str = "rustspeak";
/// RustSpeak release version. TS3 shipped 3.5.6 (2020-11-25); RustSpeak starts at 0.1.0.
pub const APP_VERSION: &str = "0.1.0";
/// Version inherited from the analysis target for compatibility notes.
pub const TS3_REFERENCE_VERSION: &str = "3.5.6";
/// Plugin API level carried over from TS3 3.5.6 (changelog: "Updated Plugin API version to 25").
pub const PLUGIN_API_VERSION: u32 = 25;
/// Default user-agent / handshake identifier sent to servers.
pub const USER_AGENT: &str = concat!("RustSpeak/", env!("CARGO_PKG_VERSION"));
/// The TS3 binary this client was modeled on (PE32+ x86_64, 17_911_768 bytes).
pub const TS3_REFERENCE_EXE: &str = "ts3client_win64.exe";
/// PE resource the icon group was bound to in the original binary.
/// RustSpeak binds its own `IDI_ICON1` group in the same layout.
pub const TS3_ICON_GROUP: &str = "IDI_ICON1";

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn api_level_is_25_from_ts3_3_5_6() {
        assert_eq!(PLUGIN_API_VERSION, 25);
    }
    #[test]
    fn name_is_rustspeak_not_teamspeak() {
        assert_eq!(APP_NAME, "RustSpeak");
    }
    #[test]
    fn user_agent_matches_version() {
        assert!(USER_AGENT.starts_with("RustSpeak/"));
    }
}
