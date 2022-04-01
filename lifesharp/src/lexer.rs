//! Tokenization of LifeSharp source code.

use crate::identifier::Id;
use crate::location::{Offset, OffsetRange};

#![deny(missing_debug_implementations)]

#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Token {
    Dedent,
    Indent,
    OpenBracket,
    CloseBracket,

}
