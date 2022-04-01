//! Opeartions on line and column numbers in the LifeSharp source code files.

/// Represents a line or column number.
pub use std::num::NonZeroUsize as Number;

/// Represents a UTF-8 byte offset into a source code file.
pub type Offset = usize;

/// Represents a range of characters in a source code file.
pub type OffsetRange = std::ops::Range<Offset>;
