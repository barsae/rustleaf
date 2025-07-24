# Program
Status: 🔴
Assertions: 0

```rustleaf
// #[fail_quietly]
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
Err(
    "Statement not yet implemented: ClassDecl { name: \"User\", members: [ClassMember { name: \"name\", kind: Field(None) }, ClassMember { name: \"email\", kind: Field(None) }], is_pub: false }",
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
Err(
    "Statement not yet implemented: ClassDecl { name: \"User\", members: [ClassMember { name: \"name\", kind: Field(None) }, ClassMember { name: \"email\", kind: Field(None) }], is_pub: false }",
)
```