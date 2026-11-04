//! RustSpeak GUI — TeamSpeak 3 layout, rebuilt in Rust with egui.
//!
//! Window: title `RustSpeak`, menu bar, toolbar, channel tree with
//! clients/talking indicators, chat log + input, status bar with PTT,
//! mic/deafen toggles, and ServerQuery test. All state lives in
//! `RustSpeakApp`; the demo session comes from `model::Session`.

use crate::branding;
use crate::model::{Channel, ChannelType, Client, ConnectionState, Session};
use std::collections::HashSet;

pub struct RustSpeakApp {
    session: Session,
    chat_input: String,
    connect_addr: String,
    nickname: String,
    status_line: String,
    log_lines: Vec<String>,
    ptt_pressed: bool,
    mic_muted: bool,
    deaf: bool,
    connected: bool,
    expanded: HashSet<u64>,
}

impl RustSpeakApp {
    pub fn new() -> Self {
        let mut session = demo_session();
        session.clients.insert(1, Client::new(1, 1, "You"));
        session.clients.insert(2, Client::new(2, 1, "Rustacean"));
        session.clients.insert(3, Client::new(3, 2, "Ferris"));
        if let Some(c) = session.clients.get_mut(&2) {
            c.is_talking = true;
        }
        RustSpeakApp {
            session,
            chat_input: String::new(),
            connect_addr: "you.are.bot".to_string(),
            nickname: "You".to_string(),
            status_line: "Disconnected. Enter a server and press Connect.".to_string(),
            log_lines: vec![
                format!("{} {} started.", branding::APP_NAME, branding::APP_VERSION),
                "Demo channels loaded (Lobby).".to_string(),
            ],
            ptt_pressed: false,
            mic_muted: false,
            deaf: false,
            connected: false,
            expanded: [0, 1].into_iter().collect(),
        }
    }

    fn log(&mut self, line: String) {
        self.log_lines.push(line);
        if self.log_lines.len() > 200 {
            self.log_lines.remove(0);
        }
    }

    fn do_connect(&mut self) {
        if self.connect_addr.trim().is_empty() {
            return;
        }
        let host = self.connect_addr.trim().to_string();
        self.log(format!("Connecting to {} as {} …", host, self.nickname));
        match crate::teamspeak::run_test_connection(&host, 10011) {
            Ok(r) => {
                self.connected = true;
                self.session.connection = ConnectionState::Connected;
                self.session.server.address = host.clone();
                self.status_line = format!("Connected to {} (greeting: {})", host, r.greeting);
                self.log(format!("ServerQuery OK: {} (channels {} · clients {})", r.greeting, r.channel_count, r.client_count));
            }
            Err(e) => {
                self.status_line = format!("Failed to reach {}: {}", host, e);
                self.log(format!("Connection failed: {}", e));
            }
        }
    }

    fn do_send(&mut self) {
        let text = self.chat_input.trim().to_string();
        if text.is_empty() {
            return;
        }
        self.session.push_chat(crate::model::ChatMessage {
            sender_id: 1,
            sender_nick: self.nickname.clone(),
            target: crate::model::ChatTarget::Channel(1),
            text: text.clone(),
            timestamp: 0,
        });
        self.log(format!("<{}> {}", self.nickname, text));
        self.chat_input.clear();
    }

    fn render_tree(&self, ui: &mut egui::Ui, parent_id: u64, depth: usize) {
        let mut children: Vec<&Channel> = self
            .session
            .channels
            .values()
            .filter(|c| c.parent_id == parent_id && c.id != parent_id)
            .collect();
        children.sort_by_key(|c| (c.order, c.id));
        for ch in children {
            let label = format!("{}{}", "  ".repeat(depth), ch.name);
            ui.collapsing(label, |ui| {
                for cl in self.session.clients_in_channel(ch.id) {
                    let mark = if cl.is_talking {
                        "🟢"
                    } else if cl.is_muted {
                        "🔇"
                    } else if cl.is_deaf {
                        "⛔"
                    } else {
                        "⚪"
                    };
                    ui.label(format!("{} {}", mark, cl.nickname));
                }
                self.render_tree(ui, ch.id, depth + 1);
            });
        }
    }
}

impl eframe::App for RustSpeakApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::containers::Panel::top("top_panel").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("Connections", |ui| {
                    if ui.button("Connect").clicked() { self.do_connect(); }
                    if ui.button("Disconnect").clicked() {
                        self.connected = false;
                        self.session.connection = ConnectionState::Disconnected;
                        self.status_line = "Disconnected.".to_string();
                        self.log("Disconnected.".to_string());
                    }
                });
                ui.menu_button("Bookmarks", |ui| {
                    if ui.button("you.are.bot:9987").clicked() {
                        self.connect_addr = "you.are.bot".to_string();
                        self.status_line = "Bookmarked you.are.bot.".to_string();
                    }
                });
                ui.menu_button("Settings", |ui| {
                    if ui.button("Toggle mute (M)").clicked() { self.mic_muted = !self.mic_muted; }
                    if ui.button("Toggle deafen (D)").clicked() { self.deaf = !self.deaf; }
                });
                ui.menu_button("Help", |ui| {
                    if ui.button("About RustSpeak").clicked() {
                        self.log(format!("{} {} — TeamSpeak 3 inspired, built in Rust.", branding::APP_NAME, branding::APP_VERSION));
                    }
                });
            });
        });

        egui::containers::Panel::bottom("status_panel").show(ui, |ui| {
            ui.horizontal(|ui| {
                let dot = if self.connected { "● online" } else { "○ offline" };
                ui.label(format!("{} · {}", dot, self.status_line));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let ptt_label = if self.ptt_pressed { "🎙 TRANSMITTING" } else { "🎙 PTT (hold)" };
                    if ui.button(ptt_label).clicked() { self.ptt_pressed = !self.ptt_pressed; }
                    let mute_label = if self.mic_muted { "🔇 Muted" } else { "🎤 Mic on" };
                    if ui.button(mute_label).clicked() { self.mic_muted = !self.mic_muted; }
                    let deaf_label = if self.deaf { "⛔ Deaf" } else { "👂 Deafen" };
                    if ui.button(deaf_label).clicked() { self.deaf = !self.deaf; }
                });
            });
        });

        egui::containers::Panel::left("channels_panel").show(ui, |ui| {
            ui.heading("Channels");
            ui.add_space(4.0);
            let name = self.session.server.name.clone();
            ui.collapsing(name, |ui| {
                self.render_tree(ui, 0, 0);
            });
            ui.add_space(8.0);
            ui.separator();
            ui.heading("Server");
            ui.horizontal(|ui| {
                ui.label("Host:");
                ui.add(egui::TextEdit::singleline(&mut self.connect_addr).hint_text("you.are.bot"));
            });
            ui.horizontal(|ui| {
                ui.label("Nick:");
                ui.add(egui::TextEdit::singleline(&mut self.nickname).hint_text("You"));
            });
            if ui.button("Connect").clicked() { self.do_connect(); }
            ui.add_space(8.0);
            ui.separator();
            ui.heading("Event log");
            egui::ScrollArea::vertical().show(ui, |ui| {
                for l in self.log_lines.iter() {
                    ui.label(l);
                }
            });
        });

        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Chat");
            egui::ScrollArea::vertical().show(ui, |ui| {
                for m in self.session.chat_log.iter() {
                    ui.horizontal(|ui| {
                        ui.label(format!("[{}]", m.sender_nick));
                        ui.label(&m.text);
                    });
                }
                if self.session.chat_log.is_empty() {
                    ui.label("No messages yet. Say hello below.");
                }
            });
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                let resp = ui.add(egui::TextEdit::singleline(&mut self.chat_input).hint_text("Type a message, press Enter to send"));
                if resp.lost_focus() && self.chat_input.len() > 0 && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    self.do_send();
                }
                if ui.button("Send").clicked() { self.do_send(); }
            });
        });
    }
}

fn demo_session() -> Session {
    Session::new().add_channel(Channel {
        id: 1,
        parent_id: 0,
        name: "Lobby".into(),
        topic: "Welcome to RustSpeak".into(),
        description: String::new(),
        channel_type: ChannelType::Normal,
        max_clients: -1,
        max_family_clients: -1,
        order: 0,
        is_recording: false,
    })
}

pub fn run() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("RustSpeak")
            .with_inner_size([1024.0, 680.0]),
        ..Default::default()
    };
    eframe::run_native(
        "RustSpeak",
        options,
        Box::new(|_| Ok(Box::<RustSpeakApp>::default())),
    )
}

impl Default for RustSpeakApp {
    fn default() -> Self {
        Self::new()
    }
}
