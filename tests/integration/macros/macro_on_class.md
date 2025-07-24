# Program
Status: 🟡
Assertions: 0

```rustleaf
#[serializable]
class User {
    var name;
    var email;
}
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
        Token(Hash),
        Token(LeftBracket),
        Token(Ident, "serializable"),
        Token(RightBracket),
        Token(Class),
        Token(Ident, "User"),
        Token(LeftBrace),
        Token(Var),
        Token(Ident, "name"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "email"),
        Token(Semicolon),
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
            ClassDecl {
                name: "User",
                members: [
                    ClassMember {
                        name: "name",
                        kind: Field(
                            None,
                        ),
                    },
                    ClassMember {
                        name: "email",
                        kind: Field(
                            None,
                        ),
                    },
                ],
                is_pub: false,
            },
        ],
    ),
)
```

# Eval
```rust
Ok(
    Block(
        [
            ClassDecl {
                name: "User",
                field_names: [
                    "name",
                    "email",
                ],
                field_defaults: [
                    None,
                    None,
                ],
                methods: [],
            },
        ],
        None,
    ),
)
```