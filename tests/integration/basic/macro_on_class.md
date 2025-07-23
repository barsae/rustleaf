# Program

```rustleaf
#[serializable]
class User {
    var name;
    var email;
}
```

# Lex

```rust
Ok(
    [
        Token {
            token_type: Hash,
            text: None,
        },
        Token {
            token_type: LeftBracket,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "serializable",
            ),
        },
        Token {
            token_type: RightBracket,
            text: None,
        },
        Token {
            token_type: Class,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "User",
            ),
        },
        Token {
            token_type: LeftBrace,
            text: None,
        },
        Token {
            token_type: Var,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "name",
            ),
        },
        Token {
            token_type: Semicolon,
            text: None,
        },
        Token {
            token_type: Var,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "email",
            ),
        },
        Token {
            token_type: Semicolon,
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

# Output

```

```

# Result

```rust
Err(
    "Statement not yet implemented: ClassDecl { name: \"User\", members: [ClassMember { name: \"name\", kind: Field(None) }, ClassMember { name: \"email\", kind: Field(None) }], is_pub: false }",
)
```
