//! Contains types used to represent the structure of LifeSharp source files.

use std::fmt::{Display, Formatter};
use crate::location::OffsetRange;

pub use crate::identifier::Id;

/// Represents content in a source code file associated with its location.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct Located<T> {
    /// Value representing something that was parsed.
    pub contents: T,
    /// Location in the source code file.
    pub location: OffsetRange,
}

/// Represents a function definition.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct FunctionDefinition<'t> {
    /// The name of the function.
    pub name: Located<&'t Id>,
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
