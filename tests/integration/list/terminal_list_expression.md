# Program
Status: 🔴
Assertions: 0

```rustleaf
fn f() {
    var j = [10, 20];
    for i in j {
        i;
    }
    [1, 2, 3]
}
// This used to parse as "for_loop[1, 2, 3]", an indexing operation
assert(f() == [1, 2, 3]);
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Fn)
parse_statement: starting at position 0 (Fn)
parse_statement: starting at position 5 (Var)
parse_expression: starting at position 8 (LeftBracket)
parse_primary: success - parsing list literal
parse_expression: starting at position 9 (Int(10))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: starting at position 11 (Int(20))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_statement: success - parsed var declaration
parse_statement: starting at position 14 (For)
parse_expression: starting at position 14 (For)
parse_primary: success - parsing for expression
parse_expression: starting at position 17 (Ident(j))
parse_primary: success - parsed identifier (j)
parse_expression: success - parsed precedence expression
parse_statement: starting at position 19 (Ident(i))
parse_statement: falling back to expression statement
parse_expression: starting at position 19 (Ident(i))
parse_primary: success - parsed identifier (i)
parse_expression: success - parsed precedence expression
parse_expression: starting at position 23 (Int(1))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: failed - Expected RightBracket, found Comma
parse_expression: starting at position 14 (For)
parse_primary: success - parsing for expression
parse_expression: starting at position 17 (Ident(j))
parse_primary: success - parsed identifier (j)
parse_expression: success - parsed precedence expression
parse_statement: starting at position 19 (Ident(i))
parse_statement: falling back to expression statement
parse_expression: starting at position 19 (Ident(i))
parse_primary: success - parsed identifier (i)
parse_expression: success - parsed precedence expression
parse_expression: starting at position 23 (Int(1))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: failed - Expected RightBracket, found Comma
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
        Token(Ident, "f"),
        Token(LeftParen),
        Token(RightParen),
        Token(LeftBrace),
        Token(Var),
        Token(Ident, "j"),
        Token(Equal),
        Token(LeftBracket),
        Token(Int, "10"),
        Token(Comma),
        Token(Int, "20"),
        Token(RightBracket),
        Token(Semicolon),
        Token(For),
        Token(Ident, "i"),
        Token(In),
        Token(Ident, "j"),
        Token(LeftBrace),
        Token(Ident, "i"),
        Token(Semicolon),
        Token(RightBrace),
        Token(LeftBracket),
        Token(Int, "1"),
        Token(Comma),
        Token(Int, "2"),
        Token(Comma),
        Token(Int, "3"),
        Token(RightBracket),
        Token(RightBrace),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "f"),
        Token(LeftParen),
        Token(RightParen),
        Token(EqualEqual),
        Token(LeftBracket),
        Token(Int, "1"),
        Token(Comma),
        Token(Int, "2"),
        Token(Comma),
        Token(Int, "3"),
        Token(RightBracket),
        Token(RightParen),
        Token(Semicolon),
        Token(Eof),
    ],
)
```

# Parse
```rust
Err(
    "Expected statement or expression",
)
```

# Eval
```rust
Skipped due to parse error
```