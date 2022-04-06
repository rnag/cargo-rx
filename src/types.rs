//! Library-specific type definitions

/// Represents any error type.
///
/// # Note
/// It maybe be worth it to refactor the error handling process to instead
/// use `anyhow` or something similar, if and when time allows.
pub type Error = Box<dyn std::error::Error + 'static>;

/// A simple type alias so as to DRY.
pub type Result<T> = std::result::Result<T, Error>;
