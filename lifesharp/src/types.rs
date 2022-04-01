//! Model of the LifeSharp type system.

use crate::print::{self, Print, Printer};

/// Represents a primitive type.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Primitive {
    /// A boolean, with the values `true` or `false`.
    Bool,
    /// A signed byte.
    S8,
    /// An unsigned byte.
    U8,
    /// Signed 16-bit integer.
    S16,
    /// Unsigned 16-bit integer.
    U16,
    /// Signed 32-bit integer.
    S32,
    /// Unsigned 32-bit integer.
    U32,
    /// Signed 64-bit integer.
    S64,
    /// Unsigned 64-bit integer.
    U64,
    /// Signed integer type.
    SSize,
    /// Unsigned integer type.
    USize,
    /// Single-precision floating-point type.
    F32,
    /// Double-precision floating-point type.
    F64,
}

impl Print for Primitive {
    fn print(&self, printer: &mut Printer) -> print::Result {
        printer.write_str(match self {
            Self::Bool => "bool",
            Self::S8 => "s8",
            Self::U8 => "u8",
            Self::S16 => "s16",
            Self::U16 => "u16",
            Self::S32 => "s32",
            Self::U32 => "u32",
            Self::S64 => "s64",
            Self::U64 => "u64",
            Self::SSize => "ssize",
            Self::USize => "usize",
            Self::F32 => "f32",
            Self::F64 => "f64",
        })
    }
}
