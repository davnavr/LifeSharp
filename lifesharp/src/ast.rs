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
    fn print(&self, printer: &mut Printer) -> print::Result {
        self.content.print(printer)
    }
}

impl<T: Print> std::fmt::Display for Located<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.print(&mut Printer::new(f))
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

impl<'t> PathId<'t> {
    /// Creates a global path.
    pub fn global(identifiers: Vec<Id<'t>>) -> Self {
        Self {
            global: true,
            identifiers,
        }
    }
}

impl Print for PathId<'_> {
    fn print(&self, printer: &mut Printer) -> print::Result {
        if self.global {
            printer.write_char('\\')?;
        }

        printer.write_iter(&self.identifiers, "\\")
    }
}

crate::print_display_impl!(PathId<'_>);

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

impl<'t> TypeId<'t> {
    /// Creates a new type with the specified path and name.
    pub fn new(path: PathId<'t>, name: Id<'t>) -> Self {
        Self { path, name }
    }
}

impl Print for TypeId<'_> {
    fn print(&self, printer: &mut Printer) -> print::Result {
        self.path.print(printer)?;
        printer.write_str("::")?;
        self.name.print(printer)
    }
}

crate::print_display_impl!(TypeId<'_>);

pub use crate::types::Primitive as PrimitiveType;

/// Represents the name of a type.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Type<'t> {
    /// A primitive type.
    Primitive(PrimitiveType),
    /// A named type located with a path.
    Named(TypeId<'t>),
    //Array { element_type: Box<Type<'t>>, count: u32 },
    //RawPointer(),
}

impl Print for Type<'_> {
    fn print(&self, printer: &mut Printer) -> print::Result {
        match self {
            Self::Primitive(primitive_type) => primitive_type.print(printer),
            Self::Named(type_name) => type_name.print(printer),
        }
    }
}

crate::print_display_impl!(Type<'_>);

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

impl Print for GenericTypeConstraint<'_> {
    fn print(&self, printer: &mut Printer) -> print::Result {
        match self {
            Self::Implements(type_name) => type_name.print(printer),
        }
    }
}

crate::print_display_impl!(GenericTypeConstraint<'_>);

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
    fn print(&self, printer: &mut Printer) -> print::Result {
        match &self.kind {
            GenericParameterKind::Type(_) => printer.write_char('\'')?,
            GenericParameterKind::Lifetime(_) => printer.write_char('~')?,
        }

        self.name.print(printer)?;

        match &self.kind {
            GenericParameterKind::Type(constraints) => {
                if !constraints.is_empty() {
                    printer.write_str(": ")?;
                    printer.write_iter(constraints, ", ")?;
                }
            }
            GenericParameterKind::Lifetime(()) => (),
        }

        Ok(())
    }
}

crate::print_display_impl!(GenericParameterDefinition<'_>);

/// Represents a pattern.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Pattern<'t> {
    /// Binds the matched value to the specified name.
    Name(Id<'t>),
    /// Ignores the value.
    Ignore,
}

impl std::default::Default for Pattern<'_> {
    fn default() -> Self {
        Self::Ignore
    }
}

impl Print for Pattern<'_> {
    fn print(&self, printer: &mut Printer) -> std::fmt::Result {
        match self {
            Self::Name(name) => name.print(printer),
            Self::Ignore => printer.write_char('_'),
        }
    }
}

crate::print_display_impl!(Pattern<'_>);

/// A series of expressions.
pub type Block<'t> = Vec<Located<Expression<'t>>>;

fn print_block<'t>(block: &[Located<Expression<'t>>], printer: &mut Printer) -> print::Result {
    printer.indent();

    for expression in block.iter() {
        expression.print(printer)?;
        printer.newline()?;
    }

    printer.dedent();

    Ok(())
}

/// Represents an `if`...`then`, `if`...`then`...`else`, or `if`...`then`...`elif`...`then`...`else` expression.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct IfElseExpression<'t> {
    /// The condition in the `if` part of the expression.
    pub condition: Expression<'t>,
    /// The expressions that are evaluated if the condition is true.
    pub true_branch: Block<'t>,
    /// The `elif` conditions and their corresponding block.
    pub other_branches: Vec<(Expression<'t>, Block<'t>)>,
    /// The expressions that are evaluated if no condition is met.
    pub else_branch: Block<'t>,
}

impl Print for IfElseExpression<'_> {
    fn print(&self, printer: &mut Printer) -> print::Result {
        printer.write_str("if ")?;
        self.condition.print(printer)?;
        printer.write_str(" then")?;
        printer.newline()?;
        print_block(&self.true_branch, printer)?;

        for (other_condition, other_branch) in self.other_branches.iter() {
            printer.write_str("elif ")?;
            other_condition.print(printer)?;
            printer.write_str(" then")?;
            printer.newline()?;
            print_block(other_branch, printer)?;
        }

        printer.write_str("else")?;
        printer.newline()?;
        print_block(&self.else_branch, printer)
    }
}

crate::print_display_impl!(IfElseExpression<'_>);

/// Represents an expression.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Expression<'t> {
    /// A literal boolean value.
    BooleanLiteral(bool),
    /// A conditional expression.
    IfElse(Box<IfElseExpression<'t>>),
    //Switch,
    //Match,
    /// A local variable or parameter.
    Name(Id<'t>),
}

impl Print for Expression<'_> {
    fn print(&self, printer: &mut Printer) -> std::fmt::Result {
        match self {
            Self::BooleanLiteral(value) => printer.write_str(if *value { "true" } else { "false" }),
            Self::IfElse(conditional) => conditional.print(printer),
            Self::Name(identifier) => identifier.print(printer),
        }
    }
}

crate::print_display_impl!(Expression<'_>);

/// Represents a parameter in a function definition.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct Parameter<'t> {
    /// Pattern applied to the argument.
    pub pattern: Pattern<'t>,
    // TODO: Might be duplicated if Name pattern allows a type in it. Could remove explicit type here to allow type inference for parameters.
    /// The type of the parameter.
    pub argument_type: Type<'t>,
}

impl<'t> Parameter<'t> {
    /// Creates a parameter with the specified type.
    pub fn new(argument_type: Type<'t>) -> Self {
        Self {
            pattern: Pattern::Ignore,
            argument_type,
        }
    }
}

impl Print for Parameter<'_> {
    fn print(&self, printer: &mut Printer) -> std::fmt::Result {
        printer.write_char('(')?;
        self.pattern.print(printer)?;
        printer.write_str(": ")?;
        self.argument_type.print(printer)?;
        printer.write_char(')')
    }
}

crate::print_display_impl!(Parameter<'_>);

/// Represents a function definition.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct FunctionDefinition<'t> {
    /// The name of the function.
    pub name: Id<'t>,
    /// The generic parameters of the function.
    pub generic_parameters: Vec<GenericParameterDefinition<'t>>,
    /// The parameters of the function.
    pub parameters: Vec<Parameter<'t>>,
    /// The return type of the function.
    pub return_type: Option<Type<'t>>,
    /// The expressions that make up the function body.
    pub body: Block<'t>,
}

impl<'t> FunctionDefinition<'t> {
    /// Creates a function definition with the specified name.
    pub fn new(name: Id<'t>) -> Self {
        Self {
            name,
            generic_parameters: Vec::default(),
            parameters: Vec::default(),
            return_type: None,
            body: Vec::default(),
        }
    }
}

impl Print for FunctionDefinition<'_> {
    fn print(&self, printer: &mut Printer) -> std::fmt::Result {
        printer.write_str("def ")?;
        self.name.print(printer)?;

        if !self.generic_parameters.is_empty() {
            printer.write_char('<')?;
            printer.write_iter(&self.generic_parameters, ", ")?;
            printer.write_char('>')?;
        }

        printer.write_char(' ')?;

        if self.parameters.is_empty() {
            printer.write_str("()")?;
        } else {
            printer.write_iter(&self.parameters, " ")?;
        }

        if let Some(return_type) = &self.return_type {
            printer.write_char(' ')?;
            return_type.print(printer)?;
        }

        printer.write_str(" =")?;
        printer.newline()?;
        print_block(&self.body, printer)?;
        Ok(())
    }
}

crate::print_display_impl!(FunctionDefinition<'_>);

/// Represents a top-level declaration defined in a source code file.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum TopDeclaration<'t> {
    /// A function definition defined at the top level.
    FunctionDefinition(Box<FunctionDefinition<'t>>),
}

impl<'t> From<FunctionDefinition<'t>> for TopDeclaration<'t> {
    fn from(function_definition: FunctionDefinition<'t>) -> Self {
        Self::FunctionDefinition(Box::new(function_definition))
    }
}

impl Print for TopDeclaration<'_> {
    fn print(&self, printer: &mut Printer) -> std::fmt::Result {
        match self {
            Self::FunctionDefinition(function_definition) => function_definition.print(printer),
        }
    }
}

crate::print_display_impl!(TopDeclaration<'_>);

/// Represents the content of a single source file.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub struct Tree<'t> {
    //header: like how files in F# can start with module or namespace?
    /// The top-level declarations declared in the source file.
    pub declarations: Vec<TopDeclaration<'t>>,
}

impl Print for Tree<'_> {
    fn print(&self, printer: &mut Printer) -> std::fmt::Result {
        for declaration in self.declarations.iter() {
            declaration.print(printer)?;
            printer.newline()?;
            printer.newline()?;
        }

        Ok(())
    }
}

crate::print_display_impl!(Tree<'_>);
