//! Tokenization of LifeSharp source code.

#![deny(missing_debug_implementations)]

use crate::identifier::Id;
use crate::location::{Location, Offset, OffsetRange};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[repr(transparent)]
pub struct LiteralString(Box<str>);

#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Token {
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
    /// Used as the path separator.
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
    LiteralString(LiteralString),
    LiteralBoolean(bool),
    Identifier(Box<Id>),
}

#[cfg(test)]
mod tests {
    use crate::lexer::Token;

    #[test]
    fn size_is_acceptable() {
        assert!(std::mem::size_of::<Token>() <= 24)
    }
}
