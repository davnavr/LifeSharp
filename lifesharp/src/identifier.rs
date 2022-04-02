//! Types to represent identifier strings in source code.

#![deny(missing_docs, missing_debug_implementations)]

use crate::print::{Print, Printer};
use std::borrow::Borrow;
use std::convert::AsRef;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;

/// A borrowed identifier string.
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

    /// Clones this borrowed identifier string to create an owned identifier string.
    pub fn to_identifier(&self) -> Identifier {
        unsafe {
            // Safety: Validation is performed in Id constructor.
            Identifier::new_unchecked(self.0.to_owned())
        }
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

impl Deref for Id {
    type Target = str;

    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<str> for Id {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Borrow<str> for Id {
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

/// An owned identifier string.
#[derive(Eq, Hash, PartialEq)]
#[repr(transparent)]
pub struct Identifier(String);

impl Identifier {
    /// Creates a new owned identifier string.
    ///
    /// # Safety
    /// See [`Id::new_unchecked`].
    pub unsafe fn new_unchecked(identifier: String) -> Self {
        Self(identifier)
    }

    /// Gets a reference to the underlying [`String`].
    pub fn as_string(&self) -> &String {
        &self.0
    }

    /// Gets a reference to a borrowed form of the identifier string.
    pub fn as_id(&self) -> &Id {
        unsafe {
            // Safety: Validation occurs in constructors.
            Id::new_unchecked(&self.0)
        }
    }
}

impl Debug for Identifier {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        Debug::fmt(self.as_id(), f)
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        Display::fmt(self.as_id(), f)
    }
}

impl Print for Identifier {
    fn print(&self, printer: &mut Printer) -> std::fmt::Result {
        self.as_id().print(printer)
    }
}

impl Deref for Identifier {
    type Target = String;

    fn deref(&self) -> &String {
        self.as_string()
    }
}

impl AsRef<String> for Identifier {
    fn as_ref(&self) -> &String {
        self.as_string()
    }
}

impl AsRef<Id> for Identifier {
    fn as_ref(&self) -> &Id {
        self.as_id()
    }
}

impl Borrow<String> for Identifier {
    fn borrow(&self) -> &String {
        self.as_string()
    }
}

impl Borrow<Id> for Identifier {
    fn borrow(&self) -> &Id {
        self.as_id()
    }
}

impl std::clone::Clone for Identifier {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }

    fn clone_from(&mut self, source: &Self) {
        std::clone::Clone::clone_from(&mut self.0, &source.0)
    }
}

impl std::borrow::ToOwned for Id {
    type Owned = Identifier;

    fn to_owned(&self) -> Identifier {
        self.to_identifier()
    }
}
