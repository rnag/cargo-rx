//! This is mostly taken from the [osstringext.rs] module in `clap`.
//!
//! [osstringext.rs]: https://github.com/clap-rs/clap/blob/d51ae89656fda527ef1bccf53fca0ad78ecb8c29/src/osstringext.rs
//!
use crate::{NL, SPACE};
pub(crate) use ext_impl::*;

use std::ffi::OsStr;

#[cfg(not(any(target_os = "windows", target_arch = "wasm32")))]
mod ext_impl {
    pub(crate) use std::os::unix::ffi::OsStrExt;
}

#[cfg(any(target_os = "windows", target_arch = "wasm32"))]
mod ext_impl {
    use super::*;
    use crate::INVALID_UTF8;

    #[doc(hidden)]
    pub trait OsStrExt3 {
        fn as_bytes(&self) -> &[u8];
    }

    impl OsStrExt3 for OsStr {
        fn as_bytes(&self) -> &[u8] {
            self.to_str().map(|s| s.as_bytes()).expect(INVALID_UTF8)
        }
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
    ///   - if the argument contains any *newlines*, we replace any raw `\n`
    ///     characters with *newlines*, since the debug representation of
    ///     `OsStr` doesn't seem to carry them over.
    ///   - if the argument contains any *spaces*, we call the *debug*
    ///     representation of `OsStr`, which returns the display value wrapped
    ///     in quotes.
    ///   - otherwise, we just call `OsStr::to_str` to convert the Unicode
    ///     bytes data to a *&str* directly, and return an owned *String*.
    ///
    fn display_string(&self) -> String {
        let mut has_newline = false;
        let mut has_space = false;

        for b in self.as_bytes() {
            if b == NL {
                has_newline = true;
                break;
            }
            if b == SPACE {
                has_space = true;
            }
        }

        // From personal testing, the implementation with `shellwords::escape()`
        // appears to be about **40x slower** in general, than just using the
        // debug representation of `OsStr`. However, I agree it would be nice
        // to use `shellwords` - at least on Mac/Linux environments.
        //
        // #[cfg(not(target_family = "windows"))]
        // #[inline]
        // fn escape_with_newlines(input: &OsStr) -> String {
        //     shellwords::escape(input.to_str().unwrap())
        // }

        #[inline]
        fn escape_with_newlines(input: &OsStr) -> String {
            format!("{input:?}").replace(r"\n", "\n")
        }

        if has_newline {
            escape_with_newlines(self)
        } else if has_space {
            format!("{self:?}")
        } else {
            self.to_str().unwrap().to_owned()
        }
    }
}
