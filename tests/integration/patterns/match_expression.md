# Program
Status: 🟢
Assertions: 3

```rustleaf
var x = 1;
var result = match x {
    0: {
        "zero"
    }
    1: {
        "one"
    }
    _: {
        "other"
    }
};
assert(result == "one");

var y = 42;
var result2 = match y {
    0: {
        "zero"
    }
    1: {
        "one"
    }
    _: {
        "other"
    }
};
assert(result2 == "other");

var z = 0;
var result3 = match z {
    0: {
        "zero"
    }
    1: {
        "one"
    }
    _: {
        "other"
    }
};
assert(result3 == "zero");
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Var)
parse_statement: starting at position 0 (Var)
parse_expression: starting at position 3 (Int(1))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 5 (Var)
parse_statement: starting at position 5 (Var)
parse_expression: starting at position 8 (Match)
parse_primary: success - parsing match expression
parse_expression: starting at position 9 (Ident(x))
parse_primary: success - parsed identifier (x)
parse_expression: success - parsed precedence expression
parse_statement: starting at position 14 (String(zero))
parse_statement: falling back to expression statement
parse_expression: starting at position 14 (String(zero))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace
parse_expression: starting at position 14 (String(zero))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: starting at position 19 (String(one))
parse_statement: falling back to expression statement
parse_expression: starting at position 19 (String(one))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace
parse_expression: starting at position 19 (String(one))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: starting at position 24 (String(other))
parse_statement: falling back to expression statement
parse_expression: starting at position 24 (String(other))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace
parse_expression: starting at position 24 (String(other))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 28 (Ident(assert))
parse_statement: starting at position 28 (Ident(assert))
parse_statement: falling back to expression statement
parse_expression: starting at position 28 (Ident(assert))
parse_primary: success - parsed identifier (assert)
parse_expression: starting at position 30 (Ident(result))
parse_primary: success - parsed identifier (result)
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_program: parsing statement at position 35 (Var)
parse_statement: starting at position 35 (Var)
parse_expression: starting at position 38 (Int(42))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 40 (Var)
parse_statement: starting at position 40 (Var)
parse_expression: starting at position 43 (Match)
parse_primary: success - parsing match expression
parse_expression: starting at position 44 (Ident(y))
parse_primary: success - parsed identifier (y)
parse_expression: success - parsed precedence expression
parse_statement: starting at position 49 (String(zero))
parse_statement: falling back to expression statement
parse_expression: starting at position 49 (String(zero))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace
parse_expression: starting at position 49 (String(zero))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: starting at position 54 (String(one))
parse_statement: falling back to expression statement
parse_expression: starting at position 54 (String(one))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace
parse_expression: starting at position 54 (String(one))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: starting at position 59 (String(other))
parse_statement: falling back to expression statement
parse_expression: starting at position 59 (String(other))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace
parse_expression: starting at position 59 (String(other))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 63 (Ident(assert))
parse_statement: starting at position 63 (Ident(assert))
parse_statement: falling back to expression statement
parse_expression: starting at position 63 (Ident(assert))
parse_primary: success - parsed identifier (assert)
parse_expression: starting at position 65 (Ident(result2))
parse_primary: success - parsed identifier (result2)
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_program: parsing statement at position 70 (Var)
parse_statement: starting at position 70 (Var)
parse_expression: starting at position 73 (Int(0))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 75 (Var)
parse_statement: starting at position 75 (Var)
parse_expression: starting at position 78 (Match)
parse_primary: success - parsing match expression
parse_expression: starting at position 79 (Ident(z))
parse_primary: success - parsed identifier (z)
parse_expression: success - parsed precedence expression
parse_statement: starting at position 84 (String(zero))
parse_statement: falling back to expression statement
parse_expression: starting at position 84 (String(zero))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace
parse_expression: starting at position 84 (String(zero))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: starting at position 89 (String(one))
parse_statement: falling back to expression statement
parse_expression: starting at position 89 (String(one))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace
parse_expression: starting at position 89 (String(one))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: starting at position 94 (String(other))
parse_statement: falling back to expression statement
parse_expression: starting at position 94 (String(other))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace
parse_expression: starting at position 94 (String(other))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 98 (Ident(assert))
parse_statement: starting at position 98 (Ident(assert))
parse_statement: falling back to expression statement
parse_expression: starting at position 98 (Ident(assert))
parse_primary: success - parsed identifier (assert)
parse_expression: starting at position 100 (Ident(result3))
parse_primary: success - parsed identifier (result3)
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_program: parsed 9 statements
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
        Token(Var),
        Token(Ident, "x"),
        Token(Equal),
        Token(Int, "1"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "result"),
        Token(Equal),
        Token(Match),
        Token(Ident, "x"),
        Token(LeftBrace),
        Token(Int, "0"),
        Token(Colon),
        Token(LeftBrace),
        Token(String, "zero"),
        Token(RightBrace),
        Token(Int, "1"),
        Token(Colon),
        Token(LeftBrace),
        Token(String, "one"),
        Token(RightBrace),
        Token(Ident, "_"),
        Token(Colon),
        Token(LeftBrace),
        Token(String, "other"),
        Token(RightBrace),
        Token(RightBrace),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result"),
        Token(EqualEqual),
        Token(String, "one"),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "y"),
        Token(Equal),
        Token(Int, "42"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "result2"),
        Token(Equal),
        Token(Match),
        Token(Ident, "y"),
        Token(LeftBrace),
        Token(Int, "0"),
        Token(Colon),
        Token(LeftBrace),
        Token(String, "zero"),
        Token(RightBrace),
        Token(Int, "1"),
        Token(Colon),
        Token(LeftBrace),
        Token(String, "one"),
        Token(RightBrace),
        Token(Ident, "_"),
        Token(Colon),
        Token(LeftBrace),
        Token(String, "other"),
        Token(RightBrace),
        Token(RightBrace),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result2"),
        Token(EqualEqual),
        Token(String, "other"),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "z"),
        Token(Equal),
        Token(Int, "0"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "result3"),
        Token(Equal),
        Token(Match),
        Token(Ident, "z"),
        Token(LeftBrace),
        Token(Int, "0"),
        Token(Colon),
        Token(LeftBrace),
        Token(String, "zero"),
        Token(RightBrace),
        Token(Int, "1"),
        Token(Colon),
        Token(LeftBrace),
        Token(String, "one"),
        Token(RightBrace),
        Token(Ident, "_"),
        Token(Colon),
        Token(LeftBrace),
        Token(String, "other"),
        Token(RightBrace),
        Token(RightBrace),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result3"),
        Token(EqualEqual),
        Token(String, "zero"),
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
            VarDecl {
                pattern: Variable(
                    "x",
                ),
                value: Some(
                    Literal(
                        Int(
                            1,
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "result",
                ),
                value: Some(
                    Match {
                        expr: Identifier(
                            "x",
                        ),
                        cases: [
                            MatchCase {
                                pattern: Literal(
                                    Int(
                                        0,
                                    ),
                                ),
                                guard: None,
                                body: Block {
                                    statements: [],
                                    final_expr: Some(
                                        Literal(
                                            String(
                                                "zero",
                                            ),
                                        ),
                                    ),
                                },
                            },
                            MatchCase {
                                pattern: Literal(
                                    Int(
                                        1,
                                    ),
                                ),
                                guard: None,
                                body: Block {
                                    statements: [],
                                    final_expr: Some(
                                        Literal(
                                            String(
                                                "one",
                                            ),
                                        ),
                                    ),
                                },
                            },
                            MatchCase {
                                pattern: Wildcard,
                                guard: None,
                                body: Block {
                                    statements: [],
                                    final_expr: Some(
                                        Literal(
                                            String(
                                                "other",
                                            ),
                                        ),
                                    ),
                                },
                            },
                        ],
                    },
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
                                "result",
                            ),
                            Literal(
                                String(
                                    "one",
                                ),
                            ),
                        ),
                    ],
                ),
            ),
            VarDecl {
                pattern: Variable(
                    "y",
                ),
                value: Some(
                    Literal(
                        Int(
                            42,
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "result2",
                ),
                value: Some(
                    Match {
                        expr: Identifier(
                            "y",
                        ),
                        cases: [
                            MatchCase {
                                pattern: Literal(
                                    Int(
                                        0,
                                    ),
                                ),
                                guard: None,
                                body: Block {
                                    statements: [],
                                    final_expr: Some(
                                        Literal(
                                            String(
                                                "zero",
                                            ),
                                        ),
                                    ),
                                },
                            },
                            MatchCase {
                                pattern: Literal(
                                    Int(
                                        1,
                                    ),
                                ),
                                guard: None,
                                body: Block {
                                    statements: [],
                                    final_expr: Some(
                                        Literal(
                                            String(
                                                "one",
                                            ),
                                        ),
                                    ),
                                },
                            },
                            MatchCase {
                                pattern: Wildcard,
                                guard: None,
                                body: Block {
                                    statements: [],
                                    final_expr: Some(
                                        Literal(
                                            String(
                                                "other",
                                            ),
                                        ),
                                    ),
                                },
                            },
                        ],
                    },
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
                                "result2",
                            ),
                            Literal(
                                String(
                                    "other",
                                ),
                            ),
                        ),
                    ],
                ),
            ),
            VarDecl {
                pattern: Variable(
                    "z",
                ),
                value: Some(
                    Literal(
                        Int(
                            0,
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "result3",
                ),
                value: Some(
                    Match {
                        expr: Identifier(
                            "z",
                        ),
                        cases: [
                            MatchCase {
                                pattern: Literal(
                                    Int(
                                        0,
                                    ),
                                ),
                                guard: None,
                                body: Block {
                                    statements: [],
                                    final_expr: Some(
                                        Literal(
                                            String(
                                                "zero",
                                            ),
                                        ),
                                    ),
                                },
                            },
                            MatchCase {
                                pattern: Literal(
                                    Int(
                                        1,
                                    ),
                                ),
                                guard: None,
                                body: Block {
                                    statements: [],
                                    final_expr: Some(
                                        Literal(
                                            String(
                                                "one",
                                            ),
                                        ),
                                    ),
                                },
                            },
                            MatchCase {
                                pattern: Wildcard,
                                guard: None,
                                body: Block {
                                    statements: [],
                                    final_expr: Some(
                                        Literal(
                                            String(
                                                "other",
                                            ),
                                        ),
                                    ),
                                },
                            },
                        ],
                    },
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
                                "result3",
                            ),
                            Literal(
                                String(
                                    "zero",
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
                    EvalDeclare {
                        name: "x",
                        init_expr: Some(
                            RustValue(
                                EvalLiteral {
                                    value: Int(
                                        1,
                                    ),
                                },
                            ),
                        ),
                    },
                ),
                RustValue(
                    EvalDeclare {
                        name: "result",
                        init_expr: Some(
                            RustValue(
                                EvalMatch {
                                    data: MatchData {
                                        expr: RustValue(
                                            EvalVariable {
                                                name: "x",
                                            },
                                        ),
                                        cases: [
                                            EvalMatchCase {
                                                pattern: Literal(
                                                    Int(
                                                        0,
                                                    ),
                                                ),
                                                guard: None,
                                                body: RustValue(
                                                    EvalBlock {
                                                        statements: [],
                                                        final_expr: Some(
                                                            RustValue(
                                                                EvalLiteral {
                                                                    value: String(
                                                                        "zero",
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            },
                                            EvalMatchCase {
                                                pattern: Literal(
                                                    Int(
                                                        1,
                                                    ),
                                                ),
                                                guard: None,
                                                body: RustValue(
                                                    EvalBlock {
                                                        statements: [],
                                                        final_expr: Some(
                                                            RustValue(
                                                                EvalLiteral {
                                                                    value: String(
                                                                        "one",
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            },
                                            EvalMatchCase {
                                                pattern: Wildcard,
                                                guard: None,
                                                body: RustValue(
                                                    EvalBlock {
                                                        statements: [],
                                                        final_expr: Some(
                                                            RustValue(
                                                                EvalLiteral {
                                                                    value: String(
                                                                        "other",
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            },
                                        ],
                                    },
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
                                                    name: "result",
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: String(
                                                    "one",
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
                    EvalDeclare {
                        name: "y",
                        init_expr: Some(
                            RustValue(
                                EvalLiteral {
                                    value: Int(
                                        42,
                                    ),
                                },
                            ),
                        ),
                    },
                ),
                RustValue(
                    EvalDeclare {
                        name: "result2",
                        init_expr: Some(
                            RustValue(
                                EvalMatch {
                                    data: MatchData {
                                        expr: RustValue(
                                            EvalVariable {
                                                name: "y",
                                            },
                                        ),
                                        cases: [
                                            EvalMatchCase {
                                                pattern: Literal(
                                                    Int(
                                                        0,
                                                    ),
                                                ),
                                                guard: None,
                                                body: RustValue(
                                                    EvalBlock {
                                                        statements: [],
                                                        final_expr: Some(
                                                            RustValue(
                                                                EvalLiteral {
                                                                    value: String(
                                                                        "zero",
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            },
                                            EvalMatchCase {
                                                pattern: Literal(
                                                    Int(
                                                        1,
                                                    ),
                                                ),
                                                guard: None,
                                                body: RustValue(
                                                    EvalBlock {
                                                        statements: [],
                                                        final_expr: Some(
                                                            RustValue(
                                                                EvalLiteral {
                                                                    value: String(
                                                                        "one",
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            },
                                            EvalMatchCase {
                                                pattern: Wildcard,
                                                guard: None,
                                                body: RustValue(
                                                    EvalBlock {
                                                        statements: [],
                                                        final_expr: Some(
                                                            RustValue(
                                                                EvalLiteral {
                                                                    value: String(
                                                                        "other",
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            },
                                        ],
                                    },
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
                                                    name: "result2",
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: String(
                                                    "other",
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
                    EvalDeclare {
                        name: "z",
                        init_expr: Some(
                            RustValue(
                                EvalLiteral {
                                    value: Int(
                                        0,
                                    ),
                                },
                            ),
                        ),
                    },
                ),
                RustValue(
                    EvalDeclare {
                        name: "result3",
                        init_expr: Some(
                            RustValue(
                                EvalMatch {
                                    data: MatchData {
                                        expr: RustValue(
                                            EvalVariable {
                                                name: "z",
                                            },
                                        ),
                                        cases: [
                                            EvalMatchCase {
                                                pattern: Literal(
                                                    Int(
                                                        0,
                                                    ),
                                                ),
                                                guard: None,
                                                body: RustValue(
                                                    EvalBlock {
                                                        statements: [],
                                                        final_expr: Some(
                                                            RustValue(
                                                                EvalLiteral {
                                                                    value: String(
                                                                        "zero",
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            },
                                            EvalMatchCase {
                                                pattern: Literal(
                                                    Int(
                                                        1,
                                                    ),
                                                ),
                                                guard: None,
                                                body: RustValue(
                                                    EvalBlock {
                                                        statements: [],
                                                        final_expr: Some(
                                                            RustValue(
                                                                EvalLiteral {
                                                                    value: String(
                                                                        "one",
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            },
                                            EvalMatchCase {
                                                pattern: Wildcard,
                                                guard: None,
                                                body: RustValue(
                                                    EvalBlock {
                                                        statements: [],
                                                        final_expr: Some(
                                                            RustValue(
                                                                EvalLiteral {
                                                                    value: String(
                                                                        "other",
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            },
                                        ],
                                    },
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
                                                    name: "result3",
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: String(
                                                    "zero",
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