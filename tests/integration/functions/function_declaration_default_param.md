# Program
Status: 🟢
Assertions: 7

```rustleaf
fn greet(name = "world") { name }
fn add(x, y = 10) { x + y }
fn multiply(a = 2, b = 3) { a * b }

var greeting1 = greet();
var greeting2 = greet("Alice");
var sum1 = add(5);
var sum2 = add(5, 15);
var product1 = multiply();
var product2 = multiply(4);
var product3 = multiply(4, 7);

assert(greeting1 == "world");
assert(greeting2 == "Alice");
assert(sum1 == 15);
assert(sum2 == 20);
assert(product1 == 6);
assert(product2 == 12);
assert(product3 == 28);
```

# Output
None

# Result
```rust
Ok(
    Unit,
)
```

# Lex
```rust
Ok(
    [
        Token(Fn),
        Token(Ident, "greet"),
        Token(LeftParen),
        Token(Ident, "name"),
        Token(Equal),
        Token(String, "world"),
        Token(RightParen),
        Token(LeftBrace),
        Token(Ident, "name"),
        Token(RightBrace),
        Token(Fn),
        Token(Ident, "add"),
        Token(LeftParen),
        Token(Ident, "x"),
        Token(Comma),
        Token(Ident, "y"),
        Token(Equal),
        Token(Int, "10"),
        Token(RightParen),
        Token(LeftBrace),
        Token(Ident, "x"),
        Token(Plus),
        Token(Ident, "y"),
        Token(RightBrace),
        Token(Fn),
        Token(Ident, "multiply"),
        Token(LeftParen),
        Token(Ident, "a"),
        Token(Equal),
        Token(Int, "2"),
        Token(Comma),
        Token(Ident, "b"),
        Token(Equal),
        Token(Int, "3"),
        Token(RightParen),
        Token(LeftBrace),
        Token(Ident, "a"),
        Token(Star),
        Token(Ident, "b"),
        Token(RightBrace),
        Token(Var),
        Token(Ident, "greeting1"),
        Token(Equal),
        Token(Ident, "greet"),
        Token(LeftParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "greeting2"),
        Token(Equal),
        Token(Ident, "greet"),
        Token(LeftParen),
        Token(String, "Alice"),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "sum1"),
        Token(Equal),
        Token(Ident, "add"),
        Token(LeftParen),
        Token(Int, "5"),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "sum2"),
        Token(Equal),
        Token(Ident, "add"),
        Token(LeftParen),
        Token(Int, "5"),
        Token(Comma),
        Token(Int, "15"),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "product1"),
        Token(Equal),
        Token(Ident, "multiply"),
        Token(LeftParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "product2"),
        Token(Equal),
        Token(Ident, "multiply"),
        Token(LeftParen),
        Token(Int, "4"),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "product3"),
        Token(Equal),
        Token(Ident, "multiply"),
        Token(LeftParen),
        Token(Int, "4"),
        Token(Comma),
        Token(Int, "7"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "greeting1"),
        Token(EqualEqual),
        Token(String, "world"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "greeting2"),
        Token(EqualEqual),
        Token(String, "Alice"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "sum1"),
        Token(EqualEqual),
        Token(Int, "15"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "sum2"),
        Token(EqualEqual),
        Token(Int, "20"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "product1"),
        Token(EqualEqual),
        Token(Int, "6"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "product2"),
        Token(EqualEqual),
        Token(Int, "12"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "product3"),
        Token(EqualEqual),
        Token(Int, "28"),
        Token(RightParen),
        Token(Semicolon),
        Token(Eof),
    ],
)
```

# Parse
```rust
Ok(
    Program(
        [
            FnDecl {
                name: "greet",
                params: [
                    Parameter {
                        name: "name",
                        default: Some(
                            String(
                                "world",
                            ),
                        ),
                        kind: Regular,
                    },
                ],
                body: Block {
                    statements: [],
                    final_expr: Some(
                        Identifier(
                            "name",
                        ),
                    ),
                },
                is_pub: false,
            },
            FnDecl {
                name: "add",
                params: [
                    Parameter {
                        name: "x",
                        default: None,
                        kind: Regular,
                    },
                    Parameter {
                        name: "y",
                        default: Some(
                            Int(
                                10,
                            ),
                        ),
                        kind: Regular,
                    },
                ],
                body: Block {
                    statements: [],
                    final_expr: Some(
                        Add(
                            Identifier(
                                "x",
                            ),
                            Identifier(
                                "y",
                            ),
                        ),
                    ),
                },
                is_pub: false,
            },
            FnDecl {
                name: "multiply",
                params: [
                    Parameter {
                        name: "a",
                        default: Some(
                            Int(
                                2,
                            ),
                        ),
                        kind: Regular,
                    },
                    Parameter {
                        name: "b",
                        default: Some(
                            Int(
                                3,
                            ),
                        ),
                        kind: Regular,
                    },
                ],
                body: Block {
                    statements: [],
                    final_expr: Some(
                        Mul(
                            Identifier(
                                "a",
                            ),
                            Identifier(
                                "b",
                            ),
                        ),
                    ),
                },
                is_pub: false,
            },
            VarDecl {
                pattern: Variable(
                    "greeting1",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "greet",
                        ),
                        [],
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "greeting2",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "greet",
                        ),
                        [
                            Literal(
                                String(
                                    "Alice",
                                ),
                            ),
                        ],
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "sum1",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "add",
                        ),
                        [
                            Literal(
                                Int(
                                    5,
                                ),
                            ),
                        ],
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "sum2",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "add",
                        ),
                        [
                            Literal(
                                Int(
                                    5,
                                ),
                            ),
                            Literal(
                                Int(
                                    15,
                                ),
                            ),
                        ],
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "product1",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "multiply",
                        ),
                        [],
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "product2",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "multiply",
                        ),
                        [
                            Literal(
                                Int(
                                    4,
                                ),
                            ),
                        ],
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "product3",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "multiply",
                        ),
                        [
                            Literal(
                                Int(
                                    4,
                                ),
                            ),
                            Literal(
                                Int(
                                    7,
                                ),
                            ),
                        ],
                    ),
                ),
            },
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            Identifier(
                                "greeting1",
                            ),
                            Literal(
                                String(
                                    "world",
                                ),
                            ),
                        ),
                    ],
                ),
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            Identifier(
                                "greeting2",
                            ),
                            Literal(
                                String(
                                    "Alice",
                                ),
                            ),
                        ),
                    ],
                ),
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            Identifier(
                                "sum1",
                            ),
                            Literal(
                                Int(
                                    15,
                                ),
                            ),
                        ),
                    ],
                ),
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            Identifier(
                                "sum2",
                            ),
                            Literal(
                                Int(
                                    20,
                                ),
                            ),
                        ),
                    ],
                ),
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            Identifier(
                                "product1",
                            ),
                            Literal(
                                Int(
                                    6,
                                ),
                            ),
                        ),
                    ],
                ),
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            Identifier(
                                "product2",
                            ),
                            Literal(
                                Int(
                                    12,
                                ),
                            ),
                        ),
                    ],
                ),
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            Identifier(
                                "product3",
                            ),
                            Literal(
                                Int(
                                    28,
                                ),
                            ),
                        ),
                    ],
                ),
            ),
        ],
    ),
)
```

# Eval
```rust
Ok(
    Block(
        [
            Function(
                "greet",
                [
                    (
                        "name",
                        Some(
                            String(
                                "world",
                            ),
                        ),
                        Regular,
                    ),
                ],
                Block(
                    [],
                    Some(
                        Variable(
                            "name",
                        ),
                    ),
                ),
            ),
            Function(
                "add",
                [
                    (
                        "x",
                        None,
                        Regular,
                    ),
                    (
                        "y",
                        Some(
                            Int(
                                10,
                            ),
                        ),
                        Regular,
                    ),
                ],
                Block(
                    [],
                    Some(
                        Call(
                            GetAttr(
                                Variable(
                                    "x",
                                ),
                                "op_add",
                            ),
                            [
                                Variable(
                                    "y",
                                ),
                            ],
                        ),
                    ),
                ),
            ),
            Function(
                "multiply",
                [
                    (
                        "a",
                        Some(
                            Int(
                                2,
                            ),
                        ),
                        Regular,
                    ),
                    (
                        "b",
                        Some(
                            Int(
                                3,
                            ),
                        ),
                        Regular,
                    ),
                ],
                Block(
                    [],
                    Some(
                        Call(
                            GetAttr(
                                Variable(
                                    "a",
                                ),
                                "op_mul",
                            ),
                            [
                                Variable(
                                    "b",
                                ),
                            ],
                        ),
                    ),
                ),
            ),
            Declare(
                "greeting1",
                Some(
                    Call(
                        Variable(
                            "greet",
                        ),
                        [],
                    ),
                ),
            ),
            Declare(
                "greeting2",
                Some(
                    Call(
                        Variable(
                            "greet",
                        ),
                        [
                            Literal(
                                String(
                                    "Alice",
                                ),
                            ),
                        ],
                    ),
                ),
            ),
            Declare(
                "sum1",
                Some(
                    Call(
                        Variable(
                            "add",
                        ),
                        [
                            Literal(
                                Int(
                                    5,
                                ),
                            ),
                        ],
                    ),
                ),
            ),
            Declare(
                "sum2",
                Some(
                    Call(
                        Variable(
                            "add",
                        ),
                        [
                            Literal(
                                Int(
                                    5,
                                ),
                            ),
                            Literal(
                                Int(
                                    15,
                                ),
                            ),
                        ],
                    ),
                ),
            ),
            Declare(
                "product1",
                Some(
                    Call(
                        Variable(
                            "multiply",
                        ),
                        [],
                    ),
                ),
            ),
            Declare(
                "product2",
                Some(
                    Call(
                        Variable(
                            "multiply",
                        ),
                        [
                            Literal(
                                Int(
                                    4,
                                ),
                            ),
                        ],
                    ),
                ),
            ),
            Declare(
                "product3",
                Some(
                    Call(
                        Variable(
                            "multiply",
                        ),
                        [
                            Literal(
                                Int(
                                    4,
                                ),
                            ),
                            Literal(
                                Int(
                                    7,
                                ),
                            ),
                        ],
                    ),
                ),
            ),
            Call(
                Variable(
                    "assert",
                ),
                [
                    Call(
                        GetAttr(
                            Variable(
                                "greeting1",
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                String(
                                    "world",
                                ),
                            ),
                        ],
                    ),
                ],
            ),
            Call(
                Variable(
                    "assert",
                ),
                [
                    Call(
                        GetAttr(
                            Variable(
                                "greeting2",
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                String(
                                    "Alice",
                                ),
                            ),
                        ],
                    ),
                ],
            ),
            Call(
                Variable(
                    "assert",
                ),
                [
                    Call(
                        GetAttr(
                            Variable(
                                "sum1",
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                Int(
                                    15,
                                ),
                            ),
                        ],
                    ),
                ],
            ),
            Call(
                Variable(
                    "assert",
                ),
                [
                    Call(
                        GetAttr(
                            Variable(
                                "sum2",
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                Int(
                                    20,
                                ),
                            ),
                        ],
                    ),
                ],
            ),
            Call(
                Variable(
                    "assert",
                ),
                [
                    Call(
                        GetAttr(
                            Variable(
                                "product1",
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                Int(
                                    6,
                                ),
                            ),
                        ],
                    ),
                ],
            ),
            Call(
                Variable(
                    "assert",
                ),
                [
                    Call(
                        GetAttr(
                            Variable(
                                "product2",
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                Int(
                                    12,
                                ),
                            ),
                        ],
                    ),
                ],
            ),
        ],
        Some(
            Call(
                Variable(
                    "assert",
                ),
                [
                    Call(
                        GetAttr(
                            Variable(
                                "product3",
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                Int(
                                    28,
                                ),
                            ),
                        ],
                    ),
                ],
            ),
        ),
    ),
)
```