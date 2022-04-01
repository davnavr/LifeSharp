//! Contains types used to represent the structure of LifeSharp source files.

#![deny(missing_debug_implementations)]

use crate::identifier;
use crate::location::{Offset, OffsetRange};
use crate::print::{self, Print, Printer};

/// Represents content in a source code file associated with its location.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct Located<T> {
    /// Value representing something that was parsed.
    pub content: T,
    /// Location in the source code file.
    pub location: OffsetRange,
}

impl<T> Located<T> {
    /// Associates parsed content with a location in the source file.
    pub fn new(content: T, start: Offset, end: Offset) -> Self {
        Self {
            content,
            location: OffsetRange { start, end },
        }
    }
}

impl<T: Print> Print for Located<T> {
    fn print(&self, p: &mut Printer) -> print::Result {
        self.content.print(p)
    }
}

/// An identifier in the source code file along with its location.
pub type Id<'t> = Located<&'t identifier::Id>;

/// A series of identifiers in source code used to indicate where a definition is located.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct PathId<'t> {
    /// Indicates if the path is relative or global.
    pub global: bool,
    /// The identifiers of the path.
    pub identifiers: Vec<Id<'t>>,
}

impl Print for PathId<'_> {
    fn print(&self, p: &mut Printer) -> print::Result {
        if self.global {
            p.write_char('\\')?;
        }

        for (index, identifier) in self.identifiers.iter().enumerate() {
            if index > 0 {
                p.write_char('\\')?;
            }

            identifier.print(p)?;
        }

        Ok(())
    }
}

impl Default for PathId<'_> {
    /// The default path, which refers to definitions in the current scope.
    fn default() -> Self {
        Self {
            global: false,
            identifiers: Vec::default(),
        }
    }
}

/// An identifier that refers to a type.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct TypeId<'t> {
    /// The path to the type.
    pub path: PathId<'t>,
    /// The name of the type.
    pub name: Id<'t>,
    //pub generic_arguments: Vec<>,
}

impl Print for TypeId<'_> {
    fn print(&self, p: &mut Printer) -> print::Result {
        Print::print(&self.path, p)?;
        p.write_str("::")?;
        Print::print(&self.name, p)?;
        Ok(())
    }
}

/// Represents the name of a type.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Type<'t> {
    /// A named type located with a path.
    Named(TypeId<'t>),
    //Array(),
    //RawPointer(),
}

/// Represents the definition of a generic parameter in a function or type definition.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct GenericParameterDefinition<'t> {
    /// The name of the generic parameter.
    pub name: Id<'t>,
    /// The type of the generic parameter.
    pub kind: GenericParameterKind<'t>,
}

/// Used to specify constraints on generic type arguments.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum GenericTypeConstraint<'t> {
    /// Requires that a generic type argument implement the specified trait.
    Implements(TypeId<'t>),
    //Outlives(LifetimeId<'t>),
}

/// Describes a generic parameter.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum GenericParameterKind<'t> {
    /// Indicates that the generic parameter is a type parameter with the specified constraints.
    Type(Vec<Located<GenericTypeConstraint<'t>>>),
    /// Indicates that the generic parameter is a lifetime parameter.
    Lifetime(()), //(Vec<LifetimeId<'t>>)
}

impl Print for GenericParameterDefinition<'_> {
    fn print(&self, p: &mut Printer) -> print::Result {
        match &self.kind {
            GenericParameterKind::Type(_) => p.write_char('\'')?,
            GenericParameterKind::Lifetime(_) => p.write_char('~')?,
        }

       self.name.print(p)?;

        match &self.kind {
            GenericParameterKind::Type(constraints) => {
                if !constraints.is_empty() {
                    p.write_str(": ")?;

                    for (index, constraint) in constraints.iter().enumerate() {
                        if index > 0 {
                            p.write_str(", ")?;
                        }

                        match &constraint.content {
                            GenericTypeConstraint::Implements(constraint_name) => {
                                Print::print(&constraint_name, p)?
                            }
                        }
                    }
                }
            }
            GenericParameterKind::Lifetime(()) => (),
        }

        Ok(())
    }
}

/// Represents a pattern.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Pattern<'t> {
    /// Binds the matched value to the specified name.
    Name(Id<'t>),
    /// Ignores the value.
    Ignore,
}

impl Print for Pattern<'_> {
    fn print(&self, p: &mut Printer) -> std::fmt::Result {
        match self {
            Self::Name(name) => Print::print(&name, p),
            Self::Ignore => p.write_char('_'),
        }
    }
}

/// Represents an expression.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Expression<'t> {
    //IntegerLiteral(),
    /// A local variable or parameter.
    Name(Id<'t>),
}

/// Represents a parameter in a function definition.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct Parameter<'t> {
    // TODO: Might be duplicated if Name pattern allows a type in it. Could remove explicit type here to allow type inference for parameters.
    /// The type of the parameter.
    pub argument_type: Type<'t>,
    /// Pattern applied to the argument.
    pub pattern: Pattern<'t>,
}

/// Represents a function definition.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct FunctionDefinition<'t> {
    /// The name of the function.
    pub name: Id<'t>,
    /// The parameters of the function.
    pub parameters: Vec<Parameter<'t>>,
    /// The generic parameters of the function.
    pub generic_parameters: Vec<GenericParameterDefinition<'t>>,
    /// The expressions that make up the function body.
    pub body: Vec<Located<Expression<'t>>>,
}

/// Represents a top-level declaration defined in a source code file.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum TopDeclaration<'t> {
    /// A function definition defined at the top level.
    FunctionDefinition(Box<FunctionDefinition<'t>>),
}

/// Represents the content of a single source file.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub struct Tree<'t> {
    //header: like how files in F# can start with module or namespace?
    /// The top-level declarations declared in the source file.
    pub declarations: Vec<TopDeclaration<'t>>,
}
