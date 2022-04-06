//! Opeartions on line and column numbers in the LifeSharp source code files.

#![deny(missing_docs, missing_debug_implementations)]

use std::collections::btree_map;

/// Represents a line or column number.
pub use std::num::NonZeroUsize as Number;

/// The first line or column number.
pub const FIRST_NUMBER: Number = unsafe { Number::new_unchecked(1) };

/// Increments a line or column number.
///
/// # Panics
/// Panics if the line or column number overflows to zero.
pub(crate) fn increment_number(number: &mut Number) {
    *number = Number::new(number.get() + 1).expect("location number overflowed");
}

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
    /// Gets a location corresponding to the first character of the file.
    pub const FIRST: Self = Self {
        line: FIRST_NUMBER,
        column: FIRST_NUMBER,
    };

    /// Gets the line number.
    pub fn line_number(&self) -> Number {
        self.line
    }

    /// Gets the column number, which is the nth code point from the start of the line to this location.
    pub fn column_number(&self) -> Number {
        self.column
    }
}

#[derive(Clone, Debug)]
struct MapKey(OffsetRange);

#[derive(Clone, Debug)]
struct MapEntry {
    line: Number,
}

/// Maps offsets in a source file to line and column numbers.
#[derive(Clone, Debug, Default)]
pub struct Map {
    lookup: btree_map::BTreeMap<MapKey, MapEntry>,
}

impl Map {
    pub(crate) fn insert(&mut self, line: Number, column: Number, offset_range: OffsetRange) {
        todo!("insert location")
    }
}
