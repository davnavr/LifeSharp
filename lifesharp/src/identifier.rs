//! Types to represent identifier strings in source code.

/// Represents a borrowed identifier string.
#[derive(Debug, Eq, Hash, PartialEq)]
#[repr(transparent)]
pub struct Id(str);


