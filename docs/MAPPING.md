# TS3 → RustSpeak mapping

Evidence-first: every row comes from REA / Craft Anything or the install
itself, not from a filename guess.

## Reference install

| Feature | TeamSpeak 3 Client 3.5.6 | RustSpeak |
| --- | --- | --- |
| Exe | `ts3client_win64.exe` (17,911,768 bytes, PE32+ x86_64) | `rustspeak.exe` (Rust native, `cargo build`) |
| Version | 3.5.6 (2020-11-25, `changelog.txt`) | `0.1.0` (`Cargo.toml`, `branding::APP_VERSION`) |
| Plugin API | v25 (3.5.6 changelog) | v25 (`branding::PLUGIN_API_VERSION`) |
| Window/brand | Qt5 (`Qt5Gui.dll` etc., 290 files) | Rust CLI (`src/branding.rs`) — no Qt |
| Icon group | `IDI_ICON1`, 6 images (256→16px) in PE resources (ev from REA `inspect_pe_resources`) | Own `IDI_ICON1` layout at packaging |
| Version info | PE `RT_VERSION` | Build-script validated (`build.rs`) |
| GFX | `gfx/default.zip`, `tile_150.png`, `tile_70.png` | Replaced by UI theme (planned) |
| Intro HTML | `html/teamspeak5_intro/index.html` | `docs/` + `--help` (no WebEngine) |
| Sound | `sound/default/*.wav` (event sounds) | `audio::AudioState` + NullBackend stub |
| Config | `settings.lock`, `-localconfig`, `-configname` | `rustspeak.toml` (`src/config.rs`) |
| CLI | `-locksettings`, `-configname …` | `rustspeak connect <SERVER> [NICK]` (`src/cli.rs`) |

## What was NOT done

- No bytes copied from `ts3client_win64.exe`.
- No Qt DLLs bundled.
- Voice codec (Opus) is behind `AudioBackend`; the demo uses `NullBackend`.

## REA evidence

- `open_binary` bound `ts3client_win64.exe` (sha256 `282de0fa…`).
- `inspect_pe_resources` → 9 resources, `RT_GROUP_ICON IDI_ICON1` with
  6 candidate images, `RT_VERSION` (752 bytes).
- `inspect_pe_imports` swept the import table (output saved to tool-results).
- `search_strings` / `binary_overview` hit the Ghidra 330 s startup deadline
  (provider_timeout, retryable); branding was pinned from resources +
  changelog + install inventory instead. Increase
  `ghidra.startup_timeout_ms` via `update_analysis_settings` before retrying
  those two calls.
- Craft Anything ran in Deep mode (`deep_mode=true`) for the investigation.
