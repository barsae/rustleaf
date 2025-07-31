# Program
Status: 🔴
Assertions: 0

```rustleaf
// Test exclusive ranges (1..10 = [1, 2, 3, 4, 5, 6, 7, 8, 9])
var range = 1..10;

// Test range properties
assert(range.start == 1);
assert(range.end == 10);
assert(range.inclusive == false);

// Test range iteration (convert to list)
var list = range.to_list();
assert(list.length == 9);
assert(list[0] == 1);
assert(list[8] == 9);
assert(not (10 in list));

// Test range membership
assert(5 in range);
assert(not (10 in range));
assert(not (0 in range));
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
        Token(Ident, "range"),
        Token(Equal),
        Token(Int, "1"),
        Token(DotDot),
        Token(Int, "10"),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "range"),
        Token(Dot),
        Token(Ident, "start"),
        Token(EqualEqual),
        Token(Int, "1"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "range"),
        Token(Dot),
        Token(Ident, "end"),
        Token(EqualEqual),
        Token(Int, "10"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "range"),
        Token(Dot),
        Token(Ident, "inclusive"),
        Token(EqualEqual),
        Token(False),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "list"),
        Token(Equal),
        Token(Ident, "range"),
        Token(Dot),
        Token(Ident, "to_list"),
        Token(LeftParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "list"),
        Token(Dot),
        Token(Ident, "length"),
        Token(EqualEqual),
        Token(Int, "9"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "list"),
        Token(LeftBracket),
        Token(Int, "0"),
        Token(RightBracket),
        Token(EqualEqual),
        Token(Int, "1"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "list"),
        Token(LeftBracket),
        Token(Int, "8"),
        Token(RightBracket),
        Token(EqualEqual),
        Token(Int, "9"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Not),
        Token(LeftParen),
        Token(Int, "10"),
        Token(In),
        Token(Ident, "list"),
        Token(RightParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Int, "5"),
        Token(In),
        Token(Ident, "range"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Not),
        Token(LeftParen),
        Token(Int, "10"),
        Token(In),
        Token(Ident, "range"),
        Token(RightParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Not),
        Token(LeftParen),
        Token(Int, "0"),
        Token(In),
        Token(Ident, "range"),
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