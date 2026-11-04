# RustSpeak

A Rust-native voice client inspired by TeamSpeak 3.

RustSpeak began with the installed TeamSpeak 3 Client (`ts3client_win64.exe`,
TeamSpeak 3 Client 3.5.6, 2020-11-25, PE32+ x86_64) and rebuilt the client
surface from scratch in Rust. Same ideas (servers, channels, chat,
push-to-talk, plugin API), new implementation, new name.

## Status

`0.2.0` with a full GUI client in Rust (egui). Launching `rustspeak` opens
the window: menu bar, toolbar, channel tree with talking indicators, chat,
PTT and mute controls, status bar. Connect to a server from the GUI or
from the CLI with `rustspeak connect`. The console demo stays available
with `--no-gui`. `rustspeak test-connection` verifies a TeamSpeak server
over ServerQuery (port 10011).

## Build

```sh
cargo build
cargo test
cargo run -- --version
cargo run -- connect play.rustspeak.gg Alice
```

## Layout

```
src/
  main.rs       entry point (GUI default, demo session)
  cli.rs        argument parsing, --help / --version
  gui.rs        egui window: menus, channel tree, chat, PTT, status bar
  teamspeak.rs  ServerQuery test connection (TCP, std only)
  branding.rs   RustSpeak name, version, plugin API v25 (from TS3 3.5.6)
  model.rs      channels, clients, chat, channel tree rendering
  protocol.rs   RustSpeak wire codec (framed commands)
  audio.rs      PTT / VAD state machine behind AudioBackend trait
  config.rs     rustspeak.toml loader (zero-dep parser)
```

## How it was derived

The REA and Craft Anything investigation pinned the reference binary before
any code was written. See `docs/MAPPING.md`.

* PE resources showed an `IDI_ICON1` group with 6 icon images and version info
  in `ts3client_win64.exe`. RustSpeak binds its own icon group in the same layout.
* The changelog pinned version 3.5.6 and Plugin API v25. Both are carried into `branding.rs`.
* TS3's 290-file install (Qt5, `gfx/`, `html/`, `sound/`) served as reference, not source. RustSpeak is a native binary, not a Qt patch.

## Next

* cpal plus opus `AudioBackend` implementation for voice in and out
* Server directory and friends
* Plugin API v25 host built on `branding.rs` and the `model.rs` tree
