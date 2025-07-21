# RustLeaf Architecture

## Overview

RustLeaf is a dynamically-typed, expression-oriented scripting language. This is a reference interpreter prioritizing simplicity and spec compliance over performance.

## Design Principles

1. **Simplicity First**: Easiest correct implementation
2. **Direct Spec Mapping**: Code structure follows specification
3. **Consume Inputs**: Lexer and Parser consume their inputs entirely
4. **Clear Data Flow**: Simple pipeline with no hidden state

## Architecture

```
main.rs (CLI entry point)
    ↓
run(source: String) -> Result<(), String>
    ↓
┌─────────────────────────────────────────┐
│ lexer::scan(source: String) -> Vec<Token> │
└─────────────────────────────────────────┘
    ↓ (consumes source)
┌─────────────────────────────────────────┐
│ parser::parse(tokens: Vec<Token>) -> Ast  │
└─────────────────────────────────────────┘
    ↓ (consumes tokens)
┌─────────────────────────────────────────┐
│ eval::evaluate(ast: Ast) -> Value         │
└─────────────────────────────────────────┘
```

## Module Structure

```
src/
├── main.rs           # CLI entry point
├── lib.rs            # Re-exports public API
├── lexer.rs          # Tokenization (consumes String → Vec<Token>)
├── parser.rs         # Parsing (consumes Vec<Token> → Ast)  
├── ast.rs            # AST definitions
├── eval.rs           # Evaluation (walks Ast → Value)
├── value.rs          # Value enum and operations
├── env.rs            # Variable scoping
├── builtins.rs      # Built-in functions
└── error.rs          # Error types
```

## Core Types

### Token (lexer.rs)
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    Int(i64),
    Float(f64),
    String(String),
    StringPart(String),    // For interpolation: "hello " in "hello ${name}"
    True, False, Null,
    
    // Identifiers and Keywords
    Ident(String),
    Var, Fn, If, Else, While, For, Return, Break, Continue,
    Class, Static, Self, Import, Export, As, From,
    Match, Case, Try, Catch, Finally, With,
    And, Or, Xor, Not, In, Is,
    
    // Operators
    Plus, Minus, Star, Slash, Percent, StarStar,
    Equal, PlusEqual, MinusEqual, StarEqual, SlashEqual,
    EqualEqual, BangEqual, Less, Greater, LessEqual, GreaterEqual,
    Ampersand, Pipe, Caret, Tilde, LessLess, GreaterGreater,
    
    // Delimiters
    LeftParen, RightParen, LeftBrace, RightBrace, LeftBracket, RightBracket,
    Comma, Semicolon, Dot, Colon, DoubleColon, Arrow,
    DollarBrace,  // ${ for string interpolation
    
    Eof,
}
```

### AST (ast.rs)
```rust
pub struct Program(pub Vec<Statement>);

pub enum Statement {
    Expression(Expression),
    VarDecl(String, Option<Expression>),
    FnDecl(String, Vec<String>, Vec<Statement>),
    ClassDecl(String, Vec<ClassMember>),
    Import(ImportSpec),
    Export(ExportSpec),
    Return(Option<Expression>),
    Break,
    Continue,
    While(Expression, Vec<Statement>),
    For(String, Expression, Vec<Statement>),
    If(Expression, Vec<Statement>, Option<Vec<Statement>>),
    Match(Expression, Vec<MatchCase>),
    Try(Vec<Statement>, Option<CatchClause>, Option<Vec<Statement>>),
    With(String, Expression, Vec<Statement>),
}

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
    If(Box<Expression>, Box<Expression>, Option<Box<Expression>>),
    
    // Closures
    Lambda(Vec<String>, Box<Expression>),
    
    // Collections
    List(Vec<Expression>),
    Dict(Vec<(Expression, Expression)>),
    
    // String interpolation (desugared to concat)
    Concat(Vec<Expression>),
}

pub enum LiteralValue {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
}
```

### Value (value.rs)
```rust
#[derive(Clone, Debug)]
pub enum Value {
    Null,
    Unit,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    List(Rc<RefCell<Vec<Value>>>),
    Dict(Rc<RefCell<IndexMap<String, Value>>>),
    Function(Function),
    Object(Rc<RefCell<Object>>),
    RustValue(Box<dyn RustValue>),
}

pub trait RustValue: Debug {
    fn type_name(&self) -> &str;
    fn get_attr(&self, name: &str) -> Option<Value>;
    fn set_attr(&mut self, name: &str, value: Value) -> Result<(), String>;
}
```

## Implementation Approach

### 1. Lexer (lexer.rs)
- Single function: `pub fn scan(source: String) -> Result<Vec<Token>, String>`
- Consumes entire source string
- Handles string interpolation: `"x=${y}"` → `[StringPart("x="), DollarBrace, Ident("y"), RightBrace, StringPart("")]`
- Returns all tokens or error

### 2. Parser (parser.rs)
- Single function: `pub fn parse(tokens: Vec<Token>) -> Result<Program, String>`
- Recursive descent with operator desugaring
- Examples of desugaring:
  ```rust
  // Input tokens → AST
  a + b              → MethodCall(GetAttr(a, "op_add"), [b])
  a.foo              → GetAttr(a, "foo")
  a.foo = b          → SetAttr(a, "foo", b)
  a[i]               → MethodCall(GetAttr(a, "op_get_item"), [i])
  a[i] = b           → MethodCall(GetAttr(a, "op_set_item"), [i, b])
  -a                 → MethodCall(GetAttr(a, "op_neg"), [])
  a == b             → MethodCall(GetAttr(a, "op_eq"), [b])
  str(a)             → MethodCall(GetAttr(a, "op_str"), [])
  "hi ${name}!"      → Concat([String("hi "), MethodCall(GetAttr(name, "op_str"), []), String("!")])
  ```

### 3. Evaluator (eval.rs)
- Entry: `pub fn evaluate(program: Program) -> Result<Value, String>`
- Only needs to handle:
  - Literals and identifiers
  - GetAttr (unified attribute access)
  - SetAttr (unified attribute assignment)
  - MethodCall (unified function/method calls)
  - Control flow nodes
- Type implementation through uniform interface:
  ```rust
  trait TypeOps {
      fn get_attr(&self, name: &str) -> Option<Value>;
  }
  ```

## Key Decisions

1. **Operator Desugaring in Parser**: Simplifies evaluator implementation
   - `a + b` → `MethodCall(GetAttr(a, "op_add"), [b])` in AST
   - `a.foo` → `GetAttr(a, "foo")`
   - `a[i]` → `MethodCall(GetAttr(a, "op_get_item"), [i])`
   - Evaluator only handles GetAttr, Call, and basic nodes

2. **Uniform Type Interface**: All types use same attribute resolution
   - Built-in types implement same interface as user types
   - Single code path for all attribute access
   - No special cases in evaluator

3. **Module System in Architecture**: Plan for it from the start
   - Include Import/Export in AST from beginning
   - Module resolution designed upfront
   - Avoids major refactoring later

4. **op_get_attr Bootstrap**: Resolved in Rust implementation
   - When implementing types in Rust, we directly return methods
   - No recursive lookup needed - breaks recursion naturally

5. **String Interpolation in Lexer**: Industry standard approach
   - `"Hello ${name}"` → tokens: `[StringPart("Hello "), DollarBrace, Ident("name"), RightBrace, StringPart("")]`
   - Parser creates concatenation expression
   - No runtime string parsing

## Testing Strategy

```rust
// Lexer test
#[test]
fn test_lexer_simple() {
    let tokens = lexer::scan("42 + 3".to_string()).unwrap();
    assert_eq!(tokens, vec![
        Token::Int(42),
        Token::Plus,
        Token::Int(3),
        Token::Eof
    ]);
}

// Parser test
#[test]
fn test_parser_simple() {
    let tokens = vec![Token::Int(42), Token::Eof];
    let ast = parser::parse(tokens).unwrap();
    // assert AST structure
}

// Eval test
#[test]
fn test_eval_simple() {
    let program = Program(vec![
        Statement::Expression(Expression::Literal(LiteralValue::Int(42)))
    ]);
    let result = eval::evaluate(program).unwrap();
    assert_eq!(result, Value::Int(42));
}
```

## Implementation Phases

1. **Phase 1: Core Infrastructure**
   - Basic types: Token, AST, Value, LiteralValue
   - TypeOps trait and built-in type implementations
   - Lexer: integers, identifiers, basic operators
   - Parser: literals, identifiers, GetAttr, MethodCall
   - Evaluator: GetAttr/MethodCall dispatch
   - Goal: `42` evaluates to `Value::Int(42)`

2. **Phase 2: Arithmetic via Methods**
   - Implement op_add, op_sub, op_mul, op_div for Int/Float
   - Parser desugaring: `2 + 3` → `MethodCall(GetAttr(2, "op_add"), [3])`
   - Goal: `2 + 3 * 4` → `14` (with precedence)

3. **Phase 3: Variables and Assignment**
   - Environment structure
   - Parser: var declarations, assignments
   - SetAttr implementation
   - Goal: `var x = 5; x = x + 3; x` → `8`

4. **Phase 4: Functions and Calls**
   - Function values in Value enum
   - fn declarations, return statements
   - Closure capture
   - Goal: `fn add(a, b) { a + b } add(2, 3)` → `5`

5. **Phase 5: Control Flow**
   - if/else expressions and statements
   - while, for loops
   - break, continue

6. **Phase 6: Collections and Methods**
   - List, Dict types with methods
   - String methods and interpolation
   - Iterator protocol

7. **Phase 7: Classes and Objects**
   - Class declarations
   - Object instantiation
   - Method dispatch on objects

8. **Phase 8: Advanced Features**
   - Modules (import/export)
   - Pattern matching
   - Error handling (try/catch)
   - Macros

## Example Evaluation Flow

```rust
// Source: "hello".len()
// Tokens: [String("hello"), Dot, Ident("len"), LeftParen, RightParen]
// AST: MethodCall(GetAttr(Literal(String("hello")), "len"), [])

// Evaluation:
1. Eval GetAttr(Literal(String("hello")), "len")
   - Create Value::String("hello")
   - Call string_type_ops.get_attr("len")
   - Returns Value::BuiltinMethod(StringLen)

2. Eval MethodCall(Value::BuiltinMethod(StringLen), [])
   - Call StringLen.call([], context)
   - Returns Value::Int(5)
```

Ready to start implementation?