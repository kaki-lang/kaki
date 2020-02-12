//! Structures which represent the abstract syntax tree of a program.

use super::span::Span;
use num_bigint::BigInt;

/// The value of an atom in an expression.
#[derive(Clone, Debug, PartialEq)]
pub enum Atom {
    /// A boolean.
    Bool(Span, bool),

    /// An integer.
    Int(Span, BigInt),

    /// A floating point number.
    Float(Span, f64),

    /// A `none`.
    None(Span),

    /// A single line string.
    StringSingle(Span, String),

    /// A multiline string.
    StringMulti(Span, String),

    /// A smart string.
    StringSmart(Span, String),
}

/// A name in an expression.
#[derive(Clone, Debug, PartialEq)]
pub enum Name {
    /// A name whose first non-underscore character is a lowercase letter.
    Lower(Span, String),

    /// A name whose first non-underscore character is a uppercase letter.
    Upper(Span, String),

    /// A name that starts with `@`, which represents a field.
    Field(Span, String),

    /// A name that starts with `@@`, which represents a static field.
    FieldStatic(Span, String),

    /// A name which is only a single underscore.
    Underscore(Span),

    /// An anonymous name which is either `$` or `$<non-negative>`.
    Anonymous(Span, u32),
}

/// A binary operator in an expression.
#[derive(Clone, Debug, PartialEq)]
pub enum BinaryOp {
    /// The binary `&` operator.
    Amp(Span),

    /// The binary `&&` operator.
    AmpAmp(Span),

    /// The binary `!=` operator.
    BangEq(Span),

    /// The binary `^` operator.
    Caret(Span),

    /// The binary `::` operator.
    ColonColon(Span),

    /// The binary `.` operator.
    Dot(Span),

    /// The binary `=` operator.
    Eq(Span),

    /// The binary `==` operator.
    EqEq(Span),

    /// The binary `>` operator.
    Gt(Span),

    /// The binary `>` operator.
    GtGt(Span),

    /// The binary `>=` operator.
    GtEq(Span),

    /// The binary `<` operator.
    Lt(Span),

    /// The binary `<<` operator.
    LtLt(Span),

    /// The binary `<=` operator.
    LtEq(Span),

    /// The binary `-` operator.
    Minus(Span),

    /// The binary `%` operator.
    Percent(Span),

    /// The binary `|` operator.
    Pipe(Span),

    /// The binary `||` operator.
    PipePipe(Span),

    /// The binary `+` operator.
    Plus(Span),

    /// The binary `?=` operator.
    QuestionEq(Span),

    /// The binary `/` operator.
    Slash(Span),

    /// The binary `//` operator.
    SlashSlash(Span),

    /// The binary `*` operator.
    Star(Span),

    /// The binary `**` operator.
    StarStar(Span),
}

/// A unary operator in an expression.
#[derive(Clone, Debug, PartialEq)]
pub enum UnaryOp {
    /// The unary `!` operator.
    Bang(Span),

    /// The unary `-` operator.
    Minus(Span),

    /// The unary `*` operator.
    Star(Span),

    /// The unary `**` operator.
    StarStar(Span),

    /// The unary `~` operator.
    Tilde(Span),
}

/// Arguments to a function.
#[derive(Clone, Debug, PartialEq)]
pub enum FuncArg {
    /// A positional argument.
    Positional(Box<Expr>),

    /// Variadic arguments passed with `*`.
    Variadic(Vec<Expr>),

    /// A keyword argument, where the left item is keyword and the right item is the value.
    Keyword(Box<Expr>, Box<Expr>),

    /// Keyword arguments passed with `**`.
    Keywords(Box<Expr>),

    /// A block argument passed with `&`.
    BlockArg(Box<Expr>),

    /// A block argument passed as a literal.
    BlockFunc(Box<FuncArgBlock>),
}

/// A block passed as an argument to a function.
#[derive(Clone, Debug, PartialEq)]
pub struct FuncArgBlock {
    /// The argument list to the block.
    pub args: Option<Vec<FuncArg>>,

    /// The body of the block.
    pub expr: Box<Expr>,
}

/// Access modifiers on items within a type.
#[derive(Clone, Debug, PartialEq)]
pub enum TypeAccessModifier {
    /// The item is marked as private, which is the absence of other modifiers.
    Private,

    /// The item is marked as public.
    Public,
}

/// The items that can appear inside of a type.
#[derive(Clone, Debug, PartialEq)]
pub enum TypeItems {
    /// A constructor, which contains whether it is public, the name, the argument list, and the
    /// body.
    Cons(TypeAccessModifier, Box<Expr>, Vec<FuncArg>, Box<Expr>),

    /// A method, which contains whether it is public, the name, the argument list, and the body.
    Method(TypeAccessModifier, Box<Expr>, Vec<FuncArg>, Box<Expr>),

    /// A static method, which contains whether it is public, the name, the argument list, and the
    /// body.
    MethodStatic(TypeAccessModifier, Box<Expr>, Vec<FuncArg>, Box<Expr>),

    /// A property, which contains whether it is public, the name, and the body.
    Property(TypeAccessModifier, Box<Expr>, Box<Expr>),

    /// A static property, which contains whether it is public, the name, and the body.
    PropertyStatic(TypeAccessModifier, Box<Expr>, Box<Expr>),

    /// A binary operator, which contains whether it is public, the first operand, the operator,
    /// the second operand, and the body.
    BinaryOp(
        TypeAccessModifier,
        Box<Expr>,
        BinaryOp,
        Box<Expr>,
        Box<Expr>,
    ),

    /// A unary operator, which contains whether it is public, the operator, the operand, and the
    /// body.
    UnaryOp(TypeAccessModifier, UnaryOp, Box<Expr>),
}

/// Access modifiers on items within a trait.
#[derive(Clone, Debug, PartialEq)]
pub enum TraitAccessModifier {
    /// The item is marked as abstract.
    Abstract,

    /// The item is marked as private, which is the absence of other modifiers.
    Private,

    /// The item is marked as public.
    Public,
}

/// The items that can appear inside of a trait.
#[derive(Clone, Debug, PartialEq)]
pub enum TraitItems {
    /// A constructor, which contains the body.
    Cons(Box<Expr>),

    /// A method, which contains whether it is abstract, the name, the argument list, and the body.
    Method(TraitAccessModifier, Box<Expr>, Vec<FuncArg>, Box<Expr>),

    /// A binary operator, which contains whether it is abstract, the first operand, the operator,
    /// the second operand, and the body.
    BinaryOp(
        TraitAccessModifier,
        Box<Expr>,
        BinaryOp,
        Box<Expr>,
        Box<Expr>,
    ),

    /// A unary operator, which contains whether it is abstract, the operator, the operand, and the
    /// body.
    UnaryOp(TraitAccessModifier, UnaryOp, Box<Expr>),
}

/// An expression. This is really the abstract syntax tree, since an entire program is simply an
/// expression.
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    /// An atomic value.
    Atom(Atom),

    /// A name.
    Name(Name),

    /// A block used for scoping. This means the block is not a function.
    Block(Vec<Expr>),

    /// A call to a function. This is contains the function itself (which may be an expression)
    /// and the arguments, and an optional literal block argument.
    FunctionCall(Box<Expr>, Vec<FuncArg>, Box<FuncArgBlock>),

    /// A binary operator, containing the left argument, the operator type, and the right argument.
    BinaryOp(Box<Expr>, BinaryOp, Box<Expr>),

    /// A unary operator, containing the operator type and the argument.
    UnaryOp(UnaryOp, Box<Expr>),

    /// An `if` expression, containing the condition, the expression to evaluate when true, and an
    /// optional `else` expression.
    If(Box<Expr>, Box<Expr>, Option<Box<Expr>>),

    /// A loop, containing the loop body.
    Loop(Box<Expr>),

    /// A `for` loop, containing the loop variable name, the loop sequence, and the loop body.
    For(Box<Expr>, Box<Expr>, Box<Expr>),

    /// A `while` loop, containing the loop condition and the loop body.
    While(Box<Expr>, Box<Expr>),

    /// A `type` definition, containing the name, the traits it has, and the items it defines.
    Type(Box<Expr>, Vec<Expr>, Vec<TypeItems>),

    /// A `trait` definition, containing the name, the traits it has, and the items it defines.
    Trait(Box<Expr>, Vec<Expr>, Vec<TraitItems>),

    /// A `use` expression.
    Use(Box<Expr>),

    /// A `pub` expression.
    Pub(Box<Expr>),

    /// A `break` expression with an optional subexpression.
    Break(Option<Box<Expr>>),

    /// A `continue` expression with an optional subexpression.
    Continue(Option<Box<Expr>>),

    /// A `return` expression with an optional subexpression.
    Return(Option<Box<Expr>>),
}
