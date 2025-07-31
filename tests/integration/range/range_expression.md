# Program
Status: 🔴
Assertions: 0

```rustleaf
// Test using ranges in for loops and expressions
var sum = 0;
for i in 1..5 {
    sum += i;
}
assert(sum == 10);  // 1 + 2 + 3 + 4 = 10

// Test using ranges with different step sizes
var range = 0..10;
var even_count = 0;
for x in range {
    if x % 2 == 0 {
        even_count += 1;
    }
}
assert(even_count == 5);  // 0, 2, 4, 6, 8

// Test range as expression
var small_range = 3..6;
assert(4 in small_range);
assert(not (6 in small_range));
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
        Token(Ident, "sum"),
        Token(Equal),
        Token(Int, "0"),
        Token(Semicolon),
        Token(For),
        Token(Ident, "i"),
        Token(In),
        Token(Int, "1"),
        Token(DotDot),
        Token(Int, "5"),
        Token(LeftBrace),
        Token(Ident, "sum"),
        Token(PlusEqual),
        Token(Ident, "i"),
        Token(Semicolon),
        Token(RightBrace),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "sum"),
        Token(EqualEqual),
        Token(Int, "10"),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "range"),
        Token(Equal),
        Token(Int, "0"),
        Token(DotDot),
        Token(Int, "10"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "even_count"),
        Token(Equal),
        Token(Int, "0"),
        Token(Semicolon),
        Token(For),
        Token(Ident, "x"),
        Token(In),
        Token(Ident, "range"),
        Token(LeftBrace),
        Token(If),
        Token(Ident, "x"),
        Token(Percent),
        Token(Int, "2"),
        Token(EqualEqual),
        Token(Int, "0"),
        Token(LeftBrace),
        Token(Ident, "even_count"),
        Token(PlusEqual),
        Token(Int, "1"),
        Token(Semicolon),
        Token(RightBrace),
        Token(RightBrace),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "even_count"),
        Token(EqualEqual),
        Token(Int, "5"),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "small_range"),
        Token(Equal),
        Token(Int, "3"),
        Token(DotDot),
        Token(Int, "6"),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Int, "4"),
        Token(In),
        Token(Ident, "small_range"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Not),
        Token(LeftParen),
        Token(Int, "6"),
        Token(In),
        Token(Ident, "small_range"),
        Token(RightParen),
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