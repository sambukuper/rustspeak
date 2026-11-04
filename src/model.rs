//! Core domain model — the subset of TeamSpeak 3 concepts RustSpeak implements.
//!
//! Channel types, client flags, and chat closely follow the TS3 data model
//! so channel trees and permissions stay familiar, while RustSpeak adds its
//! own server greeting and version fields.

use std::collections::HashMap;

/// TeamSpeak-style channel type bitflags (subset, matching TS3 semantics).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum ChannelType {
    Normal = 0,
    MaxClients = 1,
    MaxFamilyClients = 2,
    Temporary = 4,
    SemiPermanent = 8,
    Permanent = 16,
    ChannelAdmin = 32,
}

impl ChannelType {
    pub fn from_bits(v: u32) -> Option<Self> {
        match v {
            0 => Some(ChannelType::Normal),
            1 => Some(ChannelType::MaxClients),
            2 => Some(ChannelType::MaxFamilyClients),
            4 => Some(ChannelType::Temporary),
            8 => Some(ChannelType::SemiPermanent),
            16 => Some(ChannelType::Permanent),
            32 => Some(ChannelType::ChannelAdmin),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Channel {
    pub id: u64,
    pub parent_id: u64,
    pub name: String,
    pub topic: String,
    pub description: String,
    pub channel_type: ChannelType,
    pub max_clients: i32,
    pub max_family_clients: i32,
    pub order: i32,
    pub is_recording: bool,
}

impl Channel {
    pub fn root() -> Channel {
        Channel {
            id: 0,
            parent_id: 0,
            name: "Server Root".to_string(),
            topic: String::new(),
            description: String::new(),
            channel_type: ChannelType::Normal,
            max_clients: -1,
            max_family_clients: -1,
            order: 0,
            is_recording: false,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u32)]
pub enum ConnectionState {
    #[default]
    Disconnected = 0,
    Connecting = 1,
    Connected = 2,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Client {
    pub id: u64,
    pub channel_id: u64,
    pub nickname: String,
    pub is_muted: bool,
    pub is_deaf: bool,
    pub is_talking: bool,
    pub is_away: bool,
    pub talk_level: u8,
    pub unique_id: String,
}

impl Client {
    pub fn new(id: u64, channel_id: u64, nickname: impl Into<String>) -> Self {
        Client {
            id,
            channel_id,
            nickname: nickname.into(),
            is_muted: false,
            is_deaf: false,
            is_talking: false,
            is_away: false,
            talk_level: 0,
            unique_id: format!("rustspeak-{id:016}"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ChatMessage {
    pub sender_id: u64,
    pub sender_nick: String,
    pub target: ChatTarget,
    pub text: String,
    pub timestamp: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ChatTarget {
    Channel(u64),
    Server,
    User(u64),
}

#[derive(Clone, Debug)]
pub struct ServerInfo {
    pub address: String,
    pub virtual_server_id: u16,
    pub name: String,
    pub version: String,
    pub max_clients: usize,
}

impl Default for ServerInfo {
    fn default() -> Self {
        ServerInfo {
            address: "localhost:9987".to_string(),
            virtual_server_id: 1,
            name: "RustSpeak Server".to_string(),
            version: "0.1.0".to_string(),
            max_clients: 0,
        }
    }
}

/// In-memory session state — channels, clients, and chat log.
#[derive(Debug, Default)]
pub struct Session {
    pub connection: ConnectionState,
    pub self_client: Option<Client>,
    pub channels: HashMap<u64, Channel>,
    pub clients: HashMap<u64, Client>,
    pub chat_log: Vec<ChatMessage>,
    pub server: ServerInfo,
}

impl Session {
    pub fn new() -> Session {
        let mut s = Session::default();
        s.channels.insert(0, Channel::root());
        s
    }

    pub fn add_channel(mut self, c: Channel) -> Self {
        self.channels.insert(c.id, c);
        self
    }

    pub fn add_client(&mut self, c: Client) {
        self.clients.insert(c.id, c);
    }

    pub fn clients_in_channel(&self, channel_id: u64) -> Vec<&Client> {
        self.clients
            .values()
            .filter(|c| c.channel_id == channel_id)
            .collect()
    }

    pub fn push_chat(&mut self, m: ChatMessage) {
        self.chat_log.push(m);
    }

    /// Render the channel tree as it would appear in the client UI.
    pub fn render_channel_tree(&self) -> String {
        fn walk(
            session: &Session,
            parent_id: u64,
            depth: usize,
            out: &mut String,
        ) {
            let mut children: Vec<&Channel> = session
                .channels
                .values()
                .filter(|c| c.parent_id == parent_id && c.id != parent_id)
                .collect();
            children.sort_by_key(|c| (c.order, c.id));
            for c in children {
                let indent = "  ".repeat(depth);
                out.push_str(&format!("{}{} [{}]\n", indent, c.name, c.id));
                for client in session.clients_in_channel(c.id) {
                    let status = if client.is_talking {
                        "speaking"
                    } else if client.is_muted {
                        "muted"
                    } else if client.is_deaf {
                        "deaf"
                    } else {
                        "idle"
                    };
                    out.push_str(&format!(
                        "{}  - {} ({})   [{}]\n",
                        indent, client.nickname, status, client.id
                    ));
                }
                walk(session, c.id, depth + 1, out);
            }
        }
        let mut out = String::new();
        out.push_str(&format!("{}\n", self.server.name));
        walk(self, 0, 0, &mut out);
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn root_channel_exists() {
        let s = Session::new();
        assert!(s.channels.contains_key(&0));
    }
    #[test]
    fn channel_tree_orders_by_order_then_id() {
        let s = Session::new().add_channel(Channel {
            id: 2, parent_id: 0, name: "B".into(), topic: String::new(),
            description: String::new(), channel_type: ChannelType::Normal,
            max_clients: -1, max_family_clients: -1, order: 1, is_recording: false,
        }).add_channel(Channel {
            id: 1, parent_id: 0, name: "A".into(), topic: String::new(),
            description: String::new(), channel_type: ChannelType::Normal,
            max_clients: -1, max_family_clients: -1, order: 0, is_recording: false,
        });
        let tree = s.render_channel_tree();
        assert!(tree.find('A').unwrap() < tree.find('B').unwrap());
    }
}
