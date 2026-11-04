# RustSpeak

A Rust-native voice client inspired by TeamSpeak 3.

RustSpeak began with the installed TeamSpeak 3 Client (`ts3client_win64.exe`,
TeamSpeak 3 Client 3.5.6, 2020-11-25, PE32+ x86_64) and rebuilt the client
surface from scratch in Rust. Same ideas (servers, channels, chat,
push-to-talk, plugin API), new implementation, new name.

## Status

`0.1.0` with a CLI demo. Connect flow, channel tree, plugin API v25
handshake. Zero runtime dependencies, so `cargo build` works offline.

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
  main.rs       CLI entry + demo session
  cli.rs        argument parsing, --help / --version
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
* Tauri or wry UI over the channel tree in `model.rs`
