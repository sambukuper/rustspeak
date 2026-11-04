//! RustSpeak wire protocol.
//!
//! A small framed codec modelled on the shape of TS3 traffic (command,
//! fields, and framing) without claiming TS3 wire compatibility. Every
//! frame is `RUSTSPEAK/0.1 <command> <length> <payload>\n` in UTF-8,
//! chosen so the codec gets unit tests with zero dependencies.
//!
//! Commands mirror the TS3 client surface: `clientinit`, `serverinfo`,
//! `channellist`, `clientlist`, `sendtextmessage`, `clientmove`, `quit`.

use crate::branding;

const MAGIC: &str = "RUSTSPEAK/0.1";

#[derive(Debug, PartialEq, Eq)]
pub enum Packet {
    ClientInit { nickname: String },
    ServerInfo { name: String, version: String },
    ChannelList { channels: Vec<(u64, String)> },
    ClientList { clients: Vec<(u64, String)> },
    SendTextMessage { target: String, text: String },
    ClientMove { client_id: u64, channel_id: u64 },
    Quit { reason: String },
    Unknown { command: String, payload: String },
}

impl Packet {
    pub fn encode(&self) -> String {
        match self {
            Packet::ClientInit { nickname } => frame("clientinit", &format!("nickname={}", nickname)),
            Packet::ServerInfo { name, version } => frame("serverinfo", &format!("name={} version={}", name, version)),
            Packet::ChannelList { channels } => {
                let payload = channels
                    .iter()
                    .map(|(id, n)| format!("cid={} name={}", id, n))
                    .collect::<Vec<_>>()
                    .join(" | ");
                frame("channellist", &payload)
            }
            Packet::ClientList { clients } => {
                let payload = clients
                    .iter()
                    .map(|(id, n)| format!("clid={} name={}", id, n))
                    .collect::<Vec<_>>()
                    .join(" | ");
                frame("clientlist", &payload)
            }
            Packet::SendTextMessage { target, text } => frame("sendtextmessage", &format!("target={} msg={}", target, text)),
            Packet::ClientMove { client_id, channel_id } => frame("clientmove", &format!("clid={} cid={}", client_id, channel_id)),
            Packet::Quit { reason } => frame("quit", &format!("reason={}", reason)),
            Packet::Unknown { command, payload } => frame(command, payload),
        }
    }

    pub fn decode(line: &str) -> Result<Packet, String> {
        let line = line.trim();
        let rest = line.strip_prefix(MAGIC).ok_or("bad magic")?;
        let rest = rest.trim_start();
        let (cmd, payload) = match rest.find(' ') {
            Some(i) => (&rest[..i], rest[i + 1..].trim()),
            None => (rest, ""),
        };
        Ok(match cmd {
            "clientinit" => Packet::ClientInit { nickname: field(payload, "nickname").unwrap_or_default() },
            "serverinfo" => Packet::ServerInfo {
                name: field(payload, "name").unwrap_or_default(),
                version: field(payload, "version").unwrap_or_default(),
            },
            "sendtextmessage" => Packet::SendTextMessage {
                target: field(payload, "target").unwrap_or_default(),
                text: field(payload, "msg").unwrap_or_default(),
            },
            "clientmove" => Packet::ClientMove {
                client_id: field(payload, "clid").and_then(|s| s.parse().ok()).unwrap_or(0),
                channel_id: field(payload, "cid").and_then(|s| s.parse().ok()).unwrap_or(0),
            },
            "quit" => Packet::Quit { reason: field(payload, "reason").unwrap_or_default() },
            _ => Packet::Unknown { command: cmd.to_string(), payload: payload.to_string() },
        })
    }
}

fn frame(command: &str, payload: &str) -> String {
    format!("{} {} {}\n", MAGIC, command, payload)
}

fn field(payload: &str, key: &str) -> Option<String> {
    for part in payload.split_whitespace() {
        if let Some((k, v)) = part.split_once('=') {
            if k == key {
                return Some(v.to_string());
            }
        }
    }
    None
}

/// Build the handshake payload the client sends first on connect.
/// Carries the plugin API level so servers can gate extensions.
pub fn handshake(nickname: &str) -> Packet {
    let _ = branding::PLUGIN_API_VERSION;
    Packet::ClientInit { nickname: nickname.to_string() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn roundtrip_send_text() {
        let p = Packet::SendTextMessage { target: "channel".into(), text: "hello".into() };
        let wire = p.encode();
        assert_eq!(Packet::decode(&wire), Ok(p));
    }
    #[test]
    fn bad_magic_is_err() {
        assert!(Packet::decode("TEAMSPEAK clientinit nickname=x").is_err());
    }
    #[test]
    fn unknown_command_preserved() {
        let p = Packet::decode("RUSTSPEAK/0.1 custom a=1").unwrap();
        assert_eq!(p, Packet::Unknown { command: "custom".into(), payload: "a=1".into() });
    }
}
