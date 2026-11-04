//! RustSpeak — a Rust-native voice client inspired by TeamSpeak 3.
//!
//! Modeled on TeamSpeak 3 Client 3.5.6 (`ts3client_win64.exe`,
//! PE32+ x86_64, plugin API v25). Full GUI clone in Rust (egui):
//! menu bar, channel tree, chat, PTT, ServerQuery connect.

mod audio;
mod branding;
mod cli;
mod config;
mod gui;
mod model;
mod protocol;
mod teamspeak;

use model::{Channel, ChannelType, Client, Session};
use std::io::{BufRead, Write};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match cli::parse_args(&args) {
        cli::Command::Help => {
            println!("{}", cli::USAGE);
        }
        cli::Command::Version => {
            println!("{}", cli::version_string());
        }
        cli::Command::Connect { server, nickname } => {
            run_connect(&server, &nickname);
        }
        cli::Command::TestConnection { host } => {
            run_test_connection(&host);
        }
        cli::Command::Demo => {
            run_demo();
        }
        cli::Command::Gui => {
            if let Err(e) = gui::run() {
                eprintln!("GUI failed: {}", e);
                std::process::exit(1);
            }
        }
    }
}

fn run_demo() {
    println!("{} {}\n", branding::APP_NAME, branding::APP_VERSION);
    println!("Based on TeamSpeak 3 Client {} concepts (plugin API v{}), rebuilt in Rust.", branding::TS3_REFERENCE_VERSION, branding::PLUGIN_API_VERSION);
    println!("Run `rustspeak connect <SERVER> [NICKNAME]` to join a RustSpeak server.");
    println!("Run `rustspeak --help` for options.\n");

    let mut session = demo_session();
    session.connection = model::ConnectionState::Connected;
    println!("Connected (demo)\n");
    println!("{}", session.render_channel_tree());
    println!("Type `/help` for commands, `/quit` to exit.");
    println!("(Demo chat: say hello)\n");

    let stdin = std::io::stdin();
    let mut buf = String::new();
    loop {
        print!("> ");
        let _ = std::io::stdout().flush();
        buf.clear();
        match stdin.lock().read_line(&mut buf) {
            Ok(0) => break,
            Ok(_) => {}
            Err(_) => break,
        }
        let line = buf.trim();
        match line {
            "" => continue,
            "/quit" | "/exit" => {
                println!("{} {} — goodbye.", branding::APP_NAME, branding::APP_VERSION);
                break;
            }
            "/help" => println!("Commands: /who, /channels, /mute, /deaf, /quit"),
            "/who" => {
                for c in session.clients.values() {
                    println!("  {} (channel {})", c.nickname, c.channel_id);
                }
            }
            "/channels" => println!("{}", session.render_channel_tree()),
            "/mute" => println!("mute toggled (demo)"),
            "/deaf" => println!("deaf toggled (demo)"),
            "/version" => println!("{}", cli::version_string()),
            _ => {
                println!("<you> {}", line);
            }
        }
    }
}

fn run_connect(server: &str, nickname: &str) {
    println!("{} {} — connecting to {} as {} …", branding::APP_NAME, branding::APP_VERSION, server, nickname);
    println!("Handshake packet:");
    println!("  {}", protocol::handshake(nickname).encode().trim_end());
    println!("(Server greeting / channel list arrive from the RustSpeak server. This build ships without a live voice backend yet — NullBackend keeps the session wired for cpal + opus.)\n");
    let mut session = demo_session();
    session.server.address = server.to_string();
    session.self_client = Some(Client::new(1, 1, nickname.to_string()));
    session.clients.insert(1, Client::new(1, 1, nickname.to_string()));
    println!("{}", session.render_channel_tree());
}

fn run_test_connection(host: &str) {
    println!("{} {} — testing connection to {} (ServerQuery 10011) …", branding::APP_NAME, branding::APP_VERSION, host);
    match teamspeak::run_test_connection(host, 10011) {
        Ok(r) => {
            println!("Greeting: {}", r.greeting);
            println!("Help lines: {}", r.commands.len());
            for l in r.commands.iter().take(5) {
                println!("  {}", l);
            }
            println!("serverinfo:");
            for l in r.server_info_lines.iter().take(5) {
                println!("  {}", l);
            }
            println!("Channels: {}, Clients: {}", r.channel_count, r.client_count);
            if r.ok {
                println!("OK — TeamSpeak ServerQuery reachable, features verified.");
            } else {
                println!("UNEXPECTED — greeting missing TS3 marker.");
            }
        }
        Err(e) => {
            eprintln!("FAILED: {} ({})", e, host);
            std::process::exit(1);
        }
    }
}

fn demo_session() -> Session {
    Session::new()
        .add_channel(Channel {
            id: 1, parent_id: 0, name: "Lobby".into(), topic: "Welcome to RustSpeak".into(),
            description: String::new(), channel_type: ChannelType::Normal,
            max_clients: -1, max_family_clients: -1, order: 0, is_recording: false,
        })
        .add_channel(Channel {
            id: 2, parent_id: 1, name: "Rustaceans".into(), topic: String::new(),
            description: String::new(), channel_type: ChannelType::Normal,
            max_clients: -1, max_family_clients: -1, order: 0, is_recording: false,
        })
}
