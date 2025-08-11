# Program
Status: 🟢
Assertions: 2

```rustleaf
fn memoize(f) {
    var param_list = join(f.params, ", ");

    parse("var ${f.name} = {
        var cache = {};

        fn original(${param_list}) ${f.body}

        fn cached(${param_list}) {
            var args_key = str(${param_list});
            if args_key in cache {
                return cache[args_key];
            } else {
                var result = original(${param_list});
                cache[args_key] = result;
                return result;
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
        return n;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
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
parse_statement: starting at position 75 (Return)
consume_token: position 75 consumed Return
parse_expression: starting at position 76 (Ident(n))
consume_token: position 76 consumed Ident
parse_primary: success - parsed identifier (n)
parse_expression: success - parsed precedence expression
consume_token: position 77 consumed Semicolon
parse_statement: success - parsed return statement
consume_token: position 78 consumed RightBrace
consume_token: position 79 consumed Else
consume_token: position 80 consumed LeftBrace
parse_statement: starting at position 81 (Return)
consume_token: position 81 consumed Return
parse_expression: starting at position 82 (Ident(fibonacci))
consume_token: position 82 consumed Ident
parse_primary: success - parsed identifier (fibonacci)
consume_token: position 83 consumed LeftParen
parse_expression: starting at position 84 (Ident(n))
consume_token: position 84 consumed Ident
parse_primary: success - parsed identifier (n)
consume_token: position 85 consumed Minus
consume_token: position 86 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 87 consumed RightParen
consume_token: position 88 consumed Plus
consume_token: position 89 consumed Ident
parse_primary: success - parsed identifier (fibonacci)
consume_token: position 90 consumed LeftParen
parse_expression: starting at position 91 (Ident(n))
consume_token: position 91 consumed Ident
parse_primary: success - parsed identifier (n)
consume_token: position 92 consumed Minus
consume_token: position 93 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 94 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 95 consumed Semicolon
parse_statement: success - parsed return statement
consume_token: position 96 consumed RightBrace
parse_expression: success - parsed precedence expression
parse_statement: success - parsed block-like expression statement
consume_token: position 97 consumed RightBrace
parse_statement: success - parsed function declaration
parse_statement: success - parsed macro
parse_program: parsing statement at position 98 (Var)
parse_statement: starting at position 98 (Var)
consume_token: position 98 consumed Var
consume_token: position 99 consumed Ident
consume_token: position 100 consumed Equal
parse_expression: starting at position 101 (Ident(fibonacci))
consume_token: position 101 consumed Ident
parse_primary: success - parsed identifier (fibonacci)
consume_token: position 102 consumed LeftParen
parse_expression: starting at position 103 (Int(10))
consume_token: position 103 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 104 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 105 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 106 (Ident(assert))
parse_statement: starting at position 106 (Ident(assert))
consume_token: position 106 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 106 (Ident(assert))
consume_token: position 106 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 107 consumed LeftParen
parse_expression: starting at position 108 (Ident(f))
consume_token: position 108 consumed Ident
parse_primary: success - parsed identifier (f)
consume_token: position 109 consumed EqualEqual
consume_token: position 110 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 111 consumed Comma
parse_expression: starting at position 112 (String(f))
consume_token: position 112 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 113 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 114 consumed Semicolon
parse_program: parsing statement at position 115 (Ident(print))
parse_statement: starting at position 115 (Ident(print))
consume_token: position 115 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 115 (Ident(print))
consume_token: position 115 consumed Ident
parse_primary: success - parsed identifier (print)
consume_token: position 116 consumed LeftParen
parse_expression: starting at position 117 (Ident(count))
consume_token: position 117 consumed Ident
parse_primary: success - parsed identifier (count)
parse_expression: success - parsed precedence expression
consume_token: position 118 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 119 consumed Semicolon
parse_program: parsing statement at position 120 (Ident(assert))
parse_statement: starting at position 120 (Ident(assert))
consume_token: position 120 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 120 (Ident(assert))
consume_token: position 120 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 121 consumed LeftParen
parse_expression: starting at position 122 (Ident(count))
consume_token: position 122 consumed Ident
parse_primary: success - parsed identifier (count)
consume_token: position 123 consumed EqualEqual
consume_token: position 124 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 125 consumed Comma
parse_expression: starting at position 126 (String(count))
consume_token: position 126 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 127 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 128 consumed Semicolon
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
parse_statement: starting at position 33 (Return)
consume_token: position 33 consumed Return
parse_expression: starting at position 34 (Ident(n))
consume_token: position 34 consumed Ident
parse_primary: success - parsed identifier (n)
parse_expression: success - parsed precedence expression
consume_token: position 35 consumed Semicolon
parse_statement: success - parsed return statement
consume_token: position 36 consumed RightBrace
consume_token: position 37 consumed Else
consume_token: position 38 consumed LeftBrace
parse_statement: starting at position 39 (Return)
consume_token: position 39 consumed Return
parse_expression: starting at position 40 (Ident(fibonacci))
consume_token: position 40 consumed Ident
parse_primary: success - parsed identifier (fibonacci)
consume_token: position 41 consumed LeftParen
parse_expression: starting at position 42 (Ident(n))
consume_token: position 42 consumed Ident
parse_primary: success - parsed identifier (n)
consume_token: position 43 consumed Dot
consume_token: position 44 consumed Ident
consume_token: position 45 consumed LeftParen
parse_expression: starting at position 46 (Int(1))
consume_token: position 46 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 47 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 48 consumed RightParen
consume_token: position 49 consumed Dot
consume_token: position 50 consumed Ident
consume_token: position 51 consumed LeftParen
parse_expression: starting at position 52 (Ident(fibonacci))
consume_token: position 52 consumed Ident
parse_primary: success - parsed identifier (fibonacci)
consume_token: position 53 consumed LeftParen
parse_expression: starting at position 54 (Ident(n))
consume_token: position 54 consumed Ident
parse_primary: success - parsed identifier (n)
consume_token: position 55 consumed Dot
consume_token: position 56 consumed Ident
consume_token: position 57 consumed LeftParen
parse_expression: starting at position 58 (Int(2))
consume_token: position 58 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 59 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 60 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 61 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 62 consumed Semicolon
parse_statement: success - parsed return statement
consume_token: position 63 consumed RightBrace
parse_expression: success - parsed precedence expression
parse_statement: success - parsed block-like expression statement
parse_statement: starting at position 64 (Semicolon)
consume_token: position 64 consumed Semicolon
parse_statement: success - parsed empty statement
consume_token: position 65 consumed RightBrace
parse_statement: success - parsed function declaration
parse_statement: starting at position 66 (Fn)
consume_token: position 66 consumed Fn
consume_token: position 67 consumed Ident
consume_token: position 68 consumed LeftParen
consume_token: position 69 consumed Ident
consume_token: position 70 consumed RightParen
consume_token: position 71 consumed LeftBrace
parse_statement: starting at position 72 (Var)
consume_token: position 72 consumed Var
consume_token: position 73 consumed Ident
consume_token: position 74 consumed Equal
parse_expression: starting at position 75 (Ident(str))
consume_token: position 75 consumed Ident
parse_primary: success - parsed identifier (str)
consume_token: position 76 consumed LeftParen
parse_expression: starting at position 77 (Ident(n))
consume_token: position 77 consumed Ident
parse_primary: success - parsed identifier (n)
parse_expression: success - parsed precedence expression
consume_token: position 78 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 79 consumed Semicolon
parse_statement: success - parsed var declaration
parse_statement: starting at position 80 (If)
parse_expression: starting at position 80 (If)
consume_token: position 80 consumed If
parse_primary: success - parsing if expression
parse_expression: starting at position 81 (Ident(args_key))
consume_token: position 81 consumed Ident
parse_primary: success - parsed identifier (args_key)
consume_token: position 82 consumed In
consume_token: position 83 consumed Ident
parse_primary: success - parsed identifier (cache)
parse_expression: success - parsed precedence expression
consume_token: position 84 consumed LeftBrace
parse_statement: starting at position 85 (Return)
consume_token: position 85 consumed Return
parse_expression: starting at position 86 (Ident(cache))
consume_token: position 86 consumed Ident
parse_primary: success - parsed identifier (cache)
consume_token: position 87 consumed LeftBracket
parse_expression: starting at position 88 (Ident(args_key))
consume_token: position 88 consumed Ident
parse_primary: success - parsed identifier (args_key)
parse_expression: success - parsed precedence expression
consume_token: position 89 consumed RightBracket
parse_expression: success - parsed precedence expression
consume_token: position 90 consumed Semicolon
parse_statement: success - parsed return statement
consume_token: position 91 consumed RightBrace
consume_token: position 92 consumed Else
consume_token: position 93 consumed LeftBrace
parse_statement: starting at position 94 (Var)
consume_token: position 94 consumed Var
consume_token: position 95 consumed Ident
consume_token: position 96 consumed Equal
parse_expression: starting at position 97 (Ident(original))
consume_token: position 97 consumed Ident
parse_primary: success - parsed identifier (original)
consume_token: position 98 consumed LeftParen
parse_expression: starting at position 99 (Ident(n))
consume_token: position 99 consumed Ident
parse_primary: success - parsed identifier (n)
parse_expression: success - parsed precedence expression
consume_token: position 100 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 101 consumed Semicolon
parse_statement: success - parsed var declaration
parse_statement: starting at position 102 (Ident(cache))
consume_token: position 102 consumed Ident
consume_token: position 103 consumed LeftBracket
parse_expression: starting at position 104 (Ident(args_key))
consume_token: position 104 consumed Ident
parse_primary: success - parsed identifier (args_key)
parse_expression: success - parsed precedence expression
consume_token: position 105 consumed RightBracket
consume_token: position 106 consumed Equal
parse_expression: starting at position 107 (Ident(result))
consume_token: position 107 consumed Ident
parse_primary: success - parsed identifier (result)
parse_expression: success - parsed precedence expression
consume_token: position 108 consumed Semicolon
parse_statement: success - parsed assignment
parse_statement: starting at position 109 (Return)
consume_token: position 109 consumed Return
parse_expression: starting at position 110 (Ident(result))
consume_token: position 110 consumed Ident
parse_primary: success - parsed identifier (result)
parse_expression: success - parsed precedence expression
consume_token: position 111 consumed Semicolon
parse_statement: success - parsed return statement
consume_token: position 112 consumed RightBrace
parse_expression: success - parsed precedence expression
parse_statement: success - parsed block-like expression statement
consume_token: position 113 consumed RightBrace
parse_statement: success - parsed function declaration
parse_statement: starting at position 114 (Ident(cached))
consume_token: position 114 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 114 (Ident(cached))
consume_token: position 114 consumed Ident
parse_primary: success - parsed identifier (cached)
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace at position 115
parse_expression: starting at position 114 (Ident(cached))
consume_token: position 114 consumed Ident
parse_primary: success - parsed identifier (cached)
parse_expression: success - parsed precedence expression
consume_token: position 115 consumed RightBrace
parse_expression: success - parsed precedence expression
consume_token: position 116 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsed 1 statements
11
```

# Result
```rust
Ok(
    Unit,
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
        44: Token(StringPart, ");\n            if args_key in cache {\n                return cache[args_key];\n            } else {\n                var result = original("),
        45: Token(InterpolationStart),
        46: Token(Ident, "param_list"),
        47: Token(InterpolationEnd),
        48: Token(StringPart, ");\n                cache[args_key] = result;\n                return result;\n            }\n        }\n\n        cached\n    };"),
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
        75: Token(Return),
        76: Token(Ident, "n"),
        77: Token(Semicolon),
        78: Token(RightBrace),
        79: Token(Else),
        80: Token(LeftBrace),
        81: Token(Return),
        82: Token(Ident, "fibonacci"),
        83: Token(LeftParen),
        84: Token(Ident, "n"),
        85: Token(Minus),
        86: Token(Int, "1"),
        87: Token(RightParen),
        88: Token(Plus),
        89: Token(Ident, "fibonacci"),
        90: Token(LeftParen),
        91: Token(Ident, "n"),
        92: Token(Minus),
        93: Token(Int, "2"),
        94: Token(RightParen),
        95: Token(Semicolon),
        96: Token(RightBrace),
        97: Token(RightBrace),
        98: Token(Var),
        99: Token(Ident, "f"),
        100: Token(Equal),
        101: Token(Ident, "fibonacci"),
        102: Token(LeftParen),
        103: Token(Int, "10"),
        104: Token(RightParen),
        105: Token(Semicolon),
        106: Token(Ident, "assert"),
        107: Token(LeftParen),
        108: Token(Ident, "f"),
        109: Token(EqualEqual),
        110: Token(Int, "55"),
        111: Token(Comma),
        112: Token(String, "f"),
        113: Token(RightParen),
        114: Token(Semicolon),
        115: Token(Ident, "print"),
        116: Token(LeftParen),
        117: Token(Ident, "count"),
        118: Token(RightParen),
        119: Token(Semicolon),
        120: Token(Ident, "assert"),
        121: Token(LeftParen),
        122: Token(Ident, "count"),
        123: Token(EqualEqual),
        124: Token(Int, "11"),
        125: Token(Comma),
        126: Token(String, "count"),
        127: Token(RightParen),
        128: Token(Semicolon),
        129: Token(Eof)
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
                                            ");\n            if args_key in cache {\n                return cache[args_key];\n            } else {\n                var result = original(",
                                        ),
                                        Expression(
                                            Identifier(
                                                "param_list",
                                            ),
                                        ),
                                        Text(
                                            ");\n                cache[args_key] = result;\n                return result;\n            }\n        }\n\n        cached\n    };",
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
                                        statements: [
                                            Return(
                                                Some(
                                                    Identifier(
                                                        "n",
                                                    ),
                                                ),
                                            ),
                                        ],
                                        final_expr: None,
                                    },
                                    else_expr: Some(
                                        Block {
                                            statements: [
                                                Return(
                                                    Some(
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
                                                ),
                                            ],
                                            final_expr: None,
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
    RustValue(<unknown>),
)
```