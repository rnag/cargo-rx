//! Project-specific constant declarations
//!

/// Base constants

/// Project binary command name
pub const BINARY_NAME: &str = "rx";
/// The directory which contains *examples* for Cargo projects
pub const EXAMPLES_FOLDER: &str = "examples";
/// Rust file extension
pub const RUST_FILE_EXT: &str = "rs";

/// Filenames

/// Name of the `Cargo.toml` file in Cargo projects
pub const CARGO_TOML: &str = "Cargo.toml";
/// Name of the `settings.toml` file for local project cache
pub const SETTINGS_TOML: &str = "settings.toml";

/// Commands

/// The `cargo` command
pub const CARGO_CMD: &str = "cargo";
/// The `fzf` (fuzzy finder) command
pub const FZF_CMD: &str = "fzf";

/// Newline character as bytes
pub const NL: &u8 = &b'\n';

/// Space character as bytes
pub const SPACE: &u8 = &b' ';

#[cfg(any(target_os = "windows", target_arch = "wasm32"))]
pub(crate) const INVALID_UTF8: &str = "unexpected invalid UTF-8 code point";
