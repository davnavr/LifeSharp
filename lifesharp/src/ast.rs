//! Contains types used to represent the structure of LifeSharp source files.

use crate::identifier;
use crate::location::{Offset, OffsetRange};
use std::fmt::{Display, Formatter};

/// Represents content in a source code file associated with its location.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct Located<T> {
    /// Value representing something that was parsed.
    pub content: T,
    /// Location in the source code file.
    pub location: OffsetRange,
}

impl<T> Located<T> {
    /// Associates parsed content with a location in the source file.
    pub fn new(content: T, start: Offset, end: Offset) -> Self {
        Self {
            content,
            location: OffsetRange { start, end },
        }
    }
}

impl<T: Display> Display for Located<T> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        Display::fmt(&self.content, f)
    }
}

/// An identifier in the source code file along with its location.
pub type Id<'i> = Located<&'i identifier::Id>;

/// Represents a function definition.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct FunctionDefinition<'t> {
    /// The name of the function.
    pub name: Id<'t>,
}

/// Represents a top-level declaration defined in a source code file.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum TopDeclaration<'t> {
    /// A function definition defined at the top level.
    FunctionDefinition(Box<FunctionDefinition<'t>>),
}

/// Represents the content of a single source file.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub struct Tree<'t> {
    /// The top-level declarations declared in the source file.
    pub declarations: Vec<TopDeclaration<'t>>,
}
