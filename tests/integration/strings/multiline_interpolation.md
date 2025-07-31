# Program
Status: 🔴
Assertions: 0

```rustleaf
var name = "World";
var result = "Hello ${name}!
This is a multiline string with interpolation.";
print(result);
assert(result == "Hello World!
This is a multiline string with interpolation.");
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Var)
parse_statement: starting at position 0 (Var)
```

# Result
```rust
Skipped due to parse error
```

# Lex
```rust
Ok(
    [
        Token(Var),
        Token(Ident, "name"),
        Token(Equal),
        Token(String, "World"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "result"),
        Token(Equal),
        Token(StringPart, "Hello "),
        Token(InterpolationStart),
        Token(Ident, "name"),
        Token(InterpolationEnd),
        Token(StringPart, "!\nThis is a multiline string with interpolation."),
        Token(Semicolon),
        Token(Ident, "print"),
        Token(LeftParen),
        Token(Ident, "result"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result"),
        Token(EqualEqual),
        Token(String, "Hello World!\nThis is a multiline string with interpolation."),
        Token(RightParen),
        Token(Semicolon),
        Token(Eof),
    ],
)
```

# Parse
```rust
Err(
    "Expected Hash, found Var",
)
```

# Eval
```rust
Skipped due to parse error
```