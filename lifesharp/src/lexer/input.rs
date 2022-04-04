//! Allows the reading of strings from LifeSharp source code.

#![deny(missing_docs)]

use crate::location::{Location, Offset, OffsetRange};
use std::iter::IntoIterator;

/// Buffer used to store a [`String`] without line feed (`\n`) or carriage return (`\r`) characters.
#[derive(Debug)]
#[repr(transparent)]
pub struct LineBuffer<'a>(&'a mut String);

impl LineBuffer<'_> {
    /// Appends a character to the buffer.
    ///
    /// # Panics
    /// Panics if a line feed (`\n`) or carriage return (`\r`) is appended.
    pub fn push(&mut self, c: char) {
        match c {
            '\n' | '\r' => panic!("no newline characters are allowed"),
            _ => self.0.push(c),
        }
    }
}

/// Result type used when reading a line from an [`Input`].
pub type LineResult<'a, E> = Result<Option<LineBuffer<'a>>, E>;

/// Implemented by inputs to the tokenizer, allowing the reading of lines from a source.
pub trait Input {
    /// Type returned if an attempt to read characters from the source fails.
    type Error;

    /// Retrieves the next line of characters from the source and stores them in the buffer.
    ///
    /// To indicate the end of the file, return an `Ok(None)`.
    fn next_line<'a>(&mut self, buffer: LineBuffer<'a>) -> LineResult<'a, Self::Error>;
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

    fn next_line<'a>(&mut self, buffer: LineBuffer<'a>) -> LineResult<'a, Self::Error> {
        todo!("read characters until EOF or LF or CRLF is encountered");
        Ok(None)
    }
}

/// Reads lines of source code from a [`Read`].
#[derive(Debug)]
#[repr(transparent)]
pub struct ReaderInput<R> {
    reader: R,
}

impl<R: std::io::Read> From<R> for ReaderInput<R> {
    fn from(reader: R) -> Self {
        Self { reader }
    }
}

impl<R: std::io::Read> Input for ReaderInput<R> {
    type Error = std::io::Error;

    fn next_line<'a>(&mut self, buffer: LineBuffer<'a>) -> LineResult<'a, Self::Error> {
        todo!("read characters until EOF or LF or CRLF is encountered");
        Ok(None)
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

pub(super) struct Wrapper<'b, I> {
    input: I,
    buffer: &'b mut String,
}
impl<'b, I: Input> Wrapper<'b, I> {
    pub(super) fn new<S: InputSource<IntoInput = I>>(source: S, buffer: &'b mut String) -> Self {
        Self {
            input: source.into_input(),
            buffer,
        }
    }

    pub(super) fn next_line(&mut self) -> Result<Option<&str>, <I as Input>::Error> {
        self.buffer.clear();
        if self.input.next_line(LineBuffer(self.buffer))?.is_some() {
            Ok(Some(self.buffer.as_str()))
        } else {
            Ok(None)
        }
    }
}
