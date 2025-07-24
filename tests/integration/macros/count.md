# Program
Status: 🟡
Assertions: 0

```rustleaf
var counter = 0;

#[macro]
fn count(ast) {

    ast
}

#[count]
fn test() {}

test();
test();
test();
// assert(counter == 3);
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
        Token(Var),
        Token(Ident, "counter"),
        Token(Equal),
        Token(Int, "0"),
        Token(Semicolon),
        Token(Hash),
        Token(LeftBracket),
        Token(Macro),
        Token(RightBracket),
        Token(Fn),
        Token(Ident, "count"),
        Token(LeftParen),
        Token(Ident, "ast"),
        Token(RightParen),
        Token(LeftBrace),
        Token(Ident, "ast"),
        Token(RightBrace),
        Token(Hash),
        Token(LeftBracket),
        Token(Ident, "count"),
        Token(RightBracket),
        Token(Fn),
        Token(Ident, "test"),
        Token(LeftParen),
        Token(RightParen),
        Token(LeftBrace),
        Token(RightBrace),
        Token(Ident, "test"),
        Token(LeftParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "test"),
        Token(LeftParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "test"),
        Token(LeftParen),
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
                    "counter",
                ),
                value: Some(
                    Literal(
                        Int(
                            0,
                        ),
                    ),
                ),
            },
            FnDecl {
                name: "test",
                params: [],
                body: Block {
                    statements: [],
                    final_expr: None,
                },
                is_pub: false,
            },
            Expression(
                FunctionCall(
                    Identifier(
                        "test",
                    ),
                    [],
                ),
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "test",
                    ),
                    [],
                ),
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "test",
                    ),
                    [],
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
            Declare(
                "counter",
                Some(
                    Literal(
                        Int(
                            0,
                        ),
                    ),
                ),
            ),
            Function(
                "test",
                [],
                Block(
                    [],
                    None,
                ),
            ),
            Call(
                Variable(
                    "test",
                ),
                [],
            ),
            Call(
                Variable(
                    "test",
                ),
                [],
            ),
        ],
        Some(
            Call(
                Variable(
                    "test",
                ),
                [],
            ),
        ),
    ),
)
```