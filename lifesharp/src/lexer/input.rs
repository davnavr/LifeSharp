//! Allows the reading of strings from LifeSharp source code.

#![deny(missing_docs)]

use crate::location;
use std::convert::Infallible;

/// Buffer used to store a [`String`] without line feed (`\n`) or carriage return (`\r`) characters.
#[derive(Debug)]
#[repr(transparent)]
pub struct LineBuffer<'a>(&'a mut String); // TODO: Could have field for column number.

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

/// Indicates whether an [`Input`] still has lines to be read.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Continue {
    /// Indicates that more lines will follow.
    More,
    /// Indicates that the end of the file has been reached.
    End,
}

/// Implemented by inputs to the tokenizer, allowing the reading of lines from a source.
pub trait Input {
    /// Type returned if an attempt to read characters from the source fails.
    type Error;

    /// Retrieves the next line of characters from the source and stores them in the buffer.
    fn next_line<'a>(&mut self, buffer: LineBuffer<'a>) -> Result<Continue, Self::Error>;
}

impl Input for std::str::Lines<'_> {
    type Error = Infallible;

    fn next_line<'a>(&mut self, buffer: LineBuffer<'a>) -> Result<Continue, Self::Error> {
        if let Some(line) = self.next() {
            buffer.0.push_str(line);
            Ok(Continue::More)
        } else {
            Ok(Continue::End)
        }
    }
}

impl<B: std::io::BufRead> Input for std::io::Lines<B> {
    type Error = std::io::Error;

    fn next_line<'a>(&mut self, buffer: LineBuffer<'a>) -> std::io::Result<Continue> {
        match self.next() {
            Some(Ok(line)) => {
                buffer.0.push_str(line.as_str());
                Ok(Continue::More)
            }
            Some(Err(error)) => Err(error),
            None => Ok(Continue::End),
        }
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
    type IntoInput = std::str::Lines<'a>;

    fn into_input(self) -> Self::IntoInput {
        self.lines()
    }
}

impl InputSource for std::fs::File {
    type IntoInput = std::io::Lines<std::io::BufReader<Self>>;

    fn into_input(self) -> Self::IntoInput {
        std::io::BufRead::lines(std::io::BufReader::new(self))
    }
}

// TODO: Keep track of location information (could track column and line numbers directly, but storing byte offsets alongside tokens is more space efficient, so location::Map may be useful).

#[derive(Debug)]
pub(super) struct Wrapper<'b, I> {
    input: I,
    buffer: &'b mut String,
    current_line: location::Number,
}

impl<'b, I: Input> Wrapper<'b, I> {
    pub(super) fn new<S: InputSource<IntoInput = I>>(source: S, buffer: &'b mut String) -> Self {
        Self {
            input: source.into_input(),
            buffer,
            current_line: location::FIRST_NUMBER,
        }
    }

    pub(super) fn next_line(
        &mut self,
    ) -> Result<Option<(&str, location::Number)>, <I as Input>::Error> {
        self.buffer.clear();

        Ok(match self.input.next_line(LineBuffer(self.buffer))? {
            Continue::More => {
                let line_number = self.current_line;
                location::increment_number(&mut self.current_line);
                Some((self.buffer.as_str(), line_number))
            }
            Continue::End => None,
        })
    }
}
