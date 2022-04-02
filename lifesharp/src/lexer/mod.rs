//! Tokenization of LifeSharp source code.

use crate::identifier::{Id, Identifier};
use crate::location::{Location, Offset, OffsetRange};
use crate::print;
use typed_arena::Arena;

mod input;

pub use input::{CharIteratorInput, InputSource, Input};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[repr(transparent)]
pub struct LiteralString(String);

impl std::ops::Deref for LiteralString {
    type Target = String;

    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for LiteralString {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}

impl From<String> for LiteralString {
    fn from(literal: String) -> Self {
        Self(literal)
    }
}

impl print::Print for LiteralString {
    fn print(&self, printer: &mut print::Printer) -> print::Result {
        printer.write_char('\'')?;
        todo!("write characters");
        printer.write_char('\'')
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Token<'l> {
    Dedent,
    Indent,
    OpenCurlyBrace,
    CloseCurlyBrace,
    OpenParenthesis,
    CloseParenthesis,
    OpenSquareBracket,
    CloseSquareBracket,
    LessThan,
    GreaterThan,
    /// Used as the path separator (e.g. `some\path\to::SomeType`).
    BackwardSlash,
    PlusSign,
    MinusSign,
    Asterisk,
    /// Used to indicate the type of something, such as a local variable (e.g. `let x: u32`), parameter, or return type.
    Semicolon,
    ForwardSlash,
    Period,
    Equals,
    Ampersand,
    VerticalBar,
    /// Used to denote an item within a path, such as in `some\modules\containing::MyType`, where semicolons indicate that
    /// `MyType` is the name of a type.
    DoubleSemicolon,
    /// The `def` keyword indicates the start of a function definition.
    Define,
    /// The assignment operator (`<-`) writes a value to a memory location.
    Assignment,
    /// Indicates the start of an anonymous function (`fun`).
    Lambda,
    /// Indicates the return value of an anonymous function (`fun (x: u32) -> x + 1u32`).
    LambdaReturn,
    /// The `use` keyword brings items within a path into scope.
    Use,
    /// The `type` keyword indicates the start of a type definition.
    Type,
    //And, // TODO: How will bitwise operators be represented?
    //Not,
    //Or,
    LiteralCharacter(char),
    LiteralString(&'l LiteralString),
    LiteralBoolean(bool),
    Identifier(&'l Identifier),
}

/// Allows the reuse of some objects allocated during tokenization.
#[derive(Default)]
pub struct Cache<'o> {
    //line_buffer: String,
    tokens: Vec<(Token<'o>, OffsetRange)>,
    //locations:
}

#[derive(Default)]
pub struct Output<'o> {
    tokens: Box<[(Token<'o>, OffsetRange)]>,
    literal_strings: Arena<LiteralString>,
    identifiers: Arena<Identifier>,
    locations: (), //LocationMap,
}

pub fn tokenize<'o, S: InputSource>(
    source: S,
    cache: Option<&mut Cache<'o>>,
) -> Result<Output<'o>, <<S as InputSource>::IntoInput as Input>::Error> {
    let mut output = Output::default();

    if let Some(Cache {
        tokens: previous_tokens,
    }) = cache
    {
        previous_tokens.clear();
        //previous_output.locations.clear();
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use crate::lexer::Token;

    #[test]
    fn size_is_acceptable() {
        assert!(std::mem::size_of::<Token>() <= 16)
    }
}
