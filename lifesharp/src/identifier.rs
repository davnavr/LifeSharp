//! Types to represent identifier strings in source code.

#![deny(missing_docs, missing_debug_implementations)]

use crate::print::{Print, Printer};
use std::fmt::{Debug, Display, Formatter};

/// Represents a borrowed identifier string.
#[derive(Eq, Hash, PartialEq)]
#[repr(transparent)]
pub struct Id(str);

/// Error used when a string is not a valid identifier.
#[derive(Clone, Debug, thiserror::Error)]
pub enum InvalidError {
    /// Used when an identifier contains an invalid character.
    #[error("{code_point} at index {index} is not a valid identifier character")]
    InvalidCodePoint {
        /// The invalid code point.
        code_point: char,
        /// The code point index of the code point in the string.
        index: usize,
    },
    /// Used when an identifier is empty.
    #[error("identifiers must not be empty")]
    Empty,
}

impl Id {
    /// Creates a reference to a borrowed identifier string from a reference to a borrowed UTF-8 string.
    ///
    /// # Safety
    /// Callers must ensure that the identifier string is not empty and contains valid identifier characters.
    pub unsafe fn new_unchecked(identifier: &str) -> &Self {
        std::mem::transmute(identifier)
    }

    /// Creates a reference to a borrowed identifier string, checking that the string is not empty and contains valid identifier
    /// characters.
    pub fn new(identifier: &str) -> Result<&Id, InvalidError> {
        if let Some((index, bad)) = identifier.chars().enumerate().find(|(i, c)| {
            !(c.is_ascii_alphabetic() || *c == '_') || (*i == 0 && c.is_ascii_digit())
        }) {
            Err(InvalidError::InvalidCodePoint {
                code_point: bad,
                index,
            })
        } else if identifier.is_empty() {
            Err(InvalidError::Empty)
        } else {
            unsafe {
                // Safety: Validation is performed above.
                Ok(Id::new_unchecked(identifier))
            }
        }
    }

    /// Interprets the contents of this identifier string as a borrowed UTF-8 string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'i> TryFrom<&'i str> for &'i Id {
    type Error = InvalidError;

    fn try_from(identifier: &str) -> Result<&Id, InvalidError> {
        Id::new(identifier)
    }
}

impl Debug for Id {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl Print for Id {
    fn print(&self, p: &mut Printer) -> std::fmt::Result {
        p.write_str(&self.0)
    }
}

impl std::ops::Deref for Id {
    type Target = str;

    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl std::convert::AsRef<str> for Id {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::borrow::Borrow<str> for Id {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl Clone for Box<Id> {
    fn clone(&self) -> Self {
        unsafe {
            // Safety: Id has same layout as str.
            let identifier = std::mem::transmute::<&Box<Id>, &Box<str>>(self);
            std::mem::transmute(identifier.clone())
        }
    }
}
