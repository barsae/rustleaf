# Program
Status: 🟢
Assertions: 7

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
consume_token: position 0 consumed Fn
consume_token: position 1 consumed Ident
consume_token: position 2 consumed LeftParen
consume_token: position 3 consumed Ident
consume_token: position 4 consumed Equal
consume_token: position 5 consumed String
consume_token: position 6 consumed RightParen
consume_token: position 7 consumed LeftBrace
parse_statement: starting at position 8 (Ident(name))
consume_token: position 8 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 8 (Ident(name))
consume_token: position 8 consumed Ident
parse_primary: success - parsed identifier (name)
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace at position 9
parse_expression: starting at position 8 (Ident(name))
consume_token: position 8 consumed Ident
parse_primary: success - parsed identifier (name)
parse_expression: success - parsed precedence expression
consume_token: position 9 consumed RightBrace
parse_statement: success - parsed function declaration
parse_program: parsing statement at position 10 (Fn)
parse_statement: starting at position 10 (Fn)
consume_token: position 10 consumed Fn
consume_token: position 11 consumed Ident
consume_token: position 12 consumed LeftParen
consume_token: position 13 consumed Ident
consume_token: position 14 consumed Comma
consume_token: position 15 consumed Ident
consume_token: position 16 consumed Equal
consume_token: position 17 consumed Int
consume_token: position 18 consumed RightParen
consume_token: position 19 consumed LeftBrace
parse_statement: starting at position 20 (Ident(x))
consume_token: position 20 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 20 (Ident(x))
consume_token: position 20 consumed Ident
parse_primary: success - parsed identifier (x)
consume_token: position 21 consumed Plus
consume_token: position 22 consumed Ident
parse_primary: success - parsed identifier (y)
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace at position 23
parse_expression: starting at position 20 (Ident(x))
consume_token: position 20 consumed Ident
parse_primary: success - parsed identifier (x)
consume_token: position 21 consumed Plus
consume_token: position 22 consumed Ident
parse_primary: success - parsed identifier (y)
parse_expression: success - parsed precedence expression
consume_token: position 23 consumed RightBrace
parse_statement: success - parsed function declaration
parse_program: parsing statement at position 24 (Fn)
parse_statement: starting at position 24 (Fn)
consume_token: position 24 consumed Fn
consume_token: position 25 consumed Ident
consume_token: position 26 consumed LeftParen
consume_token: position 27 consumed Ident
consume_token: position 28 consumed Equal
consume_token: position 29 consumed Int
consume_token: position 30 consumed Comma
consume_token: position 31 consumed Ident
consume_token: position 32 consumed Equal
consume_token: position 33 consumed Int
consume_token: position 34 consumed RightParen
consume_token: position 35 consumed LeftBrace
parse_statement: starting at position 36 (Ident(a))
consume_token: position 36 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 36 (Ident(a))
consume_token: position 36 consumed Ident
parse_primary: success - parsed identifier (a)
consume_token: position 37 consumed Star
consume_token: position 38 consumed Ident
parse_primary: success - parsed identifier (b)
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace at position 39
parse_expression: starting at position 36 (Ident(a))
consume_token: position 36 consumed Ident
parse_primary: success - parsed identifier (a)
consume_token: position 37 consumed Star
consume_token: position 38 consumed Ident
parse_primary: success - parsed identifier (b)
parse_expression: success - parsed precedence expression
consume_token: position 39 consumed RightBrace
parse_statement: success - parsed function declaration
parse_program: parsing statement at position 40 (Var)
parse_statement: starting at position 40 (Var)
consume_token: position 40 consumed Var
consume_token: position 41 consumed Ident
consume_token: position 42 consumed Equal
parse_expression: starting at position 43 (Ident(greet))
consume_token: position 43 consumed Ident
parse_primary: success - parsed identifier (greet)
consume_token: position 44 consumed LeftParen
consume_token: position 45 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 46 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 47 (Var)
parse_statement: starting at position 47 (Var)
consume_token: position 47 consumed Var
consume_token: position 48 consumed Ident
consume_token: position 49 consumed Equal
parse_expression: starting at position 50 (Ident(greet))
consume_token: position 50 consumed Ident
parse_primary: success - parsed identifier (greet)
consume_token: position 51 consumed LeftParen
parse_expression: starting at position 52 (String(Alice))
consume_token: position 52 consumed String
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
parse_expression: starting at position 58 (Ident(add))
consume_token: position 58 consumed Ident
parse_primary: success - parsed identifier (add)
consume_token: position 59 consumed LeftParen
parse_expression: starting at position 60 (Int(5))
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
parse_expression: starting at position 66 (Ident(add))
consume_token: position 66 consumed Ident
parse_primary: success - parsed identifier (add)
consume_token: position 67 consumed LeftParen
parse_expression: starting at position 68 (Int(5))
consume_token: position 68 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 69 consumed Comma
parse_expression: starting at position 70 (Int(15))
consume_token: position 70 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 71 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 72 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 73 (Var)
parse_statement: starting at position 73 (Var)
consume_token: position 73 consumed Var
consume_token: position 74 consumed Ident
consume_token: position 75 consumed Equal
parse_expression: starting at position 76 (Ident(multiply))
consume_token: position 76 consumed Ident
parse_primary: success - parsed identifier (multiply)
consume_token: position 77 consumed LeftParen
consume_token: position 78 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 79 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 80 (Var)
parse_statement: starting at position 80 (Var)
consume_token: position 80 consumed Var
consume_token: position 81 consumed Ident
consume_token: position 82 consumed Equal
parse_expression: starting at position 83 (Ident(multiply))
consume_token: position 83 consumed Ident
parse_primary: success - parsed identifier (multiply)
consume_token: position 84 consumed LeftParen
parse_expression: starting at position 85 (Int(4))
consume_token: position 85 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 86 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 87 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 88 (Var)
parse_statement: starting at position 88 (Var)
consume_token: position 88 consumed Var
consume_token: position 89 consumed Ident
consume_token: position 90 consumed Equal
parse_expression: starting at position 91 (Ident(multiply))
consume_token: position 91 consumed Ident
parse_primary: success - parsed identifier (multiply)
consume_token: position 92 consumed LeftParen
parse_expression: starting at position 93 (Int(4))
consume_token: position 93 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 94 consumed Comma
parse_expression: starting at position 95 (Int(7))
consume_token: position 95 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 96 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 97 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 98 (Ident(assert))
parse_statement: starting at position 98 (Ident(assert))
consume_token: position 98 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 98 (Ident(assert))
consume_token: position 98 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 99 consumed LeftParen
parse_expression: starting at position 100 (Ident(greeting1))
consume_token: position 100 consumed Ident
parse_primary: success - parsed identifier (greeting1)
consume_token: position 101 consumed EqualEqual
consume_token: position 102 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 103 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 104 consumed Semicolon
parse_program: parsing statement at position 105 (Ident(assert))
parse_statement: starting at position 105 (Ident(assert))
consume_token: position 105 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 105 (Ident(assert))
consume_token: position 105 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 106 consumed LeftParen
parse_expression: starting at position 107 (Ident(greeting2))
consume_token: position 107 consumed Ident
parse_primary: success - parsed identifier (greeting2)
consume_token: position 108 consumed EqualEqual
consume_token: position 109 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 110 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 111 consumed Semicolon
parse_program: parsing statement at position 112 (Ident(assert))
parse_statement: starting at position 112 (Ident(assert))
consume_token: position 112 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 112 (Ident(assert))
consume_token: position 112 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 113 consumed LeftParen
parse_expression: starting at position 114 (Ident(sum1))
consume_token: position 114 consumed Ident
parse_primary: success - parsed identifier (sum1)
consume_token: position 115 consumed EqualEqual
consume_token: position 116 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 117 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 118 consumed Semicolon
parse_program: parsing statement at position 119 (Ident(assert))
parse_statement: starting at position 119 (Ident(assert))
consume_token: position 119 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 119 (Ident(assert))
consume_token: position 119 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 120 consumed LeftParen
parse_expression: starting at position 121 (Ident(sum2))
consume_token: position 121 consumed Ident
parse_primary: success - parsed identifier (sum2)
consume_token: position 122 consumed EqualEqual
consume_token: position 123 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 124 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 125 consumed Semicolon
parse_program: parsing statement at position 126 (Ident(assert))
parse_statement: starting at position 126 (Ident(assert))
consume_token: position 126 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 126 (Ident(assert))
consume_token: position 126 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 127 consumed LeftParen
parse_expression: starting at position 128 (Ident(product1))
consume_token: position 128 consumed Ident
parse_primary: success - parsed identifier (product1)
consume_token: position 129 consumed EqualEqual
consume_token: position 130 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 131 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 132 consumed Semicolon
parse_program: parsing statement at position 133 (Ident(assert))
parse_statement: starting at position 133 (Ident(assert))
consume_token: position 133 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 133 (Ident(assert))
consume_token: position 133 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 134 consumed LeftParen
parse_expression: starting at position 135 (Ident(product2))
consume_token: position 135 consumed Ident
parse_primary: success - parsed identifier (product2)
consume_token: position 136 consumed EqualEqual
consume_token: position 137 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 138 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 139 consumed Semicolon
parse_program: parsing statement at position 140 (Ident(assert))
parse_statement: starting at position 140 (Ident(assert))
consume_token: position 140 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 140 (Ident(assert))
consume_token: position 140 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 141 consumed LeftParen
parse_expression: starting at position 142 (Ident(product3))
consume_token: position 142 consumed Ident
parse_primary: success - parsed identifier (product3)
consume_token: position 143 consumed EqualEqual
consume_token: position 144 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 145 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 146 consumed Semicolon
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
        1: Token(Ident, "greet"),
        2: Token(LeftParen),
        3: Token(Ident, "name"),
        4: Token(Equal),
        5: Token(String, "world"),
        6: Token(RightParen),
        7: Token(LeftBrace),
        8: Token(Ident, "name"),
        9: Token(RightBrace),
        10: Token(Fn),
        11: Token(Ident, "add"),
        12: Token(LeftParen),
        13: Token(Ident, "x"),
        14: Token(Comma),
        15: Token(Ident, "y"),
        16: Token(Equal),
        17: Token(Int, "10"),
        18: Token(RightParen),
        19: Token(LeftBrace),
        20: Token(Ident, "x"),
        21: Token(Plus),
        22: Token(Ident, "y"),
        23: Token(RightBrace),
        24: Token(Fn),
        25: Token(Ident, "multiply"),
        26: Token(LeftParen),
        27: Token(Ident, "a"),
        28: Token(Equal),
        29: Token(Int, "2"),
        30: Token(Comma),
        31: Token(Ident, "b"),
        32: Token(Equal),
        33: Token(Int, "3"),
        34: Token(RightParen),
        35: Token(LeftBrace),
        36: Token(Ident, "a"),
        37: Token(Star),
        38: Token(Ident, "b"),
        39: Token(RightBrace),
        40: Token(Var),
        41: Token(Ident, "greeting1"),
        42: Token(Equal),
        43: Token(Ident, "greet"),
        44: Token(LeftParen),
        45: Token(RightParen),
        46: Token(Semicolon),
        47: Token(Var),
        48: Token(Ident, "greeting2"),
        49: Token(Equal),
        50: Token(Ident, "greet"),
        51: Token(LeftParen),
        52: Token(String, "Alice"),
        53: Token(RightParen),
        54: Token(Semicolon),
        55: Token(Var),
        56: Token(Ident, "sum1"),
        57: Token(Equal),
        58: Token(Ident, "add"),
        59: Token(LeftParen),
        60: Token(Int, "5"),
        61: Token(RightParen),
        62: Token(Semicolon),
        63: Token(Var),
        64: Token(Ident, "sum2"),
        65: Token(Equal),
        66: Token(Ident, "add"),
        67: Token(LeftParen),
        68: Token(Int, "5"),
        69: Token(Comma),
        70: Token(Int, "15"),
        71: Token(RightParen),
        72: Token(Semicolon),
        73: Token(Var),
        74: Token(Ident, "product1"),
        75: Token(Equal),
        76: Token(Ident, "multiply"),
        77: Token(LeftParen),
        78: Token(RightParen),
        79: Token(Semicolon),
        80: Token(Var),
        81: Token(Ident, "product2"),
        82: Token(Equal),
        83: Token(Ident, "multiply"),
        84: Token(LeftParen),
        85: Token(Int, "4"),
        86: Token(RightParen),
        87: Token(Semicolon),
        88: Token(Var),
        89: Token(Ident, "product3"),
        90: Token(Equal),
        91: Token(Ident, "multiply"),
        92: Token(LeftParen),
        93: Token(Int, "4"),
        94: Token(Comma),
        95: Token(Int, "7"),
        96: Token(RightParen),
        97: Token(Semicolon),
        98: Token(Ident, "assert"),
        99: Token(LeftParen),
        100: Token(Ident, "greeting1"),
        101: Token(EqualEqual),
        102: Token(String, "world"),
        103: Token(RightParen),
        104: Token(Semicolon),
        105: Token(Ident, "assert"),
        106: Token(LeftParen),
        107: Token(Ident, "greeting2"),
        108: Token(EqualEqual),
        109: Token(String, "Alice"),
        110: Token(RightParen),
        111: Token(Semicolon),
        112: Token(Ident, "assert"),
        113: Token(LeftParen),
        114: Token(Ident, "sum1"),
        115: Token(EqualEqual),
        116: Token(Int, "15"),
        117: Token(RightParen),
        118: Token(Semicolon),
        119: Token(Ident, "assert"),
        120: Token(LeftParen),
        121: Token(Ident, "sum2"),
        122: Token(EqualEqual),
        123: Token(Int, "20"),
        124: Token(RightParen),
        125: Token(Semicolon),
        126: Token(Ident, "assert"),
        127: Token(LeftParen),
        128: Token(Ident, "product1"),
        129: Token(EqualEqual),
        130: Token(Int, "6"),
        131: Token(RightParen),
        132: Token(Semicolon),
        133: Token(Ident, "assert"),
        134: Token(LeftParen),
        135: Token(Ident, "product2"),
        136: Token(EqualEqual),
        137: Token(Int, "12"),
        138: Token(RightParen),
        139: Token(Semicolon),
        140: Token(Ident, "assert"),
        141: Token(LeftParen),
        142: Token(Ident, "product3"),
        143: Token(EqualEqual),
        144: Token(Int, "28"),
        145: Token(RightParen),
        146: Token(Semicolon),
        147: Token(Eof)
    ],
)
```

# Parse
```rust
Ok(
    Program(
        [
            FnDecl {
                name: "greet",
                params: [
                    Parameter {
                        name: "name",
                        default: Some(
                            String(
                                "world",
                            ),
                        ),
                        kind: Regular,
                    },
                ],
                body: Block {
                    statements: [],
                    final_expr: Some(
                        Identifier(
                            "name",
                        ),
                    ),
                },
                is_pub: false,
            },
            FnDecl {
                name: "add",
                params: [
                    Parameter {
                        name: "x",
                        default: None,
                        kind: Regular,
                    },
                    Parameter {
                        name: "y",
                        default: Some(
                            Int(
                                10,
                            ),
                        ),
                        kind: Regular,
                    },
                ],
                body: Block {
                    statements: [],
                    final_expr: Some(
                        Add(
                            Identifier(
                                "x",
                            ),
                            Identifier(
                                "y",
                            ),
                        ),
                    ),
                },
                is_pub: false,
            },
            FnDecl {
                name: "multiply",
                params: [
                    Parameter {
                        name: "a",
                        default: Some(
                            Int(
                                2,
                            ),
                        ),
                        kind: Regular,
                    },
                    Parameter {
                        name: "b",
                        default: Some(
                            Int(
                                3,
                            ),
                        ),
                        kind: Regular,
                    },
                ],
                body: Block {
                    statements: [],
                    final_expr: Some(
                        Mul(
                            Identifier(
                                "a",
                            ),
                            Identifier(
                                "b",
                            ),
                        ),
                    ),
                },
                is_pub: false,
            },
            VarDecl {
                pattern: Variable(
                    "greeting1",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "greet",
                        ),
                        [],
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "greeting2",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "greet",
                        ),
                        [
                            Literal(
                                String(
                                    "Alice",
                                ),
                            ),
                        ],
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "sum1",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "add",
                        ),
                        [
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
                    "sum2",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "add",
                        ),
                        [
                            Literal(
                                Int(
                                    5,
                                ),
                            ),
                            Literal(
                                Int(
                                    15,
                                ),
                            ),
                        ],
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "product1",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "multiply",
                        ),
                        [],
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "product2",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "multiply",
                        ),
                        [
                            Literal(
                                Int(
                                    4,
                                ),
                            ),
                        ],
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "product3",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "multiply",
                        ),
                        [
                            Literal(
                                Int(
                                    4,
                                ),
                            ),
                            Literal(
                                Int(
                                    7,
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
                                "greeting1",
                            ),
                            Literal(
                                String(
                                    "world",
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
                            Identifier(
                                "greeting2",
                            ),
                            Literal(
                                String(
                                    "Alice",
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
                            Identifier(
                                "sum1",
                            ),
                            Literal(
                                Int(
                                    15,
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
                            Identifier(
                                "sum2",
                            ),
                            Literal(
                                Int(
                                    20,
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
                            Identifier(
                                "product1",
                            ),
                            Literal(
                                Int(
                                    6,
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
                            Identifier(
                                "product2",
                            ),
                            Literal(
                                Int(
                                    12,
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
                            Identifier(
                                "product3",
                            ),
                            Literal(
                                Int(
                                    28,
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