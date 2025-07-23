# Program

```rustleaf
match x {
    case 0 {
        "zero"
    }
    case 1 {
        "one"
    }
    case _ {
        "other"
    }
}
```

# Lex

```rust
Ok(
    [
        Token {
            token_type: Match,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "x",
            ),
        },
        Token {
            token_type: LeftBrace,
            text: None,
        },
        Token {
            token_type: Case,
            text: None,
        },
        Token {
            token_type: Int,
            text: Some(
                "0",
            ),
        },
        Token {
            token_type: LeftBrace,
            text: None,
        },
        Token {
            token_type: String,
            text: Some(
                "zero",
            ),
        },
        Token {
            token_type: RightBrace,
            text: None,
        },
        Token {
            token_type: Case,
            text: None,
        },
        Token {
            token_type: Int,
            text: Some(
                "1",
            ),
        },
        Token {
            token_type: LeftBrace,
            text: None,
        },
        Token {
            token_type: String,
            text: Some(
                "one",
            ),
        },
        Token {
            token_type: RightBrace,
            text: None,
        },
        Token {
            token_type: Case,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "_",
            ),
        },
        Token {
            token_type: LeftBrace,
            text: None,
        },
        Token {
            token_type: String,
            text: Some(
                "other",
            ),
        },
        Token {
            token_type: RightBrace,
            text: None,
        },
        Token {
            token_type: RightBrace,
            text: None,
        },
        Token {
            token_type: Eof,
            text: None,
        },
    ],
)
```

# Parse

```rust
Ok(
    Program(
        [
            Expression(
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

# Output

```

```

# Result

```rust
Err(
    "Expression not yet implemented: Match { expr: Identifier(\"x\"), cases: [MatchCase { pattern: Literal(Int(0)), guard: None, body: Block { statements: [], final_expr: Some(Literal(String(\"zero\"))) } }, MatchCase { pattern: Literal(Int(1)), guard: None, body: Block { statements: [], final_expr: Some(Literal(String(\"one\"))) } }, MatchCase { pattern: Wildcard, guard: None, body: Block { statements: [], final_expr: Some(Literal(String(\"other\"))) } }] }",
)
```
