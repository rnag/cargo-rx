//! This is mostly taken from the [osstringext.rs] module in `clap`.
//!
//! [osstringext.rs]: https://github.com/clap-rs/clap/blob/d51ae89656fda527ef1bccf53fca0ad78ecb8c29/src/osstringext.rs
//!
#[cfg(any(target_os = "windows", target_arch = "wasm32"))]
use crate::INVALID_UTF8;
use crate::{NL, SPACE};

use std::ffi::OsStr;
#[cfg(not(any(target_os = "windows", target_arch = "wasm32")))]
use std::os::unix::ffi::OsStrExt;

#[cfg(any(target_os = "windows", target_arch = "wasm32"))]
pub trait OsStrExt3 {
    fn as_bytes(&self) -> &[u8];
}

#[cfg(any(target_os = "windows", target_arch = "wasm32"))]
impl OsStrExt3 for OsStr {
    fn as_bytes(&self) -> &[u8] {
        self.to_str().map(|s| s.as_bytes()).expect(INVALID_UTF8)
    }
}

#[doc(hidden)]
pub trait OsStrExt2 {
    fn contains_byte(&self, byte: u8) -> bool;
    fn display_string(&self) -> String;
}

impl OsStrExt2 for OsStr {
    /// Check if an `OsStr` contains byte characters
    fn contains_byte(&self, byte: u8) -> bool {
        for b in self.as_bytes() {
            if b == &byte {
                return true;
            }
        }
        false
    }

    /// Return the *formatted* string representation of an `OsStr`.
    ///
    /// Here we treat the `OsStr` as a command-line argument, so the current
    /// implementation is as follows:
    ///   - if the argument contains any *newlines*, we defer the formatting
    ///     to `shellwords::escape` instead.
    ///   - if the argument contains any *spaces*, we call the *debug*
    ///     representation of `OsStr`, which returns the display value wrapped
    ///     in quotes.
    ///   - otherwise, we just call `OsStr::to_str` to convert the Unicode
    ///     bytes data to a *&str* directly, and return an owned *String*.
    ///
    fn display_string(&self) -> String {
        let mut has_nl = false;
        let mut has_space = false;

        for b in self.as_bytes() {
            if b == NL {
                has_nl = true;
                break;
            }
            if b == SPACE {
                has_space = true;
            }
        }

        if has_nl {
            shellwords::escape(self.to_str().unwrap())
        } else if has_space {
            format!("{self:#?}")
        } else {
            self.to_str().unwrap().to_owned()
        }
    }
}
