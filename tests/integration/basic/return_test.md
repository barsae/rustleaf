# Program 🔴

```rustleaf
// Test return statement in function
fn test_return() {
    return 42;
}

let result = test_return();
assert(result == 42);

// Test return without value
fn test_return_unit() {
    return;
}

let unit_result = test_return_unit();
assert(true);  // Just test it doesn't crash

// Test early return
fn test_early_return(x) {
    if x > 10 {
        return "big";
    }
    return "small";
}

assert(test_early_return(15) == "big");
assert(test_early_return(5) == "small");
```

# Output

```
Skipped due to parse error
```

# Result

```rust
Skipped due to parse error
```

# Lex

```rust
Ok(
    [
        Token(Fn),
        Token(Ident, "test_return"),
        Token(LeftParen),
        Token(RightParen),
        Token(LeftBrace),
        Token(Return),
        Token(Int, "42"),
        Token(Semicolon),
        Token(RightBrace),
        Token(Ident, "let"),
        Token(Ident, "result"),
        Token(Equal),
        Token(Ident, "test_return"),
        Token(LeftParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result"),
        Token(EqualEqual),
        Token(Int, "42"),
        Token(RightParen),
        Token(Semicolon),
        Token(Fn),
        Token(Ident, "test_return_unit"),
        Token(LeftParen),
        Token(RightParen),
        Token(LeftBrace),
        Token(Return),
        Token(Semicolon),
        Token(RightBrace),
        Token(Ident, "let"),
        Token(Ident, "unit_result"),
        Token(Equal),
        Token(Ident, "test_return_unit"),
        Token(LeftParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(True),
        Token(RightParen),
        Token(Semicolon),
        Token(Fn),
        Token(Ident, "test_early_return"),
        Token(LeftParen),
        Token(Ident, "x"),
        Token(RightParen),
        Token(LeftBrace),
        Token(If),
        Token(Ident, "x"),
        Token(Greater),
        Token(Int, "10"),
        Token(LeftBrace),
        Token(Return),
        Token(String, "big"),
        Token(Semicolon),
        Token(RightBrace),
        Token(Return),
        Token(String, "small"),
        Token(Semicolon),
        Token(RightBrace),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "test_early_return"),
        Token(LeftParen),
        Token(Int, "15"),
        Token(RightParen),
        Token(EqualEqual),
        Token(String, "big"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "test_early_return"),
        Token(LeftParen),
        Token(Int, "5"),
        Token(RightParen),
        Token(EqualEqual),
        Token(String, "small"),
        Token(RightParen),
        Token(Semicolon),
        Token(Eof),
    ],
)
```

# Parse

```rust
Err(
    "Expected statement",
)
```

# Eval

```rust
Skipped due to parse error
```
