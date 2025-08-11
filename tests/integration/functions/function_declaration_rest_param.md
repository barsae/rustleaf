# Program
Status: 🟢
Assertions: 10

```rustleaf
fn sum(*args) { args }
fn first_and_rest(first, *rest) { [first, rest] }

var empty = sum();
var single = sum(1);
var multiple = sum(1, 2, 3, 4, 5);

var mixed1 = first_and_rest(42);
var mixed2 = first_and_rest(10, 20, 30);

// Test that rest parameters collect into lists
assert(empty == []);
assert(single == [1]);
assert(multiple == [1, 2, 3, 4, 5]);

// Test mixed regular and rest parameters
assert(mixed1 == [42, []]);
assert(mixed2 == [10, [20, 30]]);

// Test individual access
assert(single[0] == 1);
assert(multiple[0] == 1);
assert(multiple[4] == 5);
assert(mixed1[0] == 42);
assert(mixed2[0] == 10);
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Fn)
parse_statement: starting at position 0 (Fn)
consume_token: position 0 consumed Fn
consume_token: position 1 consumed Ident
consume_token: position 2 consumed LeftParen
consume_token: position 3 consumed Star
consume_token: position 4 consumed Ident
consume_token: position 5 consumed RightParen
consume_token: position 6 consumed LeftBrace
parse_statement: starting at position 7 (Ident(args))
consume_token: position 7 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 7 (Ident(args))
consume_token: position 7 consumed Ident
parse_primary: success - parsed identifier (args)
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace at position 8
parse_expression: starting at position 7 (Ident(args))
consume_token: position 7 consumed Ident
parse_primary: success - parsed identifier (args)
parse_expression: success - parsed precedence expression
consume_token: position 8 consumed RightBrace
parse_statement: success - parsed function declaration
parse_program: parsing statement at position 9 (Fn)
parse_statement: starting at position 9 (Fn)
consume_token: position 9 consumed Fn
consume_token: position 10 consumed Ident
consume_token: position 11 consumed LeftParen
consume_token: position 12 consumed Ident
consume_token: position 13 consumed Comma
consume_token: position 14 consumed Star
consume_token: position 15 consumed Ident
consume_token: position 16 consumed RightParen
consume_token: position 17 consumed LeftBrace
parse_statement: starting at position 18 (LeftBracket)
parse_statement: falling back to expression statement
parse_expression: starting at position 18 (LeftBracket)
consume_token: position 18 consumed LeftBracket
parse_primary: success - parsing list literal
parse_list_literal: starting at position 19
parse_list_literal: parsing element at position 19
parse_expression: starting at position 19 (Ident(first))
consume_token: position 19 consumed Ident
parse_primary: success - parsed identifier (first)
parse_expression: success - parsed precedence expression
consume_token: position 20 consumed Comma
parse_list_literal: found comma, checking for more elements
parse_list_literal: parsing element at position 21
parse_expression: starting at position 21 (Ident(rest))
consume_token: position 21 consumed Ident
parse_primary: success - parsed identifier (rest)
parse_expression: success - parsed precedence expression
parse_list_literal: no comma, expecting ]
parse_list_literal: expecting ] at position 22
consume_token: position 22 consumed RightBracket
parse_list_literal: success
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace at position 23
parse_expression: starting at position 18 (LeftBracket)
consume_token: position 18 consumed LeftBracket
parse_primary: success - parsing list literal
parse_list_literal: starting at position 19
parse_list_literal: parsing element at position 19
parse_expression: starting at position 19 (Ident(first))
consume_token: position 19 consumed Ident
parse_primary: success - parsed identifier (first)
parse_expression: success - parsed precedence expression
consume_token: position 20 consumed Comma
parse_list_literal: found comma, checking for more elements
parse_list_literal: parsing element at position 21
parse_expression: starting at position 21 (Ident(rest))
consume_token: position 21 consumed Ident
parse_primary: success - parsed identifier (rest)
parse_expression: success - parsed precedence expression
parse_list_literal: no comma, expecting ]
parse_list_literal: expecting ] at position 22
consume_token: position 22 consumed RightBracket
parse_list_literal: success
parse_expression: success - parsed precedence expression
consume_token: position 23 consumed RightBrace
parse_statement: success - parsed function declaration
parse_program: parsing statement at position 24 (Var)
parse_statement: starting at position 24 (Var)
consume_token: position 24 consumed Var
consume_token: position 25 consumed Ident
consume_token: position 26 consumed Equal
parse_expression: starting at position 27 (Ident(sum))
consume_token: position 27 consumed Ident
parse_primary: success - parsed identifier (sum)
consume_token: position 28 consumed LeftParen
consume_token: position 29 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 30 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 31 (Var)
parse_statement: starting at position 31 (Var)
consume_token: position 31 consumed Var
consume_token: position 32 consumed Ident
consume_token: position 33 consumed Equal
parse_expression: starting at position 34 (Ident(sum))
consume_token: position 34 consumed Ident
parse_primary: success - parsed identifier (sum)
consume_token: position 35 consumed LeftParen
parse_expression: starting at position 36 (Int(1))
consume_token: position 36 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 37 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 38 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 39 (Var)
parse_statement: starting at position 39 (Var)
consume_token: position 39 consumed Var
consume_token: position 40 consumed Ident
consume_token: position 41 consumed Equal
parse_expression: starting at position 42 (Ident(sum))
consume_token: position 42 consumed Ident
parse_primary: success - parsed identifier (sum)
consume_token: position 43 consumed LeftParen
parse_expression: starting at position 44 (Int(1))
consume_token: position 44 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 45 consumed Comma
parse_expression: starting at position 46 (Int(2))
consume_token: position 46 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 47 consumed Comma
parse_expression: starting at position 48 (Int(3))
consume_token: position 48 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 49 consumed Comma
parse_expression: starting at position 50 (Int(4))
consume_token: position 50 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 51 consumed Comma
parse_expression: starting at position 52 (Int(5))
consume_token: position 52 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 53 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 54 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 55 (Var)
parse_statement: starting at position 55 (Var)
consume_token: position 55 consumed Var
consume_token: position 56 consumed Ident
consume_token: position 57 consumed Equal
parse_expression: starting at position 58 (Ident(first_and_rest))
consume_token: position 58 consumed Ident
parse_primary: success - parsed identifier (first_and_rest)
consume_token: position 59 consumed LeftParen
parse_expression: starting at position 60 (Int(42))
consume_token: position 60 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 61 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 62 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 63 (Var)
parse_statement: starting at position 63 (Var)
consume_token: position 63 consumed Var
consume_token: position 64 consumed Ident
consume_token: position 65 consumed Equal
parse_expression: starting at position 66 (Ident(first_and_rest))
consume_token: position 66 consumed Ident
parse_primary: success - parsed identifier (first_and_rest)
consume_token: position 67 consumed LeftParen
parse_expression: starting at position 68 (Int(10))
consume_token: position 68 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 69 consumed Comma
parse_expression: starting at position 70 (Int(20))
consume_token: position 70 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 71 consumed Comma
parse_expression: starting at position 72 (Int(30))
consume_token: position 72 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 73 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 74 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 75 (Ident(assert))
parse_statement: starting at position 75 (Ident(assert))
consume_token: position 75 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 75 (Ident(assert))
consume_token: position 75 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 76 consumed LeftParen
parse_expression: starting at position 77 (Ident(empty))
consume_token: position 77 consumed Ident
parse_primary: success - parsed identifier (empty)
consume_token: position 78 consumed EqualEqual
consume_token: position 79 consumed LeftBracket
parse_primary: success - parsing list literal
parse_list_literal: starting at position 80
consume_token: position 80 consumed RightBracket
parse_list_literal: empty list
parse_expression: success - parsed precedence expression
consume_token: position 81 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 82 consumed Semicolon
parse_program: parsing statement at position 83 (Ident(assert))
parse_statement: starting at position 83 (Ident(assert))
consume_token: position 83 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 83 (Ident(assert))
consume_token: position 83 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 84 consumed LeftParen
parse_expression: starting at position 85 (Ident(single))
consume_token: position 85 consumed Ident
parse_primary: success - parsed identifier (single)
consume_token: position 86 consumed EqualEqual
consume_token: position 87 consumed LeftBracket
parse_primary: success - parsing list literal
parse_list_literal: starting at position 88
parse_list_literal: parsing element at position 88
parse_expression: starting at position 88 (Int(1))
consume_token: position 88 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_list_literal: no comma, expecting ]
parse_list_literal: expecting ] at position 89
consume_token: position 89 consumed RightBracket
parse_list_literal: success
parse_expression: success - parsed precedence expression
consume_token: position 90 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 91 consumed Semicolon
parse_program: parsing statement at position 92 (Ident(assert))
parse_statement: starting at position 92 (Ident(assert))
consume_token: position 92 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 92 (Ident(assert))
consume_token: position 92 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 93 consumed LeftParen
parse_expression: starting at position 94 (Ident(multiple))
consume_token: position 94 consumed Ident
parse_primary: success - parsed identifier (multiple)
consume_token: position 95 consumed EqualEqual
consume_token: position 96 consumed LeftBracket
parse_primary: success - parsing list literal
parse_list_literal: starting at position 97
parse_list_literal: parsing element at position 97
parse_expression: starting at position 97 (Int(1))
consume_token: position 97 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 98 consumed Comma
parse_list_literal: found comma, checking for more elements
parse_list_literal: parsing element at position 99
parse_expression: starting at position 99 (Int(2))
consume_token: position 99 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 100 consumed Comma
parse_list_literal: found comma, checking for more elements
parse_list_literal: parsing element at position 101
parse_expression: starting at position 101 (Int(3))
consume_token: position 101 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 102 consumed Comma
parse_list_literal: found comma, checking for more elements
parse_list_literal: parsing element at position 103
parse_expression: starting at position 103 (Int(4))
consume_token: position 103 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 104 consumed Comma
parse_list_literal: found comma, checking for more elements
parse_list_literal: parsing element at position 105
parse_expression: starting at position 105 (Int(5))
consume_token: position 105 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_list_literal: no comma, expecting ]
parse_list_literal: expecting ] at position 106
consume_token: position 106 consumed RightBracket
parse_list_literal: success
parse_expression: success - parsed precedence expression
consume_token: position 107 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 108 consumed Semicolon
parse_program: parsing statement at position 109 (Ident(assert))
parse_statement: starting at position 109 (Ident(assert))
consume_token: position 109 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 109 (Ident(assert))
consume_token: position 109 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 110 consumed LeftParen
parse_expression: starting at position 111 (Ident(mixed1))
consume_token: position 111 consumed Ident
parse_primary: success - parsed identifier (mixed1)
consume_token: position 112 consumed EqualEqual
consume_token: position 113 consumed LeftBracket
parse_primary: success - parsing list literal
parse_list_literal: starting at position 114
parse_list_literal: parsing element at position 114
parse_expression: starting at position 114 (Int(42))
consume_token: position 114 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 115 consumed Comma
parse_list_literal: found comma, checking for more elements
parse_list_literal: parsing element at position 116
parse_expression: starting at position 116 (LeftBracket)
consume_token: position 116 consumed LeftBracket
parse_primary: success - parsing list literal
parse_list_literal: starting at position 117
consume_token: position 117 consumed RightBracket
parse_list_literal: empty list
parse_expression: success - parsed precedence expression
parse_list_literal: no comma, expecting ]
parse_list_literal: expecting ] at position 118
consume_token: position 118 consumed RightBracket
parse_list_literal: success
parse_expression: success - parsed precedence expression
consume_token: position 119 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 120 consumed Semicolon
parse_program: parsing statement at position 121 (Ident(assert))
parse_statement: starting at position 121 (Ident(assert))
consume_token: position 121 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 121 (Ident(assert))
consume_token: position 121 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 122 consumed LeftParen
parse_expression: starting at position 123 (Ident(mixed2))
consume_token: position 123 consumed Ident
parse_primary: success - parsed identifier (mixed2)
consume_token: position 124 consumed EqualEqual
consume_token: position 125 consumed LeftBracket
parse_primary: success - parsing list literal
parse_list_literal: starting at position 126
parse_list_literal: parsing element at position 126
parse_expression: starting at position 126 (Int(10))
consume_token: position 126 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 127 consumed Comma
parse_list_literal: found comma, checking for more elements
parse_list_literal: parsing element at position 128
parse_expression: starting at position 128 (LeftBracket)
consume_token: position 128 consumed LeftBracket
parse_primary: success - parsing list literal
parse_list_literal: starting at position 129
parse_list_literal: parsing element at position 129
parse_expression: starting at position 129 (Int(20))
consume_token: position 129 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 130 consumed Comma
parse_list_literal: found comma, checking for more elements
parse_list_literal: parsing element at position 131
parse_expression: starting at position 131 (Int(30))
consume_token: position 131 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_list_literal: no comma, expecting ]
parse_list_literal: expecting ] at position 132
consume_token: position 132 consumed RightBracket
parse_list_literal: success
parse_expression: success - parsed precedence expression
parse_list_literal: no comma, expecting ]
parse_list_literal: expecting ] at position 133
consume_token: position 133 consumed RightBracket
parse_list_literal: success
parse_expression: success - parsed precedence expression
consume_token: position 134 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 135 consumed Semicolon
parse_program: parsing statement at position 136 (Ident(assert))
parse_statement: starting at position 136 (Ident(assert))
consume_token: position 136 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 136 (Ident(assert))
consume_token: position 136 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 137 consumed LeftParen
parse_expression: starting at position 138 (Ident(single))
consume_token: position 138 consumed Ident
parse_primary: success - parsed identifier (single)
consume_token: position 139 consumed LeftBracket
parse_expression: starting at position 140 (Int(0))
consume_token: position 140 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 141 consumed RightBracket
consume_token: position 142 consumed EqualEqual
consume_token: position 143 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 144 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 145 consumed Semicolon
parse_program: parsing statement at position 146 (Ident(assert))
parse_statement: starting at position 146 (Ident(assert))
consume_token: position 146 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 146 (Ident(assert))
consume_token: position 146 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 147 consumed LeftParen
parse_expression: starting at position 148 (Ident(multiple))
consume_token: position 148 consumed Ident
parse_primary: success - parsed identifier (multiple)
consume_token: position 149 consumed LeftBracket
parse_expression: starting at position 150 (Int(0))
consume_token: position 150 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 151 consumed RightBracket
consume_token: position 152 consumed EqualEqual
consume_token: position 153 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 154 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 155 consumed Semicolon
parse_program: parsing statement at position 156 (Ident(assert))
parse_statement: starting at position 156 (Ident(assert))
consume_token: position 156 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 156 (Ident(assert))
consume_token: position 156 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 157 consumed LeftParen
parse_expression: starting at position 158 (Ident(multiple))
consume_token: position 158 consumed Ident
parse_primary: success - parsed identifier (multiple)
consume_token: position 159 consumed LeftBracket
parse_expression: starting at position 160 (Int(4))
consume_token: position 160 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 161 consumed RightBracket
consume_token: position 162 consumed EqualEqual
consume_token: position 163 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 164 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 165 consumed Semicolon
parse_program: parsing statement at position 166 (Ident(assert))
parse_statement: starting at position 166 (Ident(assert))
consume_token: position 166 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 166 (Ident(assert))
consume_token: position 166 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 167 consumed LeftParen
parse_expression: starting at position 168 (Ident(mixed1))
consume_token: position 168 consumed Ident
parse_primary: success - parsed identifier (mixed1)
consume_token: position 169 consumed LeftBracket
parse_expression: starting at position 170 (Int(0))
consume_token: position 170 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 171 consumed RightBracket
consume_token: position 172 consumed EqualEqual
consume_token: position 173 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 174 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 175 consumed Semicolon
parse_program: parsing statement at position 176 (Ident(assert))
parse_statement: starting at position 176 (Ident(assert))
consume_token: position 176 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 176 (Ident(assert))
consume_token: position 176 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 177 consumed LeftParen
parse_expression: starting at position 178 (Ident(mixed2))
consume_token: position 178 consumed Ident
parse_primary: success - parsed identifier (mixed2)
consume_token: position 179 consumed LeftBracket
parse_expression: starting at position 180 (Int(0))
consume_token: position 180 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 181 consumed RightBracket
consume_token: position 182 consumed EqualEqual
consume_token: position 183 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 184 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 185 consumed Semicolon
parse_program: parsed 17 statements
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
        1: Token(Ident, "sum"),
        2: Token(LeftParen),
        3: Token(Star),
        4: Token(Ident, "args"),
        5: Token(RightParen),
        6: Token(LeftBrace),
        7: Token(Ident, "args"),
        8: Token(RightBrace),
        9: Token(Fn),
        10: Token(Ident, "first_and_rest"),
        11: Token(LeftParen),
        12: Token(Ident, "first"),
        13: Token(Comma),
        14: Token(Star),
        15: Token(Ident, "rest"),
        16: Token(RightParen),
        17: Token(LeftBrace),
        18: Token(LeftBracket),
        19: Token(Ident, "first"),
        20: Token(Comma),
        21: Token(Ident, "rest"),
        22: Token(RightBracket),
        23: Token(RightBrace),
        24: Token(Var),
        25: Token(Ident, "empty"),
        26: Token(Equal),
        27: Token(Ident, "sum"),
        28: Token(LeftParen),
        29: Token(RightParen),
        30: Token(Semicolon),
        31: Token(Var),
        32: Token(Ident, "single"),
        33: Token(Equal),
        34: Token(Ident, "sum"),
        35: Token(LeftParen),
        36: Token(Int, "1"),
        37: Token(RightParen),
        38: Token(Semicolon),
        39: Token(Var),
        40: Token(Ident, "multiple"),
        41: Token(Equal),
        42: Token(Ident, "sum"),
        43: Token(LeftParen),
        44: Token(Int, "1"),
        45: Token(Comma),
        46: Token(Int, "2"),
        47: Token(Comma),
        48: Token(Int, "3"),
        49: Token(Comma),
        50: Token(Int, "4"),
        51: Token(Comma),
        52: Token(Int, "5"),
        53: Token(RightParen),
        54: Token(Semicolon),
        55: Token(Var),
        56: Token(Ident, "mixed1"),
        57: Token(Equal),
        58: Token(Ident, "first_and_rest"),
        59: Token(LeftParen),
        60: Token(Int, "42"),
        61: Token(RightParen),
        62: Token(Semicolon),
        63: Token(Var),
        64: Token(Ident, "mixed2"),
        65: Token(Equal),
        66: Token(Ident, "first_and_rest"),
        67: Token(LeftParen),
        68: Token(Int, "10"),
        69: Token(Comma),
        70: Token(Int, "20"),
        71: Token(Comma),
        72: Token(Int, "30"),
        73: Token(RightParen),
        74: Token(Semicolon),
        75: Token(Ident, "assert"),
        76: Token(LeftParen),
        77: Token(Ident, "empty"),
        78: Token(EqualEqual),
        79: Token(LeftBracket),
        80: Token(RightBracket),
        81: Token(RightParen),
        82: Token(Semicolon),
        83: Token(Ident, "assert"),
        84: Token(LeftParen),
        85: Token(Ident, "single"),
        86: Token(EqualEqual),
        87: Token(LeftBracket),
        88: Token(Int, "1"),
        89: Token(RightBracket),
        90: Token(RightParen),
        91: Token(Semicolon),
        92: Token(Ident, "assert"),
        93: Token(LeftParen),
        94: Token(Ident, "multiple"),
        95: Token(EqualEqual),
        96: Token(LeftBracket),
        97: Token(Int, "1"),
        98: Token(Comma),
        99: Token(Int, "2"),
        100: Token(Comma),
        101: Token(Int, "3"),
        102: Token(Comma),
        103: Token(Int, "4"),
        104: Token(Comma),
        105: Token(Int, "5"),
        106: Token(RightBracket),
        107: Token(RightParen),
        108: Token(Semicolon),
        109: Token(Ident, "assert"),
        110: Token(LeftParen),
        111: Token(Ident, "mixed1"),
        112: Token(EqualEqual),
        113: Token(LeftBracket),
        114: Token(Int, "42"),
        115: Token(Comma),
        116: Token(LeftBracket),
        117: Token(RightBracket),
        118: Token(RightBracket),
        119: Token(RightParen),
        120: Token(Semicolon),
        121: Token(Ident, "assert"),
        122: Token(LeftParen),
        123: Token(Ident, "mixed2"),
        124: Token(EqualEqual),
        125: Token(LeftBracket),
        126: Token(Int, "10"),
        127: Token(Comma),
        128: Token(LeftBracket),
        129: Token(Int, "20"),
        130: Token(Comma),
        131: Token(Int, "30"),
        132: Token(RightBracket),
        133: Token(RightBracket),
        134: Token(RightParen),
        135: Token(Semicolon),
        136: Token(Ident, "assert"),
        137: Token(LeftParen),
        138: Token(Ident, "single"),
        139: Token(LeftBracket),
        140: Token(Int, "0"),
        141: Token(RightBracket),
        142: Token(EqualEqual),
        143: Token(Int, "1"),
        144: Token(RightParen),
        145: Token(Semicolon),
        146: Token(Ident, "assert"),
        147: Token(LeftParen),
        148: Token(Ident, "multiple"),
        149: Token(LeftBracket),
        150: Token(Int, "0"),
        151: Token(RightBracket),
        152: Token(EqualEqual),
        153: Token(Int, "1"),
        154: Token(RightParen),
        155: Token(Semicolon),
        156: Token(Ident, "assert"),
        157: Token(LeftParen),
        158: Token(Ident, "multiple"),
        159: Token(LeftBracket),
        160: Token(Int, "4"),
        161: Token(RightBracket),
        162: Token(EqualEqual),
        163: Token(Int, "5"),
        164: Token(RightParen),
        165: Token(Semicolon),
        166: Token(Ident, "assert"),
        167: Token(LeftParen),
        168: Token(Ident, "mixed1"),
        169: Token(LeftBracket),
        170: Token(Int, "0"),
        171: Token(RightBracket),
        172: Token(EqualEqual),
        173: Token(Int, "42"),
        174: Token(RightParen),
        175: Token(Semicolon),
        176: Token(Ident, "assert"),
        177: Token(LeftParen),
        178: Token(Ident, "mixed2"),
        179: Token(LeftBracket),
        180: Token(Int, "0"),
        181: Token(RightBracket),
        182: Token(EqualEqual),
        183: Token(Int, "10"),
        184: Token(RightParen),
        185: Token(Semicolon),
        186: Token(Eof)
    ],
)
```

# Parse
```rust
Ok(
    Program(
        [
            FnDecl {
                name: "sum",
                params: [
                    Parameter {
                        name: "args",
                        default: None,
                        kind: Rest,
                    },
                ],
                body: Block {
                    statements: [],
                    final_expr: Some(
                        Identifier(
                            "args",
                        ),
                    ),
                },
                is_pub: false,
            },
            FnDecl {
                name: "first_and_rest",
                params: [
                    Parameter {
                        name: "first",
                        default: None,
                        kind: Regular,
                    },
                    Parameter {
                        name: "rest",
                        default: None,
                        kind: Rest,
                    },
                ],
                body: Block {
                    statements: [],
                    final_expr: Some(
                        List(
                            [
                                Identifier(
                                    "first",
                                ),
                                Identifier(
                                    "rest",
                                ),
                            ],
                        ),
                    ),
                },
                is_pub: false,
            },
            VarDecl {
                pattern: Variable(
                    "empty",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "sum",
                        ),
                        [],
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "single",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "sum",
                        ),
                        [
                            Literal(
                                Int(
                                    1,
                                ),
                            ),
                        ],
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "multiple",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "sum",
                        ),
                        [
                            Literal(
                                Int(
                                    1,
                                ),
                            ),
                            Literal(
                                Int(
                                    2,
                                ),
                            ),
                            Literal(
                                Int(
                                    3,
                                ),
                            ),
                            Literal(
                                Int(
                                    4,
                                ),
                            ),
                            Literal(
                                Int(
                                    5,
                                ),
                            ),
                        ],
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "mixed1",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "first_and_rest",
                        ),
                        [
                            Literal(
                                Int(
                                    42,
                                ),
                            ),
                        ],
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "mixed2",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "first_and_rest",
                        ),
                        [
                            Literal(
                                Int(
                                    10,
                                ),
                            ),
                            Literal(
                                Int(
                                    20,
                                ),
                            ),
                            Literal(
                                Int(
                                    30,
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
                                "empty",
                            ),
                            List(
                                [],
                            ),
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
                                "single",
                            ),
                            List(
                                [
                                    Literal(
                                        Int(
                                            1,
                                        ),
                                    ),
                                ],
                            ),
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
                                "multiple",
                            ),
                            List(
                                [
                                    Literal(
                                        Int(
                                            1,
                                        ),
                                    ),
                                    Literal(
                                        Int(
                                            2,
                                        ),
                                    ),
                                    Literal(
                                        Int(
                                            3,
                                        ),
                                    ),
                                    Literal(
                                        Int(
                                            4,
                                        ),
                                    ),
                                    Literal(
                                        Int(
                                            5,
                                        ),
                                    ),
                                ],
                            ),
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
                                "mixed1",
                            ),
                            List(
                                [
                                    Literal(
                                        Int(
                                            42,
                                        ),
                                    ),
                                    List(
                                        [],
                                    ),
                                ],
                            ),
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
                                "mixed2",
                            ),
                            List(
                                [
                                    Literal(
                                        Int(
                                            10,
                                        ),
                                    ),
                                    List(
                                        [
                                            Literal(
                                                Int(
                                                    20,
                                                ),
                                            ),
                                            Literal(
                                                Int(
                                                    30,
                                                ),
                                            ),
                                        ],
                                    ),
                                ],
                            ),
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
                            GetItem(
                                Identifier(
                                    "single",
                                ),
                                Literal(
                                    Int(
                                        0,
                                    ),
                                ),
                            ),
                            Literal(
                                Int(
                                    1,
                                ),
                            ),
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
                            GetItem(
                                Identifier(
                                    "multiple",
                                ),
                                Literal(
                                    Int(
                                        0,
                                    ),
                                ),
                            ),
                            Literal(
                                Int(
                                    1,
                                ),
                            ),
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
                            GetItem(
                                Identifier(
                                    "multiple",
                                ),
                                Literal(
                                    Int(
                                        4,
                                    ),
                                ),
                            ),
                            Literal(
                                Int(
                                    5,
                                ),
                            ),
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
                            GetItem(
                                Identifier(
                                    "mixed1",
                                ),
                                Literal(
                                    Int(
                                        0,
                                    ),
                                ),
                            ),
                            Literal(
                                Int(
                                    42,
                                ),
                            ),
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
                            GetItem(
                                Identifier(
                                    "mixed2",
                                ),
                                Literal(
                                    Int(
                                        0,
                                    ),
                                ),
                            ),
                            Literal(
                                Int(
                                    10,
                                ),
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