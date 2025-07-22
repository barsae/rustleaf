#[derive(Debug, Clone, PartialEq)]
pub struct Program(pub Vec<Statement>);

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub statements: Vec<Statement>,
    pub final_expr: Option<Box<Expression>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    // Macro annotation wrapping another statement
    Macro {
        name: String,
        args: Vec<MacroArg>,
        statement: Box<Statement>,
    },

    // Expression statement (expression followed by semicolon)
    Expression(Expression),

    // Variable declaration with optional destructuring pattern
    VarDecl {
        pattern: Pattern,
        value: Option<Expression>,
    },

    // Assignment (statement-only, not expression)
    Assignment {
        target: LValue,
        op: AssignOp,
        value: Expression,
    },

    // Function declaration
    FnDecl {
        name: String,
        params: Vec<Parameter>,
        body: Block,  // Function body is a block
        is_pub: bool, // pub keyword
    },

    // Class declaration
    ClassDecl {
        name: String,
        members: Vec<ClassMember>,
        is_pub: bool, // pub keyword
    },

    // Module imports
    Import(ImportSpec),

    // Control flow (statement-only)
    Return(Option<Expression>),
    Break(Option<Expression>), // Can break with value from loop expressions
    Continue,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AssignOp {
    Assign,    // =
    AddAssign, // +=
    SubAssign, // -=
    MulAssign, // *=
    DivAssign, // /=
    ModAssign, // %=
}

#[derive(Debug, Clone, PartialEq)]
pub enum LValue {
    Identifier(String),
    GetAttr(Box<Expression>, String),          // obj.field
    GetItem(Box<Expression>, Box<Expression>), // obj[key]
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    // Literals and identifiers
    Literal(LiteralValue),
    Identifier(String),
    Super, // super keyword for parent class access

    // Property access and method calls
    GetAttr(Box<Expression>, String),               // obj.field
    GetItem(Box<Expression>, Box<Expression>),      // obj[key]
    FunctionCall(Box<Expression>, Vec<Expression>), // func(args)
    MethodCall(Box<Expression>, String, Vec<Expression>), // obj.method(args)

    // Binary operators
    Add(Box<Expression>, Box<Expression>), // +
    Sub(Box<Expression>, Box<Expression>), // -
    Mul(Box<Expression>, Box<Expression>), // *
    Div(Box<Expression>, Box<Expression>), // /
    Mod(Box<Expression>, Box<Expression>), // %
    Pow(Box<Expression>, Box<Expression>), // **

    // Comparison operators
    Eq(Box<Expression>, Box<Expression>), // ==
    Ne(Box<Expression>, Box<Expression>), // !=
    Lt(Box<Expression>, Box<Expression>), // <
    Le(Box<Expression>, Box<Expression>), // <=
    Gt(Box<Expression>, Box<Expression>), // >
    Ge(Box<Expression>, Box<Expression>), // >=

    // Membership and identity operators
    In(Box<Expression>, Box<Expression>), // in
    Is(Box<Expression>, Box<Expression>), // is

    // Bitwise operators
    BitAnd(Box<Expression>, Box<Expression>),     // &
    BitOr(Box<Expression>, Box<Expression>),      // |
    BitXor(Box<Expression>, Box<Expression>),     // ^
    LeftShift(Box<Expression>, Box<Expression>),  // <<
    RightShift(Box<Expression>, Box<Expression>), // >>

    // Logical operators (with short-circuit semantics)
    And(Box<Expression>, Box<Expression>), // and
    Or(Box<Expression>, Box<Expression>),  // or
    Xor(Box<Expression>, Box<Expression>), // xor

    // Unary operators
    Neg(Box<Expression>),    // -expr
    Not(Box<Expression>),    // not expr
    BitNot(Box<Expression>), // ~expr

    // Pipe operator
    Pipe(Box<Expression>, Box<Expression>), // expr1 : expr2

    // Range operators
    RangeExclusive(Box<Expression>, Box<Expression>), // expr1..expr2
    RangeInclusive(Box<Expression>, Box<Expression>), // expr1..=expr2

    // Control flow expressions
    Block(Block),
    If {
        condition: Box<Expression>,
        then_expr: Block,
        else_expr: Option<Block>,
    },
    Match {
        expr: Box<Expression>,
        cases: Vec<MatchCase>,
    },
    While {
        condition: Box<Expression>,
        body: Block,
    },
    For {
        pattern: Pattern,
        iter: Box<Expression>,
        body: Block,
    },
    Loop {
        body: Block,
    },
    Try {
        body: Block,
        catch: CatchClause,
    },
    With {
        resources: Vec<WithResource>,
        body: Block,
    },

    // Closures
    Lambda {
        params: Vec<String>,
        body: LambdaBody,
    },

    // Collections
    List(Vec<Expression>),
    Dict(Vec<(Expression, Expression)>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    RawString(String), // r"..." strings
}

#[derive(Debug, Clone, PartialEq)]
pub enum LambdaBody {
    Expression(Box<Expression>), // |x| x + 1
    Block(Block),                // |x| { print(x); x + 1 }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub default: Option<LiteralValue>, // Default values must be literals
    pub kind: ParameterKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParameterKind {
    Regular, // name
    Rest,    // *name
    Keyword, // **name
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassMember {
    pub name: String,
    pub kind: ClassMemberKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClassMemberKind {
    Field(Option<Expression>), // Optional initializer
    Method {
        params: Vec<Parameter>,
        body: Block, // Methods have block bodies
    },
    StaticMethod {
        params: Vec<Parameter>,
        body: Block,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImportSpec {
    pub module: String,
    pub items: ImportItems,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ImportItems {
    All,                       // use module::*
    Specific(Vec<ImportItem>), // use module::{a, b as c}
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImportItem {
    pub name: String,
    pub alias: Option<String>, // for "as" renaming
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchCase {
    pub pattern: Pattern,
    pub guard: Option<Expression>,
    pub body: Block, // Match case bodies are blocks
}

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Literal(LiteralValue),
    Variable(String),
    Wildcard,                               // _
    List(Vec<Pattern>),                     // [a, b, *rest]
    ListRest(Vec<Pattern>, Option<String>), // [a, b, *rest] with optional rest capture
    Dict(Vec<DictPattern>),                 // {x, y: alias, z}
    Range(Box<Pattern>, Box<Pattern>),      // 1..10
}

#[derive(Debug, Clone, PartialEq)]
pub struct DictPattern {
    pub key: String,
    pub alias: Option<String>, // {key: alias} vs {key}
}

#[derive(Debug, Clone, PartialEq)]
pub struct CatchClause {
    pub pattern: Pattern, // Can pattern match on error object
    pub body: Block,      // Catch body is block
}

#[derive(Debug, Clone, PartialEq)]
pub struct WithResource {
    pub name: String,
    pub value: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MacroArg {
    Positional(LiteralValue),
    Named(String, LiteralValue), // key: value
}
