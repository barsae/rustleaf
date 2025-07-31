# Program
Status: 🔴
Assertions: 0

```rustleaf
var processor = |x| {
    var temp = x * 2;
    temp + 1
};

var complex_lambda = |y| {
    var first = y + 10;
    var second = first * 3;
    second - 5
};

assert(processor(5) == 11);
assert(processor(0) == 1);
assert(complex_lambda(2) == 31);
assert(complex_lambda(10) == 55);
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
        Token(Ident, "processor"),
        Token(Equal),
        Token(Pipe),
        Token(Ident, "x"),
        Token(Pipe),
        Token(LeftBrace),
        Token(Var),
        Token(Ident, "temp"),
        Token(Equal),
        Token(Ident, "x"),
        Token(Star),
        Token(Int, "2"),
        Token(Semicolon),
        Token(Ident, "temp"),
        Token(Plus),
        Token(Int, "1"),
        Token(RightBrace),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "complex_lambda"),
        Token(Equal),
        Token(Pipe),
        Token(Ident, "y"),
        Token(Pipe),
        Token(LeftBrace),
        Token(Var),
        Token(Ident, "first"),
        Token(Equal),
        Token(Ident, "y"),
        Token(Plus),
        Token(Int, "10"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "second"),
        Token(Equal),
        Token(Ident, "first"),
        Token(Star),
        Token(Int, "3"),
        Token(Semicolon),
        Token(Ident, "second"),
        Token(Minus),
        Token(Int, "5"),
        Token(RightBrace),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "processor"),
        Token(LeftParen),
        Token(Int, "5"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "11"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "processor"),
        Token(LeftParen),
        Token(Int, "0"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "1"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "complex_lambda"),
        Token(LeftParen),
        Token(Int, "2"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "31"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "complex_lambda"),
        Token(LeftParen),
        Token(Int, "10"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "55"),
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