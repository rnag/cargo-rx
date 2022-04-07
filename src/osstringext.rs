//! This is mostly taken from the [osstringext.rs] module in `clap`.
//!
//! [osstringext.rs]: https://github.com/clap-rs/clap/blob/d51ae89656fda527ef1bccf53fca0ad78ecb8c29/src/osstringext.rs
//!
#[cfg(any(target_os = "windows", target_arch = "wasm32"))]
use crate::INVALID_UTF8;
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
    /// Here we treat the `OsStr` as a command-line argument, so if contains
    /// any spaces, we call the *debug* representation of `OsStr`, which
    /// returns the display value wrapped in quotes.
    fn display_string(&self) -> String {
        if self.contains_byte(b' ') {
            format!("{self:?}")
        } else {
            self.to_str().unwrap().to_owned()
        }
    }
}
