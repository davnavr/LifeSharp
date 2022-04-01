//! Opeartions on line and column numbers in the LifeSharp source code files.

#![deny(missing_docs, missing_debug_implementations)]

/// Represents a line or column number.
pub use std::num::NonZeroUsize as Number;

/// Represents a UTF-8 byte offset into a source code file.
pub type Offset = usize;

/// Represents a range of characters in a source code file.
pub type OffsetRange = std::ops::Range<Offset>;

/// Represents a line and column number in a source file.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Location {
    line: Number,
    column: Number,
}

impl Location {
    /// Gets the line number.
    pub fn line_number(&self) -> Number {
        self.line
    }

    /// Gets the column number.
    pub fn column_number(&self) -> Number {
        self.column
    }
}
