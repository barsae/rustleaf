use crate::lexer::{LiteralValue, Token};
use std::fmt;

#[derive(Clone, PartialEq)]
pub enum AstNode {
    // Expressions
    Literal(LiteralValue, SourceLocation),
    Identifier(String, SourceLocation),
    BinaryOp {
        left: Box<AstNode>,
        operator: BinaryOperator,
        right: Box<AstNode>,
        location: SourceLocation,
    },
    UnaryOp {
        operator: UnaryOperator,
        operand: Box<AstNode>,
        location: SourceLocation,
    },
    PropertyAccess {
        object: Box<AstNode>,
        property: String,
        location: SourceLocation,
    },
    IndexAccess {
        object: Box<AstNode>,
        index: Box<AstNode>,
        location: SourceLocation,
    },
    FunctionCall {
        function: Box<AstNode>,
        arguments: Vec<Argument>,
        location: SourceLocation,
    },
    Assignment {
        target: Box<AstNode>,
        operator: AssignmentOperator,
        value: Box<AstNode>,
        location: SourceLocation,
    },
    If {
        condition: Box<AstNode>,
        then_branch: Box<AstNode>,
        else_ifs: Vec<(AstNode, AstNode)>,
        else_branch: Option<Box<AstNode>>,
        location: SourceLocation,
    },
    Match {
        value: Box<AstNode>,
        arms: Vec<MatchArm>,
        location: SourceLocation,
    },
    Try {
        body: Box<AstNode>,
        catch_clause: Option<Box<CatchClause>>,
        location: SourceLocation,
    },
    Block {
        statements: Vec<AstNode>,
        location: SourceLocation,
    },
    ListLiteral {
        elements: Vec<AstNode>,
        location: SourceLocation,
    },
    DictLiteral {
        entries: Vec<(AstNode, AstNode)>,
        location: SourceLocation,
    },
    AnonymousFunction {
        parameters: Vec<Parameter>,
        body: Box<AstNode>,
        location: SourceLocation,
    },

    // Statements
    ExpressionStatement {
        expression: Box<AstNode>,
        location: SourceLocation,
    },
    VariableDeclaration {
        name: String,
        value: Option<Box<AstNode>>,
        location: SourceLocation,
    },
    FunctionDeclaration {
        visibility: Visibility,
        name: String,
        parameters: Vec<Parameter>,
        body: Box<AstNode>,
        location: SourceLocation,
    },
    ClassDeclaration {
        visibility: Visibility,
        name: String,
        members: Vec<ClassMember>,
        location: SourceLocation,
    },
    ImportStatement {
        path: ModulePath,
        clause: Option<ImportClause>,
        location: SourceLocation,
    },
    WhileStatement {
        condition: Box<AstNode>,
        body: Box<AstNode>,
        location: SourceLocation,
    },
    ForStatement {
        variable: String,
        index_variable: Option<String>,
        iterable: Box<AstNode>,
        body: Box<AstNode>,
        location: SourceLocation,
    },
    MatchStatement {
        value: Box<AstNode>,
        arms: Vec<MatchArm>,
        location: SourceLocation,
    },
    TryStatement {
        body: Box<AstNode>,
        catch_clause: Option<CatchClause>,
        finally_clause: Option<Box<AstNode>>,
        location: SourceLocation,
    },
    WithStatement {
        bindings: Vec<WithBinding>,
        body: Box<AstNode>,
        location: SourceLocation,
    },
    BreakStatement {
        value: Option<Box<AstNode>>,
        location: SourceLocation,
    },
    ContinueStatement {
        location: SourceLocation,
    },
    ReturnStatement {
        value: Option<Box<AstNode>>,
        location: SourceLocation,
    },

    // Module level
    Program {
        items: Vec<AstNode>,
        location: SourceLocation,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    // Arithmetic
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
    // Comparison
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    // Logical
    And,
    Or,
    Xor,
    // Bitwise
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LeftShift,
    RightShift,
    // Membership
    In,
    Is,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Plus,
    Minus,
    Not,
    BitwiseNot,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AssignmentOperator {
    Assign,
    AddAssign,
    SubtractAssign,
    MultiplyAssign,
    DivideAssign,
    ModuloAssign,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Argument {
    pub value: AstNode,
    pub spread: bool,
    pub keyword_spread: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub default_value: Option<AstNode>,
    pub variadic: bool,
    pub keyword_variadic: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub guard: Option<AstNode>,
    pub body: AstNode,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Literal(LiteralValue),
    Variable(String),
    Wildcard,
    List(Vec<Pattern>),
    Dict(Vec<(String, Pattern)>),
    Range {
        start: AstNode,
        end: AstNode,
        inclusive: bool,
    },
    Or(Vec<Pattern>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CatchClause {
    pub variable: String,
    pub body: Box<AstNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Visibility {
    Public,
    Private,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClassMember {
    Field {
        visibility: Visibility,
        name: String,
        value: Option<AstNode>,
    },
    Method {
        visibility: Visibility,
        is_static: bool,
        declaration: AstNode, // FunctionDeclaration
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModulePath {
    pub root_type: ModulePathRoot,
    pub segments: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ModulePathRoot {
    Absolute, // Default: use math::geometry
    Super,    // use super::sibling
    Root,     // use root::top_level
}

#[derive(Debug, Clone, PartialEq)]
pub enum ImportClause {
    All,
    Named(Vec<String>),
    Single(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct WithBinding {
    pub name: String,
    pub value: AstNode,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SourceLocation {
    pub line: usize,
    pub column: usize,
    pub byte_offset: usize,
}

impl SourceLocation {
    pub fn from_token(token: &Token) -> Self {
        SourceLocation {
            line: token.line,
            column: token.column,
            byte_offset: token.byte_offset,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub location: SourceLocation,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Parse error at {}:{}: {}",
            self.location.line, self.location.column, self.message
        )
    }
}
