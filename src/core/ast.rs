/// Abstract Syntax Tree definitions for RustLeaf

#[derive(Debug, Clone, PartialEq)]
pub struct Program(pub Vec<Statement>);

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Expression(Expression),
    VarDecl(String, Option<Expression>),
    FnDecl {
        name: String,
        params: Vec<String>,
        body: Vec<Statement>,
    },
    ClassDecl {
        name: String,
        members: Vec<ClassMember>,
    },
    Import(ImportSpec),
    Export(ExportSpec),
    Return(Option<Expression>),
    Break,
    Continue,
    While(Expression, Vec<Statement>),
    For {
        var: String,
        iter: Expression,
        body: Vec<Statement>,
    },
    If {
        condition: Expression,
        then_body: Vec<Statement>,
        else_body: Option<Vec<Statement>>,
    },
    Match {
        expr: Expression,
        cases: Vec<MatchCase>,
    },
    Try {
        body: Vec<Statement>,
        catch: Option<CatchClause>,
        finally: Option<Vec<Statement>>,
    },
    With {
        var: String,
        expr: Expression,
        body: Vec<Statement>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    // Literals
    Literal(LiteralValue),
    Identifier(String),

    // Core operations (post-desugaring)
    GetAttr(Box<Expression>, String),
    SetAttr(Box<Expression>, String, Box<Expression>),
    MethodCall(Box<Expression>, Vec<Expression>),

    // Control flow
    Block(Vec<Statement>),
    If {
        condition: Box<Expression>,
        then_expr: Box<Expression>,
        else_expr: Option<Box<Expression>>,
    },

    // Closures
    Lambda {
        params: Vec<String>,
        body: Box<Expression>,
    },

    // Collections
    List(Vec<Expression>),
    Dict(Vec<(Expression, Expression)>),

    // String interpolation (desugared to concat)
    Concat(Vec<Expression>),

    // Logical operations (not desugared due to short-circuit)
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    Not(Box<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
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
        params: Vec<String>,
        body: Vec<Statement>,
    },
    StaticMethod {
        params: Vec<String>,
        body: Vec<Statement>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImportSpec {
    pub module: String,
    pub items: ImportItems,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ImportItems {
    All,                   // import module
    Specific(Vec<String>), // import {a, b} from module
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExportSpec {
    pub items: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchCase {
    pub pattern: Pattern,
    pub guard: Option<Expression>,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Literal(LiteralValue),
    Variable(String),
    Wildcard,
    List(Vec<Pattern>),
    // More patterns can be added later
}

#[derive(Debug, Clone, PartialEq)]
pub struct CatchClause {
    pub var: Option<String>,
    pub body: Vec<Statement>,
}
