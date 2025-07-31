# Program
Status: 🔴
Assertions: 0

```rustleaf
fn greet(name = "world") { name }
fn add(x, y = 10) { x + y }
fn multiply(a = 2, b = 3) { a * b }

var greeting1 = greet();
var greeting2 = greet("Alice");
var sum1 = add(5);
var sum2 = add(5, 15);
var product1 = multiply();
var product2 = multiply(4);
var product3 = multiply(4, 7);

assert(greeting1 == "world");
assert(greeting2 == "Alice");
assert(sum1 == 15);
assert(sum2 == 20);
assert(product1 == 6);
assert(product2 == 12);
assert(product3 == 28);
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Fn)
parse_statement: starting at position 0 (Fn)
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
        Token(Ident, "greet"),
        Token(LeftParen),
        Token(Ident, "name"),
        Token(Equal),
        Token(String, "world"),
        Token(RightParen),
        Token(LeftBrace),
        Token(Ident, "name"),
        Token(RightBrace),
        Token(Fn),
        Token(Ident, "add"),
        Token(LeftParen),
        Token(Ident, "x"),
        Token(Comma),
        Token(Ident, "y"),
        Token(Equal),
        Token(Int, "10"),
        Token(RightParen),
        Token(LeftBrace),
        Token(Ident, "x"),
        Token(Plus),
        Token(Ident, "y"),
        Token(RightBrace),
        Token(Fn),
        Token(Ident, "multiply"),
        Token(LeftParen),
        Token(Ident, "a"),
        Token(Equal),
        Token(Int, "2"),
        Token(Comma),
        Token(Ident, "b"),
        Token(Equal),
        Token(Int, "3"),
        Token(RightParen),
        Token(LeftBrace),
        Token(Ident, "a"),
        Token(Star),
        Token(Ident, "b"),
        Token(RightBrace),
        Token(Var),
        Token(Ident, "greeting1"),
        Token(Equal),
        Token(Ident, "greet"),
        Token(LeftParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "greeting2"),
        Token(Equal),
        Token(Ident, "greet"),
        Token(LeftParen),
        Token(String, "Alice"),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "sum1"),
        Token(Equal),
        Token(Ident, "add"),
        Token(LeftParen),
        Token(Int, "5"),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "sum2"),
        Token(Equal),
        Token(Ident, "add"),
        Token(LeftParen),
        Token(Int, "5"),
        Token(Comma),
        Token(Int, "15"),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "product1"),
        Token(Equal),
        Token(Ident, "multiply"),
        Token(LeftParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "product2"),
        Token(Equal),
        Token(Ident, "multiply"),
        Token(LeftParen),
        Token(Int, "4"),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "product3"),
        Token(Equal),
        Token(Ident, "multiply"),
        Token(LeftParen),
        Token(Int, "4"),
        Token(Comma),
        Token(Int, "7"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "greeting1"),
        Token(EqualEqual),
        Token(String, "world"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "greeting2"),
        Token(EqualEqual),
        Token(String, "Alice"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "sum1"),
        Token(EqualEqual),
        Token(Int, "15"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "sum2"),
        Token(EqualEqual),
        Token(Int, "20"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "product1"),
        Token(EqualEqual),
        Token(Int, "6"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "product2"),
        Token(EqualEqual),
        Token(Int, "12"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "product3"),
        Token(EqualEqual),
        Token(Int, "28"),
        Token(RightParen),
        Token(Semicolon),
        Token(Eof),
    ],
)
```

# Parse
```rust
Err(
    "Expected Hash, found Fn",
)
```

# Eval
```rust
Skipped due to parse error
```