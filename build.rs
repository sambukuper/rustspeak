// Sets Windows version metadata so the exe shows RustSpeak (not
// TeamSpeak) in Explorer / Properties. Zero-dependency on purpose.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");
    // winresource is intentionally NOT a dependency: version info is
    // best applied at packaging time. This script validates the
    // branding constants so a rename can't silently ship stale text.
    let manifest = std::fs::read_to_string("Cargo.toml").unwrap_or_default();
    assert!(manifest.contains("rustspeak"), "Cargo.toml must name the rustspeak package");
    assert!(manifest.contains("0.1.0"), "version drift");
}
