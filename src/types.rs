//! Library-specific type definitions

/// A simple type alias so as to DRY.
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + 'static>>;
