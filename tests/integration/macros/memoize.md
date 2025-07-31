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
consume_token: position 0 consumed Fn
consume_token: position 1 consumed Ident
consume_token: position 2 consumed LeftParen
consume_token: position 3 consumed Ident
consume_token: position 4 consumed RightParen
consume_token: position 5 consumed LeftBrace
parse_statement: starting at position 6 (Var)
consume_token: position 6 consumed Var
consume_token: position 7 consumed Ident
consume_token: position 8 consumed Equal
parse_expression: starting at position 9 (Ident(join))
consume_token: position 9 consumed Ident
parse_primary: success - parsed identifier (join)
consume_token: position 10 consumed LeftParen
parse_expression: starting at position 11 (Ident(f))
consume_token: position 11 consumed Ident
parse_primary: success - parsed identifier (f)
consume_token: position 12 consumed Dot
consume_token: position 13 consumed Ident
parse_expression: success - parsed precedence expression
consume_token: position 14 consumed Comma
parse_expression: starting at position 15 (String(, ))
consume_token: position 15 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 16 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 17 consumed Semicolon
parse_statement: success - parsed var declaration
parse_statement: starting at position 18 (Ident(parse))
consume_token: position 18 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 18 (Ident(parse))
consume_token: position 18 consumed Ident
parse_primary: success - parsed identifier (parse)
consume_token: position 19 consumed LeftParen
parse_expression: starting at position 20 (StringPart(var ))
parse_primary: success - parsing interpolated string
consume_token: position 20 consumed StringPart
consume_token: position 21 consumed InterpolationStart
parse_expression: starting at position 22 (Ident(f))
consume_token: position 22 consumed Ident
parse_primary: success - parsed identifier (f)
consume_token: position 23 consumed Dot
consume_token: position 24 consumed Ident
parse_expression: success - parsed precedence expression
consume_token: position 25 consumed InterpolationEnd
consume_token: position 26 consumed StringPart
consume_token: position 27 consumed InterpolationStart
parse_expression: starting at position 28 (Ident(param_list))
consume_token: position 28 consumed Ident
parse_primary: success - parsed identifier (param_list)
parse_expression: success - parsed precedence expression
consume_token: position 29 consumed InterpolationEnd
consume_token: position 30 consumed StringPart
consume_token: position 31 consumed InterpolationStart
parse_expression: starting at position 32 (Ident(f))
consume_token: position 32 consumed Ident
parse_primary: success - parsed identifier (f)
consume_token: position 33 consumed Dot
consume_token: position 34 consumed Ident
parse_expression: success - parsed precedence expression
consume_token: position 35 consumed InterpolationEnd
consume_token: position 36 consumed StringPart
consume_token: position 37 consumed InterpolationStart
parse_expression: starting at position 38 (Ident(param_list))
consume_token: position 38 consumed Ident
parse_primary: success - parsed identifier (param_list)
parse_expression: success - parsed precedence expression
consume_token: position 39 consumed InterpolationEnd
consume_token: position 40 consumed StringPart
consume_token: position 41 consumed InterpolationStart
parse_expression: starting at position 42 (Ident(param_list))
consume_token: position 42 consumed Ident
parse_primary: success - parsed identifier (param_list)
parse_expression: success - parsed precedence expression
consume_token: position 43 consumed InterpolationEnd
consume_token: position 44 consumed StringPart
consume_token: position 45 consumed InterpolationStart
parse_expression: starting at position 46 (Ident(param_list))
consume_token: position 46 consumed Ident
parse_primary: success - parsed identifier (param_list)
parse_expression: success - parsed precedence expression
consume_token: position 47 consumed InterpolationEnd
consume_token: position 48 consumed StringPart
parse_expression: success - parsed precedence expression
consume_token: position 49 consumed RightParen
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace at position 50
parse_expression: starting at position 18 (Ident(parse))
consume_token: position 18 consumed Ident
parse_primary: success - parsed identifier (parse)
consume_token: position 19 consumed LeftParen
parse_expression: starting at position 20 (StringPart(var ))
parse_primary: success - parsing interpolated string
consume_token: position 20 consumed StringPart
consume_token: position 21 consumed InterpolationStart
parse_expression: starting at position 22 (Ident(f))
consume_token: position 22 consumed Ident
parse_primary: success - parsed identifier (f)
consume_token: position 23 consumed Dot
consume_token: position 24 consumed Ident
parse_expression: success - parsed precedence expression
consume_token: position 25 consumed InterpolationEnd
consume_token: position 26 consumed StringPart
consume_token: position 27 consumed InterpolationStart
parse_expression: starting at position 28 (Ident(param_list))
consume_token: position 28 consumed Ident
parse_primary: success - parsed identifier (param_list)
parse_expression: success - parsed precedence expression
consume_token: position 29 consumed InterpolationEnd
consume_token: position 30 consumed StringPart
consume_token: position 31 consumed InterpolationStart
parse_expression: starting at position 32 (Ident(f))
consume_token: position 32 consumed Ident
parse_primary: success - parsed identifier (f)
consume_token: position 33 consumed Dot
consume_token: position 34 consumed Ident
parse_expression: success - parsed precedence expression
consume_token: position 35 consumed InterpolationEnd
consume_token: position 36 consumed StringPart
consume_token: position 37 consumed InterpolationStart
parse_expression: starting at position 38 (Ident(param_list))
consume_token: position 38 consumed Ident
parse_primary: success - parsed identifier (param_list)
parse_expression: success - parsed precedence expression
consume_token: position 39 consumed InterpolationEnd
consume_token: position 40 consumed StringPart
consume_token: position 41 consumed InterpolationStart
parse_expression: starting at position 42 (Ident(param_list))
consume_token: position 42 consumed Ident
parse_primary: success - parsed identifier (param_list)
parse_expression: success - parsed precedence expression
consume_token: position 43 consumed InterpolationEnd
consume_token: position 44 consumed StringPart
consume_token: position 45 consumed InterpolationStart
parse_expression: starting at position 46 (Ident(param_list))
consume_token: position 46 consumed Ident
parse_primary: success - parsed identifier (param_list)
parse_expression: success - parsed precedence expression
consume_token: position 47 consumed InterpolationEnd
consume_token: position 48 consumed StringPart
parse_expression: success - parsed precedence expression
consume_token: position 49 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 50 consumed RightBrace
parse_statement: success - parsed function declaration
parse_program: parsing statement at position 51 (Var)
parse_statement: starting at position 51 (Var)
consume_token: position 51 consumed Var
consume_token: position 52 consumed Ident
consume_token: position 53 consumed Equal
parse_expression: starting at position 54 (Int(0))
consume_token: position 54 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 55 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 56 (Hash)
parse_statement: starting at position 56 (Hash)
consume_token: position 56 consumed Hash
consume_token: position 57 consumed LeftBracket
consume_token: position 58 consumed Ident
consume_token: position 59 consumed RightBracket
parse_statement: starting at position 60 (Fn)
consume_token: position 60 consumed Fn
consume_token: position 61 consumed Ident
consume_token: position 62 consumed LeftParen
consume_token: position 63 consumed Ident
consume_token: position 64 consumed RightParen
consume_token: position 65 consumed LeftBrace
parse_statement: starting at position 66 (Ident(count))
consume_token: position 66 consumed Ident
consume_token: position 67 consumed PlusEqual
parse_expression: starting at position 68 (Int(1))
consume_token: position 68 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 69 consumed Semicolon
parse_statement: success - parsed assignment
parse_statement: starting at position 70 (If)
parse_expression: starting at position 70 (If)
consume_token: position 70 consumed If
parse_primary: success - parsing if expression
parse_expression: starting at position 71 (Ident(n))
consume_token: position 71 consumed Ident
parse_primary: success - parsed identifier (n)
consume_token: position 72 consumed LessEqual
consume_token: position 73 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 74 consumed LeftBrace
parse_statement: starting at position 75 (Ident(n))
consume_token: position 75 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 75 (Ident(n))
consume_token: position 75 consumed Ident
parse_primary: success - parsed identifier (n)
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace at position 76
parse_expression: starting at position 75 (Ident(n))
consume_token: position 75 consumed Ident
parse_primary: success - parsed identifier (n)
parse_expression: success - parsed precedence expression
consume_token: position 76 consumed RightBrace
consume_token: position 77 consumed Else
consume_token: position 78 consumed LeftBrace
parse_statement: starting at position 79 (Ident(fibonacci))
consume_token: position 79 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 79 (Ident(fibonacci))
consume_token: position 79 consumed Ident
parse_primary: success - parsed identifier (fibonacci)
consume_token: position 80 consumed LeftParen
parse_expression: starting at position 81 (Ident(n))
consume_token: position 81 consumed Ident
parse_primary: success - parsed identifier (n)
consume_token: position 82 consumed Minus
consume_token: position 83 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 84 consumed RightParen
consume_token: position 85 consumed Plus
consume_token: position 86 consumed Ident
parse_primary: success - parsed identifier (fibonacci)
consume_token: position 87 consumed LeftParen
parse_expression: starting at position 88 (Ident(n))
consume_token: position 88 consumed Ident
parse_primary: success - parsed identifier (n)
consume_token: position 89 consumed Minus
consume_token: position 90 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 91 consumed RightParen
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace at position 92
parse_expression: starting at position 79 (Ident(fibonacci))
consume_token: position 79 consumed Ident
parse_primary: success - parsed identifier (fibonacci)
consume_token: position 80 consumed LeftParen
parse_expression: starting at position 81 (Ident(n))
consume_token: position 81 consumed Ident
parse_primary: success - parsed identifier (n)
consume_token: position 82 consumed Minus
consume_token: position 83 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 84 consumed RightParen
consume_token: position 85 consumed Plus
consume_token: position 86 consumed Ident
parse_primary: success - parsed identifier (fibonacci)
consume_token: position 87 consumed LeftParen
parse_expression: starting at position 88 (Ident(n))
consume_token: position 88 consumed Ident
parse_primary: success - parsed identifier (n)
consume_token: position 89 consumed Minus
consume_token: position 90 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 91 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 92 consumed RightBrace
parse_expression: success - parsed precedence expression
parse_statement: success - parsed block-like expression statement
consume_token: position 93 consumed RightBrace
parse_statement: success - parsed function declaration
parse_statement: success - parsed macro
parse_program: parsing statement at position 94 (Var)
parse_statement: starting at position 94 (Var)
consume_token: position 94 consumed Var
consume_token: position 95 consumed Ident
consume_token: position 96 consumed Equal
parse_expression: starting at position 97 (Ident(fibonacci))
consume_token: position 97 consumed Ident
parse_primary: success - parsed identifier (fibonacci)
consume_token: position 98 consumed LeftParen
parse_expression: starting at position 99 (Int(10))
consume_token: position 99 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 100 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 101 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 102 (Ident(assert))
parse_statement: starting at position 102 (Ident(assert))
consume_token: position 102 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 102 (Ident(assert))
consume_token: position 102 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 103 consumed LeftParen
parse_expression: starting at position 104 (Ident(f))
consume_token: position 104 consumed Ident
parse_primary: success - parsed identifier (f)
consume_token: position 105 consumed EqualEqual
consume_token: position 106 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 107 consumed Comma
parse_expression: starting at position 108 (String(f))
consume_token: position 108 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 109 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 110 consumed Semicolon
parse_program: parsing statement at position 111 (Ident(print))
parse_statement: starting at position 111 (Ident(print))
consume_token: position 111 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 111 (Ident(print))
consume_token: position 111 consumed Ident
parse_primary: success - parsed identifier (print)
consume_token: position 112 consumed LeftParen
parse_expression: starting at position 113 (Ident(count))
consume_token: position 113 consumed Ident
parse_primary: success - parsed identifier (count)
parse_expression: success - parsed precedence expression
consume_token: position 114 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 115 consumed Semicolon
parse_program: parsing statement at position 116 (Ident(assert))
parse_statement: starting at position 116 (Ident(assert))
consume_token: position 116 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 116 (Ident(assert))
consume_token: position 116 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 117 consumed LeftParen
parse_expression: starting at position 118 (Ident(count))
consume_token: position 118 consumed Ident
parse_primary: success - parsed identifier (count)
consume_token: position 119 consumed EqualEqual
consume_token: position 120 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 121 consumed Comma
parse_expression: starting at position 122 (String(count))
consume_token: position 122 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 123 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 124 consumed Semicolon
parse_program: parsed 7 statements
parse_program: starting
parse_program: parsing statement at position 0 (Var)
parse_statement: starting at position 0 (Var)
consume_token: position 0 consumed Var
consume_token: position 1 consumed Ident
consume_token: position 2 consumed Equal
parse_expression: starting at position 3 (LeftBrace)
parse_primary: success - parsing block or dict
consume_token: position 3 consumed LeftBrace
parse_primary: failed - no matching primary expression for Var
parse_statement: starting at position 4 (Var)
consume_token: position 4 consumed Var
consume_token: position 5 consumed Ident
consume_token: position 6 consumed Equal
parse_expression: starting at position 7 (LeftBrace)
parse_primary: success - parsing block or dict
consume_token: position 7 consumed LeftBrace
consume_token: position 8 consumed RightBrace
parse_expression: success - parsed precedence expression
consume_token: position 9 consumed Semicolon
parse_statement: success - parsed var declaration
parse_statement: starting at position 10 (Fn)
consume_token: position 10 consumed Fn
consume_token: position 11 consumed Ident
consume_token: position 12 consumed LeftParen
consume_token: position 13 consumed Ident
consume_token: position 14 consumed RightParen
consume_token: position 15 consumed LeftBrace
parse_statement: starting at position 16 (Ident(count))
consume_token: position 16 consumed Ident
consume_token: position 17 consumed Equal
parse_expression: starting at position 18 (Ident(count))
consume_token: position 18 consumed Ident
parse_primary: success - parsed identifier (count)
consume_token: position 19 consumed Dot
consume_token: position 20 consumed Ident
consume_token: position 21 consumed LeftParen
parse_expression: starting at position 22 (Int(1))
consume_token: position 22 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 23 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 24 consumed Semicolon
parse_statement: success - parsed assignment
parse_statement: starting at position 25 (If)
parse_expression: starting at position 25 (If)
consume_token: position 25 consumed If
parse_primary: success - parsing if expression
parse_expression: starting at position 26 (Ident(n))
consume_token: position 26 consumed Ident
parse_primary: success - parsed identifier (n)
consume_token: position 27 consumed Dot
consume_token: position 28 consumed Ident
consume_token: position 29 consumed LeftParen
parse_expression: starting at position 30 (Int(1))
consume_token: position 30 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 31 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 32 consumed LeftBrace
parse_statement: starting at position 33 (Ident(n))
consume_token: position 33 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 33 (Ident(n))
consume_token: position 33 consumed Ident
parse_primary: success - parsed identifier (n)
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace at position 34
parse_expression: starting at position 33 (Ident(n))
consume_token: position 33 consumed Ident
parse_primary: success - parsed identifier (n)
parse_expression: success - parsed precedence expression
consume_token: position 34 consumed RightBrace
consume_token: position 35 consumed Else
consume_token: position 36 consumed LeftBrace
parse_statement: starting at position 37 (Ident(fibonacci))
consume_token: position 37 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 37 (Ident(fibonacci))
consume_token: position 37 consumed Ident
parse_primary: success - parsed identifier (fibonacci)
consume_token: position 38 consumed LeftParen
parse_expression: starting at position 39 (Ident(n))
consume_token: position 39 consumed Ident
parse_primary: success - parsed identifier (n)
consume_token: position 40 consumed Dot
consume_token: position 41 consumed Ident
consume_token: position 42 consumed LeftParen
parse_expression: starting at position 43 (Int(1))
consume_token: position 43 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 44 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 45 consumed RightParen
consume_token: position 46 consumed Dot
consume_token: position 47 consumed Ident
consume_token: position 48 consumed LeftParen
parse_expression: starting at position 49 (Ident(fibonacci))
consume_token: position 49 consumed Ident
parse_primary: success - parsed identifier (fibonacci)
consume_token: position 50 consumed LeftParen
parse_expression: starting at position 51 (Ident(n))
consume_token: position 51 consumed Ident
parse_primary: success - parsed identifier (n)
consume_token: position 52 consumed Dot
consume_token: position 53 consumed Ident
consume_token: position 54 consumed LeftParen
parse_expression: starting at position 55 (Int(2))
consume_token: position 55 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 56 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 57 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 58 consumed RightParen
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace at position 59
parse_expression: starting at position 37 (Ident(fibonacci))
consume_token: position 37 consumed Ident
parse_primary: success - parsed identifier (fibonacci)
consume_token: position 38 consumed LeftParen
parse_expression: starting at position 39 (Ident(n))
consume_token: position 39 consumed Ident
parse_primary: success - parsed identifier (n)
consume_token: position 40 consumed Dot
consume_token: position 41 consumed Ident
consume_token: position 42 consumed LeftParen
parse_expression: starting at position 43 (Int(1))
consume_token: position 43 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 44 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 45 consumed RightParen
consume_token: position 46 consumed Dot
consume_token: position 47 consumed Ident
consume_token: position 48 consumed LeftParen
parse_expression: starting at position 49 (Ident(fibonacci))
consume_token: position 49 consumed Ident
parse_primary: success - parsed identifier (fibonacci)
consume_token: position 50 consumed LeftParen
parse_expression: starting at position 51 (Ident(n))
consume_token: position 51 consumed Ident
parse_primary: success - parsed identifier (n)
consume_token: position 52 consumed Dot
consume_token: position 53 consumed Ident
consume_token: position 54 consumed LeftParen
parse_expression: starting at position 55 (Int(2))
consume_token: position 55 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 56 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 57 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 58 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 59 consumed RightBrace
parse_expression: success - parsed precedence expression
parse_statement: success - parsed block-like expression statement
parse_statement: starting at position 60 (Semicolon)
parse_statement: falling back to expression statement
parse_expression: starting at position 60 (Semicolon)
parse_primary: failed - no matching primary expression for Semicolon
parse_expression: failed - Expected expression, found Semicolon
parse_statement: failed - Expected expression, found Semicolon
parse_expression: starting at position 60 (Semicolon)
parse_primary: failed - no matching primary expression for Semicolon
parse_expression: failed - Expected expression, found Semicolon
parse_expression: starting at position 10 (Fn)
parse_primary: failed - no matching primary expression for Fn
parse_expression: failed - Expected expression, found Fn
parse_expression: failed - Expected statement or expression
```

# Result
```rust
Err(
    "Failed to parse: Expected statement or expression:\nvar fibonacci = {\n        var cache = {};\n\n        fn original(n) {\n    count = count.op_add(1);\n    if n.op_le(1) {\n    n\n} else {\n    fibonacci(n.op_sub(1)).op_add(fibonacci(n.op_sub(2)))\n};\n}\n\n        fn cached(n) {\n            var args_key = str(n);\n            if args_key in cache {\n                cache[args_key]\n            } else {\n                var result = original(n);\n                cache[args_key] = result;\n                result\n            }\n        }\n\n        cached\n    };",
)
```

# Lex
```rust
Ok(
    [
        0: Token(Fn),
        1: Token(Ident, "memoize"),
        2: Token(LeftParen),
        3: Token(Ident, "f"),
        4: Token(RightParen),
        5: Token(LeftBrace),
        6: Token(Var),
        7: Token(Ident, "param_list"),
        8: Token(Equal),
        9: Token(Ident, "join"),
        10: Token(LeftParen),
        11: Token(Ident, "f"),
        12: Token(Dot),
        13: Token(Ident, "params"),
        14: Token(Comma),
        15: Token(String, ", "),
        16: Token(RightParen),
        17: Token(Semicolon),
        18: Token(Ident, "parse"),
        19: Token(LeftParen),
        20: Token(StringPart, "var "),
        21: Token(InterpolationStart),
        22: Token(Ident, "f"),
        23: Token(Dot),
        24: Token(Ident, "name"),
        25: Token(InterpolationEnd),
        26: Token(StringPart, " = {\n        var cache = {};\n\n        fn original("),
        27: Token(InterpolationStart),
        28: Token(Ident, "param_list"),
        29: Token(InterpolationEnd),
        30: Token(StringPart, ") "),
        31: Token(InterpolationStart),
        32: Token(Ident, "f"),
        33: Token(Dot),
        34: Token(Ident, "body"),
        35: Token(InterpolationEnd),
        36: Token(StringPart, "\n\n        fn cached("),
        37: Token(InterpolationStart),
        38: Token(Ident, "param_list"),
        39: Token(InterpolationEnd),
        40: Token(StringPart, ") {\n            var args_key = str("),
        41: Token(InterpolationStart),
        42: Token(Ident, "param_list"),
        43: Token(InterpolationEnd),
        44: Token(StringPart, ");\n            if args_key in cache {\n                cache[args_key]\n            } else {\n                var result = original("),
        45: Token(InterpolationStart),
        46: Token(Ident, "param_list"),
        47: Token(InterpolationEnd),
        48: Token(StringPart, ");\n                cache[args_key] = result;\n                result\n            }\n        }\n\n        cached\n    };"),
        49: Token(RightParen),
        50: Token(RightBrace),
        51: Token(Var),
        52: Token(Ident, "count"),
        53: Token(Equal),
        54: Token(Int, "0"),
        55: Token(Semicolon),
        56: Token(Hash),
        57: Token(LeftBracket),
        58: Token(Ident, "memoize"),
        59: Token(RightBracket),
        60: Token(Fn),
        61: Token(Ident, "fibonacci"),
        62: Token(LeftParen),
        63: Token(Ident, "n"),
        64: Token(RightParen),
        65: Token(LeftBrace),
        66: Token(Ident, "count"),
        67: Token(PlusEqual),
        68: Token(Int, "1"),
        69: Token(Semicolon),
        70: Token(If),
        71: Token(Ident, "n"),
        72: Token(LessEqual),
        73: Token(Int, "1"),
        74: Token(LeftBrace),
        75: Token(Ident, "n"),
        76: Token(RightBrace),
        77: Token(Else),
        78: Token(LeftBrace),
        79: Token(Ident, "fibonacci"),
        80: Token(LeftParen),
        81: Token(Ident, "n"),
        82: Token(Minus),
        83: Token(Int, "1"),
        84: Token(RightParen),
        85: Token(Plus),
        86: Token(Ident, "fibonacci"),
        87: Token(LeftParen),
        88: Token(Ident, "n"),
        89: Token(Minus),
        90: Token(Int, "2"),
        91: Token(RightParen),
        92: Token(RightBrace),
        93: Token(RightBrace),
        94: Token(Var),
        95: Token(Ident, "f"),
        96: Token(Equal),
        97: Token(Ident, "fibonacci"),
        98: Token(LeftParen),
        99: Token(Int, "10"),
        100: Token(RightParen),
        101: Token(Semicolon),
        102: Token(Ident, "assert"),
        103: Token(LeftParen),
        104: Token(Ident, "f"),
        105: Token(EqualEqual),
        106: Token(Int, "55"),
        107: Token(Comma),
        108: Token(String, "f"),
        109: Token(RightParen),
        110: Token(Semicolon),
        111: Token(Ident, "print"),
        112: Token(LeftParen),
        113: Token(Ident, "count"),
        114: Token(RightParen),
        115: Token(Semicolon),
        116: Token(Ident, "assert"),
        117: Token(LeftParen),
        118: Token(Ident, "count"),
        119: Token(EqualEqual),
        120: Token(Int, "11"),
        121: Token(Comma),
        122: Token(String, "count"),
        123: Token(RightParen),
        124: Token(Semicolon),
        125: Token(Eof)
    ],
)
```

# Parse
```rust
Ok(
    Program(
        [
            FnDecl {
                name: "memoize",
                params: [
                    Parameter {
                        name: "f",
                        default: None,
                        kind: Regular,
                    },
                ],
                body: Block {
                    statements: [
                        VarDecl {
                            pattern: Variable(
                                "param_list",
                            ),
                            value: Some(
                                FunctionCall(
                                    Identifier(
                                        "join",
                                    ),
                                    [
                                        GetAttr(
                                            Identifier(
                                                "f",
                                            ),
                                            "params",
                                        ),
                                        Literal(
                                            String(
                                                ", ",
                                            ),
                                        ),
                                    ],
                                ),
                            ),
                        },
                    ],
                    final_expr: Some(
                        FunctionCall(
                            Identifier(
                                "parse",
                            ),
                            [
                                InterpolatedString(
                                    [
                                        Text(
                                            "var ",
                                        ),
                                        Expression(
                                            GetAttr(
                                                Identifier(
                                                    "f",
                                                ),
                                                "name",
                                            ),
                                        ),
                                        Text(
                                            " = {\n        var cache = {};\n\n        fn original(",
                                        ),
                                        Expression(
                                            Identifier(
                                                "param_list",
                                            ),
                                        ),
                                        Text(
                                            ") ",
                                        ),
                                        Expression(
                                            GetAttr(
                                                Identifier(
                                                    "f",
                                                ),
                                                "body",
                                            ),
                                        ),
                                        Text(
                                            "\n\n        fn cached(",
                                        ),
                                        Expression(
                                            Identifier(
                                                "param_list",
                                            ),
                                        ),
                                        Text(
                                            ") {\n            var args_key = str(",
                                        ),
                                        Expression(
                                            Identifier(
                                                "param_list",
                                            ),
                                        ),
                                        Text(
                                            ");\n            if args_key in cache {\n                cache[args_key]\n            } else {\n                var result = original(",
                                        ),
                                        Expression(
                                            Identifier(
                                                "param_list",
                                            ),
                                        ),
                                        Text(
                                            ");\n                cache[args_key] = result;\n                result\n            }\n        }\n\n        cached\n    };",
                                        ),
                                    ],
                                ),
                            ],
                        ),
                    ),
                },
                is_pub: false,
            },
            VarDecl {
                pattern: Variable(
                    "count",
                ),
                value: Some(
                    Literal(
                        Int(
                            0,
                        ),
                    ),
                ),
            },
            Macro {
                name: "memoize",
                args: [],
                statement: FnDecl {
                    name: "fibonacci",
                    params: [
                        Parameter {
                            name: "n",
                            default: None,
                            kind: Regular,
                        },
                    ],
                    body: Block {
                        statements: [
                            Assignment {
                                target: Identifier(
                                    "count",
                                ),
                                op: AddAssign,
                                value: Literal(
                                    Int(
                                        1,
                                    ),
                                ),
                            },
                            Expression(
                                If {
                                    condition: Le(
                                        Identifier(
                                            "n",
                                        ),
                                        Literal(
                                            Int(
                                                1,
                                            ),
                                        ),
                                    ),
                                    then_expr: Block {
                                        statements: [],
                                        final_expr: Some(
                                            Identifier(
                                                "n",
                                            ),
                                        ),
                                    },
                                    else_expr: Some(
                                        Block {
                                            statements: [],
                                            final_expr: Some(
                                                Add(
                                                    FunctionCall(
                                                        Identifier(
                                                            "fibonacci",
                                                        ),
                                                        [
                                                            Sub(
                                                                Identifier(
                                                                    "n",
                                                                ),
                                                                Literal(
                                                                    Int(
                                                                        1,
                                                                    ),
                                                                ),
                                                            ),
                                                        ],
                                                    ),
                                                    FunctionCall(
                                                        Identifier(
                                                            "fibonacci",
                                                        ),
                                                        [
                                                            Sub(
                                                                Identifier(
                                                                    "n",
                                                                ),
                                                                Literal(
                                                                    Int(
                                                                        2,
                                                                    ),
                                                                ),
                                                            ),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                },
                            ),
                        ],
                        final_expr: None,
                    },
                    is_pub: false,
                },
            },
            VarDecl {
                pattern: Variable(
                    "f",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "fibonacci",
                        ),
                        [
                            Literal(
                                Int(
                                    10,
                                ),
                            ),
                        ],
                    ),
                ),
            },
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            Identifier(
                                "f",
                            ),
                            Literal(
                                Int(
                                    55,
                                ),
                            ),
                        ),
                        Literal(
                            String(
                                "f",
                            ),
                        ),
                    ],
                ),
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "print",
                    ),
                    [
                        Identifier(
                            "count",
                        ),
                    ],
                ),
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            Identifier(
                                "count",
                            ),
                            Literal(
                                Int(
                                    11,
                                ),
                            ),
                        ),
                        Literal(
                            String(
                                "count",
                            ),
                        ),
                    ],
                ),
            ),
        ],
    ),
)
```

# Eval
```rust
Ok(
    RustValue(
        EvalProgram {
            statements: [
                RustValue(
                    EvalFunction {
                        data: FunctionData {
                            name: "memoize",
                            params: [
                                (
                                    "f",
                                    None,
                                    Regular,
                                ),
                            ],
                            body: RustValue(
                                EvalBlock {
                                    statements: [
                                        RustValue(
                                            EvalDeclare {
                                                name: "param_list",
                                                init_expr: Some(
                                                    RustValue(
                                                        EvalCall {
                                                            func_expr: RustValue(
                                                                EvalVariable {
                                                                    name: "join",
                                                                },
                                                            ),
                                                            args: [
                                                                RustValue(
                                                                    EvalGetAttr {
                                                                        obj_expr: RustValue(
                                                                            EvalVariable {
                                                                                name: "f",
                                                                            },
                                                                        ),
                                                                        attr_name: "params",
                                                                    },
                                                                ),
                                                                RustValue(
                                                                    EvalLiteral {
                                                                        value: String(
                                                                            ", ",
                                                                        ),
                                                                    },
                                                                ),
                                                            ],
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                    ],
                                    final_expr: Some(
                                        RustValue(
                                            EvalCall {
                                                func_expr: RustValue(
                                                    EvalVariable {
                                                        name: "parse",
                                                    },
                                                ),
                                                args: [
                                                    RustValue(
                                                        EvalCall {
                                                            func_expr: RustValue(
                                                                EvalGetAttr {
                                                                    obj_expr: RustValue(
                                                                        EvalLiteral {
                                                                            value: String(
                                                                                "var ",
                                                                            ),
                                                                        },
                                                                    ),
                                                                    attr_name: "op_add",
                                                                },
                                                            ),
                                                            args: [
                                                                RustValue(
                                                                    EvalCall {
                                                                        func_expr: RustValue(
                                                                            EvalVariable {
                                                                                name: "str",
                                                                            },
                                                                        ),
                                                                        args: [
                                                                            RustValue(
                                                                                EvalGetAttr {
                                                                                    obj_expr: RustValue(
                                                                                        EvalVariable {
                                                                                            name: "f",
                                                                                        },
                                                                                    ),
                                                                                    attr_name: "name",
                                                                                },
                                                                            ),
                                                                        ],
                                                                    },
                                                                ),
                                                                RustValue(
                                                                    EvalLiteral {
                                                                        value: String(
                                                                            " = {\n        var cache = {};\n\n        fn original(",
                                                                        ),
                                                                    },
                                                                ),
                                                                RustValue(
                                                                    EvalCall {
                                                                        func_expr: RustValue(
                                                                            EvalVariable {
                                                                                name: "str",
                                                                            },
                                                                        ),
                                                                        args: [
                                                                            RustValue(
                                                                                EvalVariable {
                                                                                    name: "param_list",
                                                                                },
                                                                            ),
                                                                        ],
                                                                    },
                                                                ),
                                                                RustValue(
                                                                    EvalLiteral {
                                                                        value: String(
                                                                            ") ",
                                                                        ),
                                                                    },
                                                                ),
                                                                RustValue(
                                                                    EvalCall {
                                                                        func_expr: RustValue(
                                                                            EvalVariable {
                                                                                name: "str",
                                                                            },
                                                                        ),
                                                                        args: [
                                                                            RustValue(
                                                                                EvalGetAttr {
                                                                                    obj_expr: RustValue(
                                                                                        EvalVariable {
                                                                                            name: "f",
                                                                                        },
                                                                                    ),
                                                                                    attr_name: "body",
                                                                                },
                                                                            ),
                                                                        ],
                                                                    },
                                                                ),
                                                                RustValue(
                                                                    EvalLiteral {
                                                                        value: String(
                                                                            "\n\n        fn cached(",
                                                                        ),
                                                                    },
                                                                ),
                                                                RustValue(
                                                                    EvalCall {
                                                                        func_expr: RustValue(
                                                                            EvalVariable {
                                                                                name: "str",
                                                                            },
                                                                        ),
                                                                        args: [
                                                                            RustValue(
                                                                                EvalVariable {
                                                                                    name: "param_list",
                                                                                },
                                                                            ),
                                                                        ],
                                                                    },
                                                                ),
                                                                RustValue(
                                                                    EvalLiteral {
                                                                        value: String(
                                                                            ") {\n            var args_key = str(",
                                                                        ),
                                                                    },
                                                                ),
                                                                RustValue(
                                                                    EvalCall {
                                                                        func_expr: RustValue(
                                                                            EvalVariable {
                                                                                name: "str",
                                                                            },
                                                                        ),
                                                                        args: [
                                                                            RustValue(
                                                                                EvalVariable {
                                                                                    name: "param_list",
                                                                                },
                                                                            ),
                                                                        ],
                                                                    },
                                                                ),
                                                                RustValue(
                                                                    EvalLiteral {
                                                                        value: String(
                                                                            ");\n            if args_key in cache {\n                cache[args_key]\n            } else {\n                var result = original(",
                                                                        ),
                                                                    },
                                                                ),
                                                                RustValue(
                                                                    EvalCall {
                                                                        func_expr: RustValue(
                                                                            EvalVariable {
                                                                                name: "str",
                                                                            },
                                                                        ),
                                                                        args: [
                                                                            RustValue(
                                                                                EvalVariable {
                                                                                    name: "param_list",
                                                                                },
                                                                            ),
                                                                        ],
                                                                    },
                                                                ),
                                                                RustValue(
                                                                    EvalLiteral {
                                                                        value: String(
                                                                            ");\n                cache[args_key] = result;\n                result\n            }\n        }\n\n        cached\n    };",
                                                                        ),
                                                                    },
                                                                ),
                                                            ],
                                                        },
                                                    ),
                                                ],
                                            },
                                        ),
                                    ),
                                },
                            ),
                        },
                    },
                ),
                RustValue(
                    EvalDeclare {
                        name: "count",
                        init_expr: Some(
                            RustValue(
                                EvalLiteral {
                                    value: Int(
                                        0,
                                    ),
                                },
                            ),
                        ),
                    },
                ),
                RustValue(
                    EvalMacro {
                        data: MacroData {
                            macro_fn: RustValue(
                                EvalVariable {
                                    name: "memoize",
                                },
                            ),
                            target: RustValue(
                                EvalFunction {
                                    data: FunctionData {
                                        name: "fibonacci",
                                        params: [
                                            (
                                                "n",
                                                None,
                                                Regular,
                                            ),
                                        ],
                                        body: RustValue(
                                            EvalBlock {
                                                statements: [
                                                    RustValue(
                                                        EvalAssign {
                                                            name: "count",
                                                            expr: RustValue(
                                                                EvalCall {
                                                                    func_expr: RustValue(
                                                                        EvalGetAttr {
                                                                            obj_expr: RustValue(
                                                                                EvalVariable {
                                                                                    name: "count",
                                                                                },
                                                                            ),
                                                                            attr_name: "op_add",
                                                                        },
                                                                    ),
                                                                    args: [
                                                                        RustValue(
                                                                            EvalLiteral {
                                                                                value: Int(
                                                                                    1,
                                                                                ),
                                                                            },
                                                                        ),
                                                                    ],
                                                                },
                                                            ),
                                                        },
                                                    ),
                                                    RustValue(
                                                        EvalIf {
                                                            condition: RustValue(
                                                                EvalCall {
                                                                    func_expr: RustValue(
                                                                        EvalGetAttr {
                                                                            obj_expr: RustValue(
                                                                                EvalVariable {
                                                                                    name: "n",
                                                                                },
                                                                            ),
                                                                            attr_name: "op_le",
                                                                        },
                                                                    ),
                                                                    args: [
                                                                        RustValue(
                                                                            EvalLiteral {
                                                                                value: Int(
                                                                                    1,
                                                                                ),
                                                                            },
                                                                        ),
                                                                    ],
                                                                },
                                                            ),
                                                            then_expr: RustValue(
                                                                EvalBlock {
                                                                    statements: [],
                                                                    final_expr: Some(
                                                                        RustValue(
                                                                            EvalVariable {
                                                                                name: "n",
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                            else_expr: Some(
                                                                RustValue(
                                                                    EvalBlock {
                                                                        statements: [],
                                                                        final_expr: Some(
                                                                            RustValue(
                                                                                EvalCall {
                                                                                    func_expr: RustValue(
                                                                                        EvalGetAttr {
                                                                                            obj_expr: RustValue(
                                                                                                EvalCall {
                                                                                                    func_expr: RustValue(
                                                                                                        EvalVariable {
                                                                                                            name: "fibonacci",
                                                                                                        },
                                                                                                    ),
                                                                                                    args: [
                                                                                                        RustValue(
                                                                                                            EvalCall {
                                                                                                                func_expr: RustValue(
                                                                                                                    EvalGetAttr {
                                                                                                                        obj_expr: RustValue(
                                                                                                                            EvalVariable {
                                                                                                                                name: "n",
                                                                                                                            },
                                                                                                                        ),
                                                                                                                        attr_name: "op_sub",
                                                                                                                    },
                                                                                                                ),
                                                                                                                args: [
                                                                                                                    RustValue(
                                                                                                                        EvalLiteral {
                                                                                                                            value: Int(
                                                                                                                                1,
                                                                                                                            ),
                                                                                                                        },
                                                                                                                    ),
                                                                                                                ],
                                                                                                            },
                                                                                                        ),
                                                                                                    ],
                                                                                                },
                                                                                            ),
                                                                                            attr_name: "op_add",
                                                                                        },
                                                                                    ),
                                                                                    args: [
                                                                                        RustValue(
                                                                                            EvalCall {
                                                                                                func_expr: RustValue(
                                                                                                    EvalVariable {
                                                                                                        name: "fibonacci",
                                                                                                    },
                                                                                                ),
                                                                                                args: [
                                                                                                    RustValue(
                                                                                                        EvalCall {
                                                                                                            func_expr: RustValue(
                                                                                                                EvalGetAttr {
                                                                                                                    obj_expr: RustValue(
                                                                                                                        EvalVariable {
                                                                                                                            name: "n",
                                                                                                                        },
                                                                                                                    ),
                                                                                                                    attr_name: "op_sub",
                                                                                                                },
                                                                                                            ),
                                                                                                            args: [
                                                                                                                RustValue(
                                                                                                                    EvalLiteral {
                                                                                                                        value: Int(
                                                                                                                            2,
                                                                                                                        ),
                                                                                                                    },
                                                                                                                ),
                                                                                                            ],
                                                                                                        },
                                                                                                    ),
                                                                                                ],
                                                                                            },
                                                                                        ),
                                                                                    ],
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ],
                                                final_expr: None,
                                            },
                                        ),
                                    },
                                },
                            ),
                            args: [],
                        },
                    },
                ),
                RustValue(
                    EvalDeclare {
                        name: "f",
                        init_expr: Some(
                            RustValue(
                                EvalCall {
                                    func_expr: RustValue(
                                        EvalVariable {
                                            name: "fibonacci",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: Int(
                                                    10,
                                                ),
                                            },
                                        ),
                                    ],
                                },
                            ),
                        ),
                    },
                ),
                RustValue(
                    EvalCall {
                        func_expr: RustValue(
                            EvalVariable {
                                name: "assert",
                            },
                        ),
                        args: [
                            RustValue(
                                EvalCall {
                                    func_expr: RustValue(
                                        EvalGetAttr {
                                            obj_expr: RustValue(
                                                EvalVariable {
                                                    name: "f",
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: Int(
                                                    55,
                                                ),
                                            },
                                        ),
                                    ],
                                },
                            ),
                            RustValue(
                                EvalLiteral {
                                    value: String(
                                        "f",
                                    ),
                                },
                            ),
                        ],
                    },
                ),
                RustValue(
                    EvalCall {
                        func_expr: RustValue(
                            EvalVariable {
                                name: "print",
                            },
                        ),
                        args: [
                            RustValue(
                                EvalVariable {
                                    name: "count",
                                },
                            ),
                        ],
                    },
                ),
                RustValue(
                    EvalCall {
                        func_expr: RustValue(
                            EvalVariable {
                                name: "assert",
                            },
                        ),
                        args: [
                            RustValue(
                                EvalCall {
                                    func_expr: RustValue(
                                        EvalGetAttr {
                                            obj_expr: RustValue(
                                                EvalVariable {
                                                    name: "count",
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: Int(
                                                    11,
                                                ),
                                            },
                                        ),
                                    ],
                                },
                            ),
                            RustValue(
                                EvalLiteral {
                                    value: String(
                                        "count",
                                    ),
                                },
                            ),
                        ],
                    },
                ),
            ],
        },
    ),
)
```