# Program
Status: 🔴
Assertions: 0

```rustleaf
class File() {
    var opened;
    var closed;

    fn op_open() {
        self.opened += 1;
    }

    fn op_close() {
        self.closed += 1;
    }

    fn read() {}
}

var f = File();
f.opened = 0;
f.closed = 0;
with f {
    f.read();
}
assert(f.opened == 1, "opened should be 1");
assert(f.closed == 1, "closed should be 1");
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Class)
parse_statement: starting at position 0 (Class)
```

# Result
```rust
Skipped due to parse error
```

# Lex
```rust
Ok(
    [
        Token(Class),
        Token(Ident, "File"),
        Token(LeftParen),
        Token(RightParen),
        Token(LeftBrace),
        Token(Var),
        Token(Ident, "opened"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "closed"),
        Token(Semicolon),
        Token(Fn),
        Token(Ident, "op_open"),
        Token(LeftParen),
        Token(RightParen),
        Token(LeftBrace),
        Token(Ident, "self"),
        Token(Dot),
        Token(Ident, "opened"),
        Token(PlusEqual),
        Token(Int, "1"),
        Token(Semicolon),
        Token(RightBrace),
        Token(Fn),
        Token(Ident, "op_close"),
        Token(LeftParen),
        Token(RightParen),
        Token(LeftBrace),
        Token(Ident, "self"),
        Token(Dot),
        Token(Ident, "closed"),
        Token(PlusEqual),
        Token(Int, "1"),
        Token(Semicolon),
        Token(RightBrace),
        Token(Fn),
        Token(Ident, "read"),
        Token(LeftParen),
        Token(RightParen),
        Token(LeftBrace),
        Token(RightBrace),
        Token(RightBrace),
        Token(Var),
        Token(Ident, "f"),
        Token(Equal),
        Token(Ident, "File"),
        Token(LeftParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "f"),
        Token(Dot),
        Token(Ident, "opened"),
        Token(Equal),
        Token(Int, "0"),
        Token(Semicolon),
        Token(Ident, "f"),
        Token(Dot),
        Token(Ident, "closed"),
        Token(Equal),
        Token(Int, "0"),
        Token(Semicolon),
        Token(With),
        Token(Ident, "f"),
        Token(LeftBrace),
        Token(Ident, "f"),
        Token(Dot),
        Token(Ident, "read"),
        Token(LeftParen),
        Token(RightParen),
        Token(Semicolon),
        Token(RightBrace),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "f"),
        Token(Dot),
        Token(Ident, "opened"),
        Token(EqualEqual),
        Token(Int, "1"),
        Token(Comma),
        Token(String, "opened should be 1"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "f"),
        Token(Dot),
        Token(Ident, "closed"),
        Token(EqualEqual),
        Token(Int, "1"),
        Token(Comma),
        Token(String, "closed should be 1"),
        Token(RightParen),
        Token(Semicolon),
        Token(Eof),
    ],
)
```

# Parse
```rust
Err(
    "Expected Hash, found Class",
)
```

# Eval
```rust
Skipped due to parse error
```