# Program
Status: 🔴
Assertions: 0

```rustleaf
fn memoize(f) {
    var param_list = join(f.params, ", ");

    parse("var ${f.name} = {
        var cache = {};

        fn original(${param_list}) ${f.body}

        fn cached(${param_list}) {
            var args_key = str(${param_list});
            if args_key in cache {
                cache[args_key]
            } else {
                var result = original(${param_list});
                cache[args_key] = result;
                result
            }
        }

        cached
    };")
}

var count = 0;
#[memoize]
fn fibonacci(n) {
    count += 1;
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

var f = fibonacci(10);
assert(f == 55, "f");
print(count);
assert(count == 11, "count");
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
        Token(Ident, "memoize"),
        Token(LeftParen),
        Token(Ident, "f"),
        Token(RightParen),
        Token(LeftBrace),
        Token(Var),
        Token(Ident, "param_list"),
        Token(Equal),
        Token(Ident, "join"),
        Token(LeftParen),
        Token(Ident, "f"),
        Token(Dot),
        Token(Ident, "params"),
        Token(Comma),
        Token(String, ", "),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "parse"),
        Token(LeftParen),
        Token(StringPart, "var "),
        Token(InterpolationStart),
        Token(Ident, "f"),
        Token(Dot),
        Token(Ident, "name"),
        Token(InterpolationEnd),
        Token(StringPart, " = {\n        var cache = {};\n\n        fn original("),
        Token(InterpolationStart),
        Token(Ident, "param_list"),
        Token(InterpolationEnd),
        Token(StringPart, ") "),
        Token(InterpolationStart),
        Token(Ident, "f"),
        Token(Dot),
        Token(Ident, "body"),
        Token(InterpolationEnd),
        Token(StringPart, "\n\n        fn cached("),
        Token(InterpolationStart),
        Token(Ident, "param_list"),
        Token(InterpolationEnd),
        Token(StringPart, ") {\n            var args_key = str("),
        Token(InterpolationStart),
        Token(Ident, "param_list"),
        Token(InterpolationEnd),
        Token(StringPart, ");\n            if args_key in cache {\n                cache[args_key]\n            } else {\n                var result = original("),
        Token(InterpolationStart),
        Token(Ident, "param_list"),
        Token(InterpolationEnd),
        Token(StringPart, ");\n                cache[args_key] = result;\n                result\n            }\n        }\n\n        cached\n    };"),
        Token(RightParen),
        Token(RightBrace),
        Token(Var),
        Token(Ident, "count"),
        Token(Equal),
        Token(Int, "0"),
        Token(Semicolon),
        Token(Hash),
        Token(LeftBracket),
        Token(Ident, "memoize"),
        Token(RightBracket),
        Token(Fn),
        Token(Ident, "fibonacci"),
        Token(LeftParen),
        Token(Ident, "n"),
        Token(RightParen),
        Token(LeftBrace),
        Token(Ident, "count"),
        Token(PlusEqual),
        Token(Int, "1"),
        Token(Semicolon),
        Token(If),
        Token(Ident, "n"),
        Token(LessEqual),
        Token(Int, "1"),
        Token(LeftBrace),
        Token(Ident, "n"),
        Token(RightBrace),
        Token(Else),
        Token(LeftBrace),
        Token(Ident, "fibonacci"),
        Token(LeftParen),
        Token(Ident, "n"),
        Token(Minus),
        Token(Int, "1"),
        Token(RightParen),
        Token(Plus),
        Token(Ident, "fibonacci"),
        Token(LeftParen),
        Token(Ident, "n"),
        Token(Minus),
        Token(Int, "2"),
        Token(RightParen),
        Token(RightBrace),
        Token(RightBrace),
        Token(Var),
        Token(Ident, "f"),
        Token(Equal),
        Token(Ident, "fibonacci"),
        Token(LeftParen),
        Token(Int, "10"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "f"),
        Token(EqualEqual),
        Token(Int, "55"),
        Token(Comma),
        Token(String, "f"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "print"),
        Token(LeftParen),
        Token(Ident, "count"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "count"),
        Token(EqualEqual),
        Token(Int, "11"),
        Token(Comma),
        Token(String, "count"),
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