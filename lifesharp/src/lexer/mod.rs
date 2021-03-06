//! Tokenization of LifeSharp source code.

use crate::identifier::Identifier;
use crate::location::{self, Location, OffsetRange};
use crate::print;
use typed_arena::Arena;

mod input;

pub use input::{Continue, Input, InputSource};

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
    /// Used to separate things on the same line.
    Semicolon,
    ForwardSlash,
    Period,
    Equals,
    Ampersand,
    VerticalBar,
    /// Used to indicate the type of something, such as a local variable (e.g. `let x: u32`), parameter, or return type.
    Colon,
    /// Used to denote an item within a path, such as in `some\modules\containing::MyType`, where semicolons indicate that
    /// `MyType` is the name of a type.
    DoubleColon,
    /// The assignment operator (`<-`) writes a value to a memory location.
    Assignment,
    /// Indicates the return value of an anonymous function (`fun (x: u32) -> x + 1u32`).
    LambdaReturn,
    /// The `def` keyword indicates the start of a function definition.
    KeywordDef,
    /// Indicates the start of an anonymous function (`fun`).
    KeywordFun,
    /// The `use` keyword brings items within a path into scope.
    KeywordUse,
    /// The `type` keyword indicates the start of a type definition.
    KeywordType,
    //And, // TODO: How will bitwise operators be represented?
    //Not,
    //Or,
    LiteralCharacter(char),
    LiteralString(&'l LiteralString),
    LiteralBoolean(bool),
    Identifier(&'l Identifier),
    TypeParameter(&'l Identifier),
    LifetimeParameter(&'l Identifier),
}

/// Allows the reuse of some objects allocated during tokenization.
#[derive(Debug, Default)]
pub struct Cache<'o> {
    //buffer: std::cell::RefCell<Buffer>, // Allows dropping of all buffers when tokenization is done.
    line_buffer: String,
    tokens: Vec<(Token<'o>, OffsetRange)>,
    //locations:
    //literal_strings: Arena<LiteralString>,
    //identifiers: Arena<Identifier>,
}

#[derive(Debug, Default)]
pub struct Output<'o> {
    tokens: Box<[(Token<'o>, OffsetRange)]>,
    //literal_strings: Arena<LiteralString>,
    //identifiers: Arena<Identifier>,
    locations: (), //LocationMap,
}

impl Output<'_> {
    /// Gets the tokens from the source file.
    pub fn tokens(&self) -> &[(Token<'_>, OffsetRange)] {
        &self.tokens
    }

    //pub fn locations(&self) -> &LocationMap

    //pub fn located_tokens
}

pub fn tokenize<'o, S: InputSource>(
    source: S,
    cache: Option<&mut Cache<'o>>,
) -> Result<Output<'o>, <<S as InputSource>::IntoInput as Input>::Error> {
    let mut owned_line_buffer;
    let line_buffer: &mut String;

    let mut owned_tokens;
    let tokens: &mut Vec<(Token<'o>, OffsetRange)>;

    if let Some(Cache {
        line_buffer: ref mut previous_line_buffer,
        tokens: ref mut previous_tokens,
    }) = cache
    {
        line_buffer = previous_line_buffer;

        previous_tokens.clear();
        tokens = previous_tokens;

        //previous_output.locations.clear();
    } else {
        owned_line_buffer = String::default();
        line_buffer = &mut owned_line_buffer;

        owned_tokens = Vec::default();
        tokens = &mut owned_tokens;
    }

    let mut input = input::Wrapper::new(source, line_buffer);
    let mut next_byte_offset: location::Offset = 0;
    let mut current_indent_level = 0u64;

    /// Allows reading of characters from a line of source code, automatically counting position information and allowing
    /// backtracking.
    #[derive(Clone)]
    struct LineCharacters<'a> {
        remaining: std::str::Chars<'a>,
        column_number: location::Number,
        byte_offset: location::Offset,
    }

    impl<'a> LineCharacters<'a> {
        fn new(line: &'a str, byte_offset: location::Offset) -> Self {
            Self {
                remaining: line.chars(),
                column_number: location::FIRST_NUMBER,
                byte_offset,
            }
        }

        fn next_char(&self) -> Option<(char, location::Offset, Self)> {
            let mut remaining = self.remaining.clone();
            let next = remaining.next()?;
            let byte_offset = self.byte_offset;
            let mut next_characters = Self {
                remaining,
                byte_offset: self.byte_offset + 1,
                ..self.clone()
            };

            location::increment_number(&mut next_characters.column_number);
            Some((next, byte_offset, next_characters))
        }
    }

    while let Some((current_line, line_number)) = input.next_line()? {
        // TODO: Count leading spaces in current line to calculate indentation.

        let mut line = LineCharacters::new(current_line, next_byte_offset);

        while let Some((code_point, start_byte_offset, remaining_line)) = line.next_char() {
            macro_rules! simple_token {
                ($name: ident) => {{
                    // TODO: On token emit, add to the location map.
                    tokens.push((
                        Token::$name,
                        location::OffsetRange {
                            start: start_byte_offset,
                            end: start_byte_offset + 1,
                        },
                    ));
                    line = remaining_line;
                    break;
                }};
            }

            match code_point {
                '{' => simple_token!(OpenCurlyBrace),
                '}' => simple_token!(CloseCurlyBrace),
                '(' => simple_token!(OpenParenthesis),
                ')' => simple_token!(CloseParenthesis),
                '[' => simple_token!(OpenSquareBracket),
                ']' => simple_token!(CloseSquareBracket),
                '<' => simple_token!(LessThan),
                '>' => simple_token!(GreaterThan),
                '\\' => simple_token!(BackwardSlash),
                '+' => simple_token!(PlusSign),
                '-' => simple_token!(MinusSign),
                '*' => simple_token!(Asterisk),
                ';' => simple_token!(Semicolon),
                '/' => simple_token!(ForwardSlash),
                '.' => simple_token!(Period),
                '=' => simple_token!(Equals),
                '&' => simple_token!(Ampersand),
                '|' => simple_token!(VerticalBar),
                //':' // TODO: Check if double colon
                
                _ => todo!("other tokens"),
            }
        }

        next_byte_offset = line.byte_offset;
    }

    Ok(Output {
        tokens: tokens.clone().into_boxed_slice(),
        locations: (),
    })
}

#[cfg(test)]
mod tests {
    use crate::location::OffsetRange;
    use crate::lexer::{self, Token};

    #[test]
    fn token_size_is_acceptable() {
        assert!(std::mem::size_of::<Token>() <= 16)
    }

    macro_rules! single_token_test {
        ($name: ident, $input: expr, $output: expr) => {
            #[test]
            fn $name() {
                let input: &'static str = $input;
                let tokens = lexer::tokenize(input, None).unwrap();
                let expected: Token = $output;
                assert_eq!(&[ (expected, OffsetRange { start: 0, end: 1 }) ], tokens.tokens())
            }
        };
    }

    single_token_test!(open_curly_brace, "{", Token::OpenCurlyBrace);
    single_token_test!(close_curly_brace, "}", Token::CloseCurlyBrace);
    single_token_test!(open_parenthesis, "(", Token::OpenParenthesis);
    single_token_test!(close_parenthesis, ")", Token::CloseParenthesis);
    single_token_test!(open_square_bracket, "[", Token::OpenSquareBracket);
    single_token_test!(close_square_bracket, "]", Token::CloseSquareBracket);
    single_token_test!(less_than, "<", Token::LessThan);
    single_token_test!(greater_than, ">", Token::GreaterThan);
    single_token_test!(backward_slash, "\\", Token::BackwardSlash);
    single_token_test!(plus_sign, "+", Token::PlusSign);
    single_token_test!(minus_sign, "-", Token::MinusSign);
    single_token_test!(asterisk, "*", Token::Asterisk);
    single_token_test!(semicolon, ";", Token::Semicolon);
    single_token_test!(forward_slash, "/", Token::ForwardSlash);
    single_token_test!(period, ".", Token::Period);
    single_token_test!(equal_sign, "=", Token::Equals);
    single_token_test!(ampersand, "&", Token::Ampersand);
    single_token_test!(vertical_bar, "|", Token::VerticalBar);
}
