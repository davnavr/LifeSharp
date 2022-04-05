//! Opeartions on line and column numbers in the LifeSharp source code files.

#![deny(missing_docs, missing_debug_implementations)]

/// Represents a line or column number.
pub use std::num::NonZeroUsize as Number;

/// The first line or column number.
pub const FIRST_NUMBER: Number = unsafe { Number::new_unchecked(1) };

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
    /// Gets a location corresponding to the first character of the file
    pub const FIRST: Self = Self {
        line: FIRST_NUMBER,
        column: FIRST_NUMBER,
    };

    /// Gets the line number.
    pub fn line_number(&self) -> Number {
        self.line
    }

    /// Gets the column number.
    pub fn column_number(&self) -> Number {
        self.column
    }
}

/// Maps offsets in a source file to line and column numbers.
#[derive(Debug)]
pub struct Map {
    
}
