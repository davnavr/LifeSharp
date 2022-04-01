//! Printing LifeSharp source code.

use std::fmt::{Formatter, Write as _};

/// Type returned by functions that print source code.
pub use std::fmt::Result;

/// Used for printing source code.
pub struct Printer<'a> {
    output: Formatter<'a>,
    indent_level: usize,
    /// If `true`, indicates that indentation has not yet been written for the current line of source code.
    write_indent: bool,
}

impl Printer<'_> {
    /// Creates a printer that writes source code to the specified `Formatter`.
    pub fn new(output: Formatter<'_>) -> Self {
        Self {
            output,
            indent_level: 0,
            write_indent: true,
        }
    }

    /// Increases the indentation level of any following indentation that is written.
    pub fn indent(&mut self) {
        self.indent_level += 1;
    }

    /// Decreases the indentation level of any following indentation that is written.
    pub fn dedent(&mut self) {
        self.indent_level -= 1;
    }

    fn write_indentation(&mut self) -> Result {
        if self.write_indent {
            for _ in 0..self.indent_level {
                self.output.write_str("    ")?;
            }

            self.write_indent = false;
        }

        Ok(())
    }

    /// Writes a newline into the source code, indicating that indentation must be written in the new line.
    /// 
    /// Use this as the primary means to emit newlines into the output, as other methods will not indicate that a indentation
    /// must be written.
    pub fn newline(&mut self) -> Result {
        self.write_indent = true;
        self.output.write_char('\n')?;
    }

    /// Writes a character to the output.
    pub fn write_char(&mut self, c: char) -> Result {
        self.write_indentation()?;
        self.output.write_char(c)
    }

    /// Writes a string into the output.
    pub fn write_str(&mut self, s: &str) -> Result {
        self.write_indentation()?;
        self.output.write_str(s)
    }

    /// Writes the formatted arguments into the output.
    pub fn write_fmt(&mut self, f: std::fmt::Arguments<'_>) -> Result {
        self.write_indentation()?;
        self.output.write_fmt(f)
    }
}

/// Trait implemented by types that represent AST nodes to print source code.
pub trait Print {
    /// Prints source code.
    fn print(&self, printer: &mut Printer<'_>) -> Result;
}
