# Program
Status: 🔴
Assertions: 0

```rustleaf
var x = 1;
var result = match x {
    case 0 {
        "zero"
    }
    case 1 {
        "one"
    }
    case _ {
        "other"
    }
};
assert(result == "one");

var y = 42;
var result2 = match y {
    case 0 {
        "zero"
    }
    case 1 {
        "one"
    }
    case _ {
        "other"
    }
};
assert(result2 == "other");

var z = 0;
var result3 = match z {
    case 0 {
        "zero"
    }
    case 1 {
        "one"
    }
    case _ {
        "other"
    }
};
assert(result3 == "zero");
```

# Output
None

# Result
```rust
Err(
    "Expression not yet implemented: Match { expr: Identifier(\"x\"), cases: [MatchCase { pattern: Literal(Int(0)), guard: None, body: Block { statements: [], final_expr: Some(Literal(String(\"zero\"))) } }, MatchCase { pattern: Literal(Int(1)), guard: None, body: Block { statements: [], final_expr: Some(Literal(String(\"one\"))) } }, MatchCase { pattern: Wildcard, guard: None, body: Block { statements: [], final_expr: Some(Literal(String(\"other\"))) } }] }",
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
        Token(Case),
        Token(Int, "0"),
        Token(LeftBrace),
        Token(String, "zero"),
        Token(RightBrace),
        Token(Case),
        Token(Int, "1"),
        Token(LeftBrace),
        Token(String, "one"),
        Token(RightBrace),
        Token(Case),
        Token(Ident, "_"),
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
        Token(Case),
        Token(Int, "0"),
        Token(LeftBrace),
        Token(String, "zero"),
        Token(RightBrace),
        Token(Case),
        Token(Int, "1"),
        Token(LeftBrace),
        Token(String, "one"),
        Token(RightBrace),
        Token(Case),
        Token(Ident, "_"),
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
        Token(Case),
        Token(Int, "0"),
        Token(LeftBrace),
        Token(String, "zero"),
        Token(RightBrace),
        Token(Case),
        Token(Int, "1"),
        Token(LeftBrace),
        Token(String, "one"),
        Token(RightBrace),
        Token(Case),
        Token(Ident, "_"),
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
Err(
    "Expression not yet implemented: Match { expr: Identifier(\"x\"), cases: [MatchCase { pattern: Literal(Int(0)), guard: None, body: Block { statements: [], final_expr: Some(Literal(String(\"zero\"))) } }, MatchCase { pattern: Literal(Int(1)), guard: None, body: Block { statements: [], final_expr: Some(Literal(String(\"one\"))) } }, MatchCase { pattern: Wildcard, guard: None, body: Block { statements: [], final_expr: Some(Literal(String(\"other\"))) } }] }",
)
```