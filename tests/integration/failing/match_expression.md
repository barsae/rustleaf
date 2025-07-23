# Program 🔴
```rustleaf
// #[fail_quietly]
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
        Token(Eof),
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