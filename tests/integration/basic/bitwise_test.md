# Program 🔴

```rustleaf
// Bitwise operators test
assert((5 & 3) == 1);  // 101 & 011 = 001
assert((5 | 3) == 7);  // 101 | 011 = 111
assert((5 ^ 3) == 6);  // 101 ^ 011 = 110

// Bit shifts
assert((8 << 1) == 16);
assert((8 >> 1) == 4);
assert((8 >> 2) == 2);

// Bitwise NOT
assert((~5) == -6);  // Two's complement
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
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(LeftParen),
        Token(Int, "5"),
        Token(Ampersand),
        Token(Int, "3"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "1"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(LeftParen),
        Token(Int, "5"),
        Token(Pipe),
        Token(Int, "3"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "7"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(LeftParen),
        Token(Int, "5"),
        Token(Caret),
        Token(Int, "3"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "6"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(LeftParen),
        Token(Int, "8"),
        Token(LessLess),
        Token(Int, "1"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "16"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(LeftParen),
        Token(Int, "8"),
        Token(GreaterGreater),
        Token(Int, "1"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "4"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(LeftParen),
        Token(Int, "8"),
        Token(GreaterGreater),
        Token(Int, "2"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "2"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(LeftParen),
        Token(Tilde),
        Token(Int, "5"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Minus),
        Token(Int, "6"),
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
