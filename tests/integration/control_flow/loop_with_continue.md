# Program
Status: 🔴
Assertions: 0

```rustleaf
var i = 0;
var result = loop {
    i = i + 1;
    if i < 3 {
        continue;
    }
    break i * 10;
};

var j = 0;
var count = 0;
var result2 = loop {
    j = j + 1;
    if j <= 5 {
        count = count + 1;
        continue;
    }
    break j + count;
};

assert(result == 30);
assert(i == 3);
assert(result2 == 11);  
assert(j == 6);
assert(count == 5);
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
        Token(Ident, "i"),
        Token(Equal),
        Token(Int, "0"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "result"),
        Token(Equal),
        Token(Loop),
        Token(LeftBrace),
        Token(Ident, "i"),
        Token(Equal),
        Token(Ident, "i"),
        Token(Plus),
        Token(Int, "1"),
        Token(Semicolon),
        Token(If),
        Token(Ident, "i"),
        Token(Less),
        Token(Int, "3"),
        Token(LeftBrace),
        Token(Continue),
        Token(Semicolon),
        Token(RightBrace),
        Token(Break),
        Token(Ident, "i"),
        Token(Star),
        Token(Int, "10"),
        Token(Semicolon),
        Token(RightBrace),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "j"),
        Token(Equal),
        Token(Int, "0"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "count"),
        Token(Equal),
        Token(Int, "0"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "result2"),
        Token(Equal),
        Token(Loop),
        Token(LeftBrace),
        Token(Ident, "j"),
        Token(Equal),
        Token(Ident, "j"),
        Token(Plus),
        Token(Int, "1"),
        Token(Semicolon),
        Token(If),
        Token(Ident, "j"),
        Token(LessEqual),
        Token(Int, "5"),
        Token(LeftBrace),
        Token(Ident, "count"),
        Token(Equal),
        Token(Ident, "count"),
        Token(Plus),
        Token(Int, "1"),
        Token(Semicolon),
        Token(Continue),
        Token(Semicolon),
        Token(RightBrace),
        Token(Break),
        Token(Ident, "j"),
        Token(Plus),
        Token(Ident, "count"),
        Token(Semicolon),
        Token(RightBrace),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result"),
        Token(EqualEqual),
        Token(Int, "30"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "i"),
        Token(EqualEqual),
        Token(Int, "3"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result2"),
        Token(EqualEqual),
        Token(Int, "11"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "j"),
        Token(EqualEqual),
        Token(Int, "6"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "count"),
        Token(EqualEqual),
        Token(Int, "5"),
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