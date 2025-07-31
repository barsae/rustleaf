# Program
Status: 🟢
Assertions: 10

```rustleaf
fn sum(*args) { args }
fn first_and_rest(first, *rest) { [first, rest] }

var empty = sum();
var single = sum(1);
var multiple = sum(1, 2, 3, 4, 5);

var mixed1 = first_and_rest(42);
var mixed2 = first_and_rest(10, 20, 30);

// Test that rest parameters collect into lists
assert(empty == []);
assert(single == [1]);
assert(multiple == [1, 2, 3, 4, 5]);

// Test mixed regular and rest parameters
assert(mixed1 == [42, []]);
assert(mixed2 == [10, [20, 30]]);

// Test individual access
assert(single[0] == 1);
assert(multiple[0] == 1);
assert(multiple[4] == 5);
assert(mixed1[0] == 42);
assert(mixed2[0] == 10);
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Fn)
parse_statement: starting at position 0 (Fn)
parse_statement: starting at position 7 (Ident(args))
parse_statement: falling back to expression statement
parse_expression: starting at position 7 (Ident(args))
parse_primary: success - parsed identifier (args)
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace
parse_expression: starting at position 7 (Ident(args))
parse_primary: success - parsed identifier (args)
parse_expression: success - parsed precedence expression
parse_statement: success - parsed function declaration
parse_program: parsing statement at position 9 (Fn)
parse_statement: starting at position 9 (Fn)
parse_statement: starting at position 18 (LeftBracket)
parse_statement: falling back to expression statement
parse_expression: starting at position 18 (LeftBracket)
parse_primary: success - parsing list literal
parse_expression: starting at position 19 (Ident(first))
parse_primary: success - parsed identifier (first)
parse_expression: success - parsed precedence expression
parse_expression: starting at position 21 (Ident(rest))
parse_primary: success - parsed identifier (rest)
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace
parse_expression: starting at position 18 (LeftBracket)
parse_primary: success - parsing list literal
parse_expression: starting at position 19 (Ident(first))
parse_primary: success - parsed identifier (first)
parse_expression: success - parsed precedence expression
parse_expression: starting at position 21 (Ident(rest))
parse_primary: success - parsed identifier (rest)
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_statement: success - parsed function declaration
parse_program: parsing statement at position 24 (Var)
parse_statement: starting at position 24 (Var)
parse_expression: starting at position 27 (Ident(sum))
parse_primary: success - parsed identifier (sum)
parse_expression: success - parsed precedence expression
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 31 (Var)
parse_statement: starting at position 31 (Var)
parse_expression: starting at position 34 (Ident(sum))
parse_primary: success - parsed identifier (sum)
parse_expression: starting at position 36 (Int(1))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 39 (Var)
parse_statement: starting at position 39 (Var)
parse_expression: starting at position 42 (Ident(sum))
parse_primary: success - parsed identifier (sum)
parse_expression: starting at position 44 (Int(1))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: starting at position 46 (Int(2))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: starting at position 48 (Int(3))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: starting at position 50 (Int(4))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: starting at position 52 (Int(5))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 55 (Var)
parse_statement: starting at position 55 (Var)
parse_expression: starting at position 58 (Ident(first_and_rest))
parse_primary: success - parsed identifier (first_and_rest)
parse_expression: starting at position 60 (Int(42))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 63 (Var)
parse_statement: starting at position 63 (Var)
parse_expression: starting at position 66 (Ident(first_and_rest))
parse_primary: success - parsed identifier (first_and_rest)
parse_expression: starting at position 68 (Int(10))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: starting at position 70 (Int(20))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: starting at position 72 (Int(30))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 75 (Ident(assert))
parse_statement: starting at position 75 (Ident(assert))
parse_statement: falling back to expression statement
parse_expression: starting at position 75 (Ident(assert))
parse_primary: success - parsed identifier (assert)
parse_expression: starting at position 77 (Ident(empty))
parse_primary: success - parsed identifier (empty)
parse_primary: success - parsing list literal
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_program: parsing statement at position 83 (Ident(assert))
parse_statement: starting at position 83 (Ident(assert))
parse_statement: falling back to expression statement
parse_expression: starting at position 83 (Ident(assert))
parse_primary: success - parsed identifier (assert)
parse_expression: starting at position 85 (Ident(single))
parse_primary: success - parsed identifier (single)
parse_primary: success - parsing list literal
parse_expression: starting at position 88 (Int(1))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_program: parsing statement at position 92 (Ident(assert))
parse_statement: starting at position 92 (Ident(assert))
parse_statement: falling back to expression statement
parse_expression: starting at position 92 (Ident(assert))
parse_primary: success - parsed identifier (assert)
parse_expression: starting at position 94 (Ident(multiple))
parse_primary: success - parsed identifier (multiple)
parse_primary: success - parsing list literal
parse_expression: starting at position 97 (Int(1))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: starting at position 99 (Int(2))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: starting at position 101 (Int(3))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: starting at position 103 (Int(4))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: starting at position 105 (Int(5))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_program: parsing statement at position 109 (Ident(assert))
parse_statement: starting at position 109 (Ident(assert))
parse_statement: falling back to expression statement
parse_expression: starting at position 109 (Ident(assert))
parse_primary: success - parsed identifier (assert)
parse_expression: starting at position 111 (Ident(mixed1))
parse_primary: success - parsed identifier (mixed1)
parse_primary: success - parsing list literal
parse_expression: starting at position 114 (Int(42))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: starting at position 116 (LeftBracket)
parse_primary: success - parsing list literal
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_program: parsing statement at position 121 (Ident(assert))
parse_statement: starting at position 121 (Ident(assert))
parse_statement: falling back to expression statement
parse_expression: starting at position 121 (Ident(assert))
parse_primary: success - parsed identifier (assert)
parse_expression: starting at position 123 (Ident(mixed2))
parse_primary: success - parsed identifier (mixed2)
parse_primary: success - parsing list literal
parse_expression: starting at position 126 (Int(10))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: starting at position 128 (LeftBracket)
parse_primary: success - parsing list literal
parse_expression: starting at position 129 (Int(20))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: starting at position 131 (Int(30))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_program: parsing statement at position 136 (Ident(assert))
parse_statement: starting at position 136 (Ident(assert))
parse_statement: falling back to expression statement
parse_expression: starting at position 136 (Ident(assert))
parse_primary: success - parsed identifier (assert)
parse_expression: starting at position 138 (Ident(single))
parse_primary: success - parsed identifier (single)
parse_expression: starting at position 140 (Int(0))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_program: parsing statement at position 146 (Ident(assert))
parse_statement: starting at position 146 (Ident(assert))
parse_statement: falling back to expression statement
parse_expression: starting at position 146 (Ident(assert))
parse_primary: success - parsed identifier (assert)
parse_expression: starting at position 148 (Ident(multiple))
parse_primary: success - parsed identifier (multiple)
parse_expression: starting at position 150 (Int(0))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_program: parsing statement at position 156 (Ident(assert))
parse_statement: starting at position 156 (Ident(assert))
parse_statement: falling back to expression statement
parse_expression: starting at position 156 (Ident(assert))
parse_primary: success - parsed identifier (assert)
parse_expression: starting at position 158 (Ident(multiple))
parse_primary: success - parsed identifier (multiple)
parse_expression: starting at position 160 (Int(4))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_program: parsing statement at position 166 (Ident(assert))
parse_statement: starting at position 166 (Ident(assert))
parse_statement: falling back to expression statement
parse_expression: starting at position 166 (Ident(assert))
parse_primary: success - parsed identifier (assert)
parse_expression: starting at position 168 (Ident(mixed1))
parse_primary: success - parsed identifier (mixed1)
parse_expression: starting at position 170 (Int(0))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_program: parsing statement at position 176 (Ident(assert))
parse_statement: starting at position 176 (Ident(assert))
parse_statement: falling back to expression statement
parse_expression: starting at position 176 (Ident(assert))
parse_primary: success - parsed identifier (assert)
parse_expression: starting at position 178 (Ident(mixed2))
parse_primary: success - parsed identifier (mixed2)
parse_expression: starting at position 180 (Int(0))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_program: parsed 17 statements
```

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
        Token(Ident, "sum"),
        Token(LeftParen),
        Token(Star),
        Token(Ident, "args"),
        Token(RightParen),
        Token(LeftBrace),
        Token(Ident, "args"),
        Token(RightBrace),
        Token(Fn),
        Token(Ident, "first_and_rest"),
        Token(LeftParen),
        Token(Ident, "first"),
        Token(Comma),
        Token(Star),
        Token(Ident, "rest"),
        Token(RightParen),
        Token(LeftBrace),
        Token(LeftBracket),
        Token(Ident, "first"),
        Token(Comma),
        Token(Ident, "rest"),
        Token(RightBracket),
        Token(RightBrace),
        Token(Var),
        Token(Ident, "empty"),
        Token(Equal),
        Token(Ident, "sum"),
        Token(LeftParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "single"),
        Token(Equal),
        Token(Ident, "sum"),
        Token(LeftParen),
        Token(Int, "1"),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "multiple"),
        Token(Equal),
        Token(Ident, "sum"),
        Token(LeftParen),
        Token(Int, "1"),
        Token(Comma),
        Token(Int, "2"),
        Token(Comma),
        Token(Int, "3"),
        Token(Comma),
        Token(Int, "4"),
        Token(Comma),
        Token(Int, "5"),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "mixed1"),
        Token(Equal),
        Token(Ident, "first_and_rest"),
        Token(LeftParen),
        Token(Int, "42"),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "mixed2"),
        Token(Equal),
        Token(Ident, "first_and_rest"),
        Token(LeftParen),
        Token(Int, "10"),
        Token(Comma),
        Token(Int, "20"),
        Token(Comma),
        Token(Int, "30"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "empty"),
        Token(EqualEqual),
        Token(LeftBracket),
        Token(RightBracket),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "single"),
        Token(EqualEqual),
        Token(LeftBracket),
        Token(Int, "1"),
        Token(RightBracket),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "multiple"),
        Token(EqualEqual),
        Token(LeftBracket),
        Token(Int, "1"),
        Token(Comma),
        Token(Int, "2"),
        Token(Comma),
        Token(Int, "3"),
        Token(Comma),
        Token(Int, "4"),
        Token(Comma),
        Token(Int, "5"),
        Token(RightBracket),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "mixed1"),
        Token(EqualEqual),
        Token(LeftBracket),
        Token(Int, "42"),
        Token(Comma),
        Token(LeftBracket),
        Token(RightBracket),
        Token(RightBracket),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "mixed2"),
        Token(EqualEqual),
        Token(LeftBracket),
        Token(Int, "10"),
        Token(Comma),
        Token(LeftBracket),
        Token(Int, "20"),
        Token(Comma),
        Token(Int, "30"),
        Token(RightBracket),
        Token(RightBracket),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "single"),
        Token(LeftBracket),
        Token(Int, "0"),
        Token(RightBracket),
        Token(EqualEqual),
        Token(Int, "1"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "multiple"),
        Token(LeftBracket),
        Token(Int, "0"),
        Token(RightBracket),
        Token(EqualEqual),
        Token(Int, "1"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "multiple"),
        Token(LeftBracket),
        Token(Int, "4"),
        Token(RightBracket),
        Token(EqualEqual),
        Token(Int, "5"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "mixed1"),
        Token(LeftBracket),
        Token(Int, "0"),
        Token(RightBracket),
        Token(EqualEqual),
        Token(Int, "42"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "mixed2"),
        Token(LeftBracket),
        Token(Int, "0"),
        Token(RightBracket),
        Token(EqualEqual),
        Token(Int, "10"),
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
                name: "sum",
                params: [
                    Parameter {
                        name: "args",
                        default: None,
                        kind: Rest,
                    },
                ],
                body: Block {
                    statements: [],
                    final_expr: Some(
                        Identifier(
                            "args",
                        ),
                    ),
                },
                is_pub: false,
            },
            FnDecl {
                name: "first_and_rest",
                params: [
                    Parameter {
                        name: "first",
                        default: None,
                        kind: Regular,
                    },
                    Parameter {
                        name: "rest",
                        default: None,
                        kind: Rest,
                    },
                ],
                body: Block {
                    statements: [],
                    final_expr: Some(
                        List(
                            [
                                Identifier(
                                    "first",
                                ),
                                Identifier(
                                    "rest",
                                ),
                            ],
                        ),
                    ),
                },
                is_pub: false,
            },
            VarDecl {
                pattern: Variable(
                    "empty",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "sum",
                        ),
                        [],
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "single",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "sum",
                        ),
                        [
                            Literal(
                                Int(
                                    1,
                                ),
                            ),
                        ],
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "multiple",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "sum",
                        ),
                        [
                            Literal(
                                Int(
                                    1,
                                ),
                            ),
                            Literal(
                                Int(
                                    2,
                                ),
                            ),
                            Literal(
                                Int(
                                    3,
                                ),
                            ),
                            Literal(
                                Int(
                                    4,
                                ),
                            ),
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
                    "mixed1",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "first_and_rest",
                        ),
                        [
                            Literal(
                                Int(
                                    42,
                                ),
                            ),
                        ],
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "mixed2",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "first_and_rest",
                        ),
                        [
                            Literal(
                                Int(
                                    10,
                                ),
                            ),
                            Literal(
                                Int(
                                    20,
                                ),
                            ),
                            Literal(
                                Int(
                                    30,
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
                                "empty",
                            ),
                            List(
                                [],
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
                                "single",
                            ),
                            List(
                                [
                                    Literal(
                                        Int(
                                            1,
                                        ),
                                    ),
                                ],
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
                                "multiple",
                            ),
                            List(
                                [
                                    Literal(
                                        Int(
                                            1,
                                        ),
                                    ),
                                    Literal(
                                        Int(
                                            2,
                                        ),
                                    ),
                                    Literal(
                                        Int(
                                            3,
                                        ),
                                    ),
                                    Literal(
                                        Int(
                                            4,
                                        ),
                                    ),
                                    Literal(
                                        Int(
                                            5,
                                        ),
                                    ),
                                ],
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
                                "mixed1",
                            ),
                            List(
                                [
                                    Literal(
                                        Int(
                                            42,
                                        ),
                                    ),
                                    List(
                                        [],
                                    ),
                                ],
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
                                "mixed2",
                            ),
                            List(
                                [
                                    Literal(
                                        Int(
                                            10,
                                        ),
                                    ),
                                    List(
                                        [
                                            Literal(
                                                Int(
                                                    20,
                                                ),
                                            ),
                                            Literal(
                                                Int(
                                                    30,
                                                ),
                                            ),
                                        ],
                                    ),
                                ],
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
                            GetItem(
                                Identifier(
                                    "single",
                                ),
                                Literal(
                                    Int(
                                        0,
                                    ),
                                ),
                            ),
                            Literal(
                                Int(
                                    1,
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
                            GetItem(
                                Identifier(
                                    "multiple",
                                ),
                                Literal(
                                    Int(
                                        0,
                                    ),
                                ),
                            ),
                            Literal(
                                Int(
                                    1,
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
                            GetItem(
                                Identifier(
                                    "multiple",
                                ),
                                Literal(
                                    Int(
                                        4,
                                    ),
                                ),
                            ),
                            Literal(
                                Int(
                                    5,
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
                            GetItem(
                                Identifier(
                                    "mixed1",
                                ),
                                Literal(
                                    Int(
                                        0,
                                    ),
                                ),
                            ),
                            Literal(
                                Int(
                                    42,
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
                            GetItem(
                                Identifier(
                                    "mixed2",
                                ),
                                Literal(
                                    Int(
                                        0,
                                    ),
                                ),
                            ),
                            Literal(
                                Int(
                                    10,
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
    RustValue(
        EvalProgram {
            statements: [
                RustValue(
                    EvalFunction {
                        data: FunctionData {
                            name: "sum",
                            params: [
                                (
                                    "args",
                                    None,
                                    Rest,
                                ),
                            ],
                            body: RustValue(
                                EvalBlock {
                                    statements: [],
                                    final_expr: Some(
                                        RustValue(
                                            EvalVariable {
                                                name: "args",
                                            },
                                        ),
                                    ),
                                },
                            ),
                        },
                    },
                ),
                RustValue(
                    EvalFunction {
                        data: FunctionData {
                            name: "first_and_rest",
                            params: [
                                (
                                    "first",
                                    None,
                                    Regular,
                                ),
                                (
                                    "rest",
                                    None,
                                    Rest,
                                ),
                            ],
                            body: RustValue(
                                EvalBlock {
                                    statements: [],
                                    final_expr: Some(
                                        RustValue(
                                            EvalList {
                                                elements: [
                                                    RustValue(
                                                        EvalVariable {
                                                            name: "first",
                                                        },
                                                    ),
                                                    RustValue(
                                                        EvalVariable {
                                                            name: "rest",
                                                        },
                                                    ),
                                                ],
                                            },
                                        ),
                                    ),
                                },
                            ),
                        },
                    },
                ),
                RustValue(
                    EvalDeclare {
                        name: "empty",
                        init_expr: Some(
                            RustValue(
                                EvalCall {
                                    func_expr: RustValue(
                                        EvalVariable {
                                            name: "sum",
                                        },
                                    ),
                                    args: [],
                                },
                            ),
                        ),
                    },
                ),
                RustValue(
                    EvalDeclare {
                        name: "single",
                        init_expr: Some(
                            RustValue(
                                EvalCall {
                                    func_expr: RustValue(
                                        EvalVariable {
                                            name: "sum",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: Int(
                                                    1,
                                                ),
                                            },
                                        ),
                                    ],
                                },
                            ),
                        ),
                    },
                ),
                RustValue(
                    EvalDeclare {
                        name: "multiple",
                        init_expr: Some(
                            RustValue(
                                EvalCall {
                                    func_expr: RustValue(
                                        EvalVariable {
                                            name: "sum",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: Int(
                                                    1,
                                                ),
                                            },
                                        ),
                                        RustValue(
                                            EvalLiteral {
                                                value: Int(
                                                    2,
                                                ),
                                            },
                                        ),
                                        RustValue(
                                            EvalLiteral {
                                                value: Int(
                                                    3,
                                                ),
                                            },
                                        ),
                                        RustValue(
                                            EvalLiteral {
                                                value: Int(
                                                    4,
                                                ),
                                            },
                                        ),
                                        RustValue(
                                            EvalLiteral {
                                                value: Int(
                                                    5,
                                                ),
                                            },
                                        ),
                                    ],
                                },
                            ),
                        ),
                    },
                ),
                RustValue(
                    EvalDeclare {
                        name: "mixed1",
                        init_expr: Some(
                            RustValue(
                                EvalCall {
                                    func_expr: RustValue(
                                        EvalVariable {
                                            name: "first_and_rest",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: Int(
                                                    42,
                                                ),
                                            },
                                        ),
                                    ],
                                },
                            ),
                        ),
                    },
                ),
                RustValue(
                    EvalDeclare {
                        name: "mixed2",
                        init_expr: Some(
                            RustValue(
                                EvalCall {
                                    func_expr: RustValue(
                                        EvalVariable {
                                            name: "first_and_rest",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: Int(
                                                    10,
                                                ),
                                            },
                                        ),
                                        RustValue(
                                            EvalLiteral {
                                                value: Int(
                                                    20,
                                                ),
                                            },
                                        ),
                                        RustValue(
                                            EvalLiteral {
                                                value: Int(
                                                    30,
                                                ),
                                            },
                                        ),
                                    ],
                                },
                            ),
                        ),
                    },
                ),
                RustValue(
                    EvalCall {
                        func_expr: RustValue(
                            EvalVariable {
                                name: "assert",
                            },
                        ),
                        args: [
                            RustValue(
                                EvalCall {
                                    func_expr: RustValue(
                                        EvalGetAttr {
                                            obj_expr: RustValue(
                                                EvalVariable {
                                                    name: "empty",
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalList {
                                                elements: [],
                                            },
                                        ),
                                    ],
                                },
                            ),
                        ],
                    },
                ),
                RustValue(
                    EvalCall {
                        func_expr: RustValue(
                            EvalVariable {
                                name: "assert",
                            },
                        ),
                        args: [
                            RustValue(
                                EvalCall {
                                    func_expr: RustValue(
                                        EvalGetAttr {
                                            obj_expr: RustValue(
                                                EvalVariable {
                                                    name: "single",
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalList {
                                                elements: [
                                                    RustValue(
                                                        EvalLiteral {
                                                            value: Int(
                                                                1,
                                                            ),
                                                        },
                                                    ),
                                                ],
                                            },
                                        ),
                                    ],
                                },
                            ),
                        ],
                    },
                ),
                RustValue(
                    EvalCall {
                        func_expr: RustValue(
                            EvalVariable {
                                name: "assert",
                            },
                        ),
                        args: [
                            RustValue(
                                EvalCall {
                                    func_expr: RustValue(
                                        EvalGetAttr {
                                            obj_expr: RustValue(
                                                EvalVariable {
                                                    name: "multiple",
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalList {
                                                elements: [
                                                    RustValue(
                                                        EvalLiteral {
                                                            value: Int(
                                                                1,
                                                            ),
                                                        },
                                                    ),
                                                    RustValue(
                                                        EvalLiteral {
                                                            value: Int(
                                                                2,
                                                            ),
                                                        },
                                                    ),
                                                    RustValue(
                                                        EvalLiteral {
                                                            value: Int(
                                                                3,
                                                            ),
                                                        },
                                                    ),
                                                    RustValue(
                                                        EvalLiteral {
                                                            value: Int(
                                                                4,
                                                            ),
                                                        },
                                                    ),
                                                    RustValue(
                                                        EvalLiteral {
                                                            value: Int(
                                                                5,
                                                            ),
                                                        },
                                                    ),
                                                ],
                                            },
                                        ),
                                    ],
                                },
                            ),
                        ],
                    },
                ),
                RustValue(
                    EvalCall {
                        func_expr: RustValue(
                            EvalVariable {
                                name: "assert",
                            },
                        ),
                        args: [
                            RustValue(
                                EvalCall {
                                    func_expr: RustValue(
                                        EvalGetAttr {
                                            obj_expr: RustValue(
                                                EvalVariable {
                                                    name: "mixed1",
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalList {
                                                elements: [
                                                    RustValue(
                                                        EvalLiteral {
                                                            value: Int(
                                                                42,
                                                            ),
                                                        },
                                                    ),
                                                    RustValue(
                                                        EvalList {
                                                            elements: [],
                                                        },
                                                    ),
                                                ],
                                            },
                                        ),
                                    ],
                                },
                            ),
                        ],
                    },
                ),
                RustValue(
                    EvalCall {
                        func_expr: RustValue(
                            EvalVariable {
                                name: "assert",
                            },
                        ),
                        args: [
                            RustValue(
                                EvalCall {
                                    func_expr: RustValue(
                                        EvalGetAttr {
                                            obj_expr: RustValue(
                                                EvalVariable {
                                                    name: "mixed2",
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalList {
                                                elements: [
                                                    RustValue(
                                                        EvalLiteral {
                                                            value: Int(
                                                                10,
                                                            ),
                                                        },
                                                    ),
                                                    RustValue(
                                                        EvalList {
                                                            elements: [
                                                                RustValue(
                                                                    EvalLiteral {
                                                                        value: Int(
                                                                            20,
                                                                        ),
                                                                    },
                                                                ),
                                                                RustValue(
                                                                    EvalLiteral {
                                                                        value: Int(
                                                                            30,
                                                                        ),
                                                                    },
                                                                ),
                                                            ],
                                                        },
                                                    ),
                                                ],
                                            },
                                        ),
                                    ],
                                },
                            ),
                        ],
                    },
                ),
                RustValue(
                    EvalCall {
                        func_expr: RustValue(
                            EvalVariable {
                                name: "assert",
                            },
                        ),
                        args: [
                            RustValue(
                                EvalCall {
                                    func_expr: RustValue(
                                        EvalGetAttr {
                                            obj_expr: RustValue(
                                                EvalGetItem {
                                                    obj_expr: RustValue(
                                                        EvalVariable {
                                                            name: "single",
                                                        },
                                                    ),
                                                    index_expr: RustValue(
                                                        EvalLiteral {
                                                            value: Int(
                                                                0,
                                                            ),
                                                        },
                                                    ),
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: Int(
                                                    1,
                                                ),
                                            },
                                        ),
                                    ],
                                },
                            ),
                        ],
                    },
                ),
                RustValue(
                    EvalCall {
                        func_expr: RustValue(
                            EvalVariable {
                                name: "assert",
                            },
                        ),
                        args: [
                            RustValue(
                                EvalCall {
                                    func_expr: RustValue(
                                        EvalGetAttr {
                                            obj_expr: RustValue(
                                                EvalGetItem {
                                                    obj_expr: RustValue(
                                                        EvalVariable {
                                                            name: "multiple",
                                                        },
                                                    ),
                                                    index_expr: RustValue(
                                                        EvalLiteral {
                                                            value: Int(
                                                                0,
                                                            ),
                                                        },
                                                    ),
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: Int(
                                                    1,
                                                ),
                                            },
                                        ),
                                    ],
                                },
                            ),
                        ],
                    },
                ),
                RustValue(
                    EvalCall {
                        func_expr: RustValue(
                            EvalVariable {
                                name: "assert",
                            },
                        ),
                        args: [
                            RustValue(
                                EvalCall {
                                    func_expr: RustValue(
                                        EvalGetAttr {
                                            obj_expr: RustValue(
                                                EvalGetItem {
                                                    obj_expr: RustValue(
                                                        EvalVariable {
                                                            name: "multiple",
                                                        },
                                                    ),
                                                    index_expr: RustValue(
                                                        EvalLiteral {
                                                            value: Int(
                                                                4,
                                                            ),
                                                        },
                                                    ),
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: Int(
                                                    5,
                                                ),
                                            },
                                        ),
                                    ],
                                },
                            ),
                        ],
                    },
                ),
                RustValue(
                    EvalCall {
                        func_expr: RustValue(
                            EvalVariable {
                                name: "assert",
                            },
                        ),
                        args: [
                            RustValue(
                                EvalCall {
                                    func_expr: RustValue(
                                        EvalGetAttr {
                                            obj_expr: RustValue(
                                                EvalGetItem {
                                                    obj_expr: RustValue(
                                                        EvalVariable {
                                                            name: "mixed1",
                                                        },
                                                    ),
                                                    index_expr: RustValue(
                                                        EvalLiteral {
                                                            value: Int(
                                                                0,
                                                            ),
                                                        },
                                                    ),
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: Int(
                                                    42,
                                                ),
                                            },
                                        ),
                                    ],
                                },
                            ),
                        ],
                    },
                ),
                RustValue(
                    EvalCall {
                        func_expr: RustValue(
                            EvalVariable {
                                name: "assert",
                            },
                        ),
                        args: [
                            RustValue(
                                EvalCall {
                                    func_expr: RustValue(
                                        EvalGetAttr {
                                            obj_expr: RustValue(
                                                EvalGetItem {
                                                    obj_expr: RustValue(
                                                        EvalVariable {
                                                            name: "mixed2",
                                                        },
                                                    ),
                                                    index_expr: RustValue(
                                                        EvalLiteral {
                                                            value: Int(
                                                                0,
                                                            ),
                                                        },
                                                    ),
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: Int(
                                                    10,
                                                ),
                                            },
                                        ),
                                    ],
                                },
                            ),
                        ],
                    },
                ),
            ],
        },
    ),
)
```