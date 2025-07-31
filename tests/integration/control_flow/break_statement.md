# Program
Status: 🔴
Assertions: 0

```rustleaf
var result = loop {
    break 42;
};

var result2 = loop {
    break "hello";
};

var counter = 0;
var result3 = loop {
    counter = counter + 1;
    if counter == 3 {
        break counter * 5;
    }
};

assert(result == 42);
assert(result2 == "hello");
assert(result3 == 15);
assert(counter == 3);
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
        Token(Ident, "result"),
        Token(Equal),
        Token(Loop),
        Token(LeftBrace),
        Token(Break),
        Token(Int, "42"),
        Token(Semicolon),
        Token(RightBrace),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "result2"),
        Token(Equal),
        Token(Loop),
        Token(LeftBrace),
        Token(Break),
        Token(String, "hello"),
        Token(Semicolon),
        Token(RightBrace),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "counter"),
        Token(Equal),
        Token(Int, "0"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "result3"),
        Token(Equal),
        Token(Loop),
        Token(LeftBrace),
        Token(Ident, "counter"),
        Token(Equal),
        Token(Ident, "counter"),
        Token(Plus),
        Token(Int, "1"),
        Token(Semicolon),
        Token(If),
        Token(Ident, "counter"),
        Token(EqualEqual),
        Token(Int, "3"),
        Token(LeftBrace),
        Token(Break),
        Token(Ident, "counter"),
        Token(Star),
        Token(Int, "5"),
        Token(Semicolon),
        Token(RightBrace),
        Token(RightBrace),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result"),
        Token(EqualEqual),
        Token(Int, "42"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result2"),
        Token(EqualEqual),
        Token(String, "hello"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result3"),
        Token(EqualEqual),
        Token(Int, "15"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "counter"),
        Token(EqualEqual),
        Token(Int, "3"),
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