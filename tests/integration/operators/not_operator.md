# Program
Status: 🔴
Assertions: 0

```rustleaf
// Test 'not' as unary operator
assert(not true == false);
assert(not false == true);

// Test with expressions
var x = 5;
assert(not (x > 10) == true);
assert(not (x < 3) == true);   // x=5, x<3 is false, not false is true
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Ident(assert))
parse_statement: starting at position 0 (Ident(assert))
```

# Result
```rust
Skipped due to parse error
```

# Lex
```rust
Ok(
    [
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Not),
        Token(True),
        Token(EqualEqual),
        Token(False),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Not),
        Token(False),
        Token(EqualEqual),
        Token(True),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "x"),
        Token(Equal),
        Token(Int, "5"),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Not),
        Token(LeftParen),
        Token(Ident, "x"),
        Token(Greater),
        Token(Int, "10"),
        Token(RightParen),
        Token(EqualEqual),
        Token(True),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Not),
        Token(LeftParen),
        Token(Ident, "x"),
        Token(Less),
        Token(Int, "3"),
        Token(RightParen),
        Token(EqualEqual),
        Token(True),
        Token(RightParen),
        Token(Semicolon),
        Token(Eof),
    ],
)
```

# Parse
```rust
Err(
    "Expected Hash, found Ident",
)
```

# Eval
```rust
Skipped due to parse error
```