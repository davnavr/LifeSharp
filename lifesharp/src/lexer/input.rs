//! Allows the reading of strings from LifeSharp source code.

#![deny(missing_docs)]

use crate::location::{Location, Offset, OffsetRange};
use std::iter::IntoIterator;

/// Implemented by inputs to the tokenizer, allowing the reading of lines from a source.
pub trait Input {
    /// Type returned if an attempt to read characters from the source fails.
    type Error;

    /// Retrieves the next line of characters from the source and stores them in the buffer.
    /// If no characters are appended to the buffer, then it is interpreted as the end of the file being encountered.
    ///
    /// Implementors should ensure that no line feed (`\n`) or carriage return (`\r`) characters are present in the buffer.
    fn next_line(&mut self, buffer: &mut String) -> Result<(), Self::Error>;
}

/// Reads lines of source code from a character iterator.
#[derive(Debug)]
#[repr(transparent)]
pub struct CharIteratorInput<C>(C);

impl<C: IntoIterator<Item = char>> From<C> for CharIteratorInput<C> {
    fn from(iterator: C) -> Self {
        Self(iterator)
    }
}

impl<C: IntoIterator<Item = char>> Input for CharIteratorInput<C> {
    type Error = std::convert::Infallible;

    fn next_line(&mut self, buffer: &mut String) -> Result<(), Self::Error> {
        todo!("read characters until EOF or LF or CRLF is encountered");
        Ok(())
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct ReaderInput<R> {
    reader: R,
}

impl<R: std::io::Read> From<R> for ReaderInput<R> {
    fn from(reader: R) -> Self {
        Self {
            reader
        }
    }
}

impl<R: std::io::Read> Input for ReaderInput<R> {
    type Error = std::io::Error;

    fn next_line(&mut self, buffer: &mut String) -> Result<(), Self::Error> {
        todo!("read characters until EOF or LF or CRLF is encountered");
        Ok(())
    }
}

/// Conversion into an [`Input`] to the tokenizer.
pub trait InputSource {
    /// The type of input.
    type IntoInput: Input;

    /// Creates an input from a value.
    fn into_input(self) -> Self::IntoInput;
}

impl<I: Input> InputSource for I {
    type IntoInput = I;

    fn into_input(self) -> Self {
        self
    }
}

impl<'a> InputSource for &'a str {
    type IntoInput = CharIteratorInput<std::str::Chars<'a>>;

    fn into_input(self) -> Self::IntoInput {
        self.chars().into()
    }
}

impl InputSource for std::fs::File {
    type IntoInput = ReaderInput<std::fs::File>;

    fn into_input(self) -> Self::IntoInput {
        self.into()
    }
}

pub(super) struct InputWrapper<'c, I> {
    input: I,
    line_buffer: &'c mut String,
}
