//! Extensions for `PathBuf`
//!
use std::path::PathBuf;

#[doc(hidden)]
pub trait PathBufExt {
    fn last(&self) -> String;
}

impl PathBufExt for PathBuf {
    /// returns the last component of a `PathBuf`, as a string.
    fn last(&self) -> String {
        self.components()
            .next_back()
            .unwrap()
            .as_os_str()
            .to_str()
            .unwrap()
            .to_owned()
    }
}
