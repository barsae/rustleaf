# Program
Status: 🟢
Assertions: 22

```rustleaf
// Comprehensive test of is and is not operators

// Basic type checking
var int_val = 42;
var float_val = 3.14;
var string_val = "hello";
var bool_val = true;
var list_val = [1, 2, 3];
var dict_val = {"key": "value"};
var range_val = 1..10;
var lambda_val = |x| x + 1;

// Test 'is' operator with various types
assert(int_val is Int, "int_val should be Int");
assert(float_val is Float, "float_val should be Float");
assert(string_val is String, "string_val should be String");
assert(bool_val is Bool, "bool_val should be Bool");
assert(list_val is List, "list_val should be List");
assert(dict_val is Dict, "dict_val should be Dict");
assert(range_val is Range, "range_val should be Range");
assert(lambda_val is Function, "lambda_val should be Function");

// Test 'is not' operator
assert(int_val is not String, "int_val should not be String");
assert(float_val is not Int, "float_val should not be Int");
assert(string_val is not Bool, "string_val should not be Bool");
assert(bool_val is not List, "bool_val should not be List");
assert(list_val is not Dict, "list_val should not be Dict");
assert(dict_val is not Range, "dict_val should not be Range");
assert(range_val is not Function, "range_val should not be Function");
assert(lambda_val is not Int, "lambda_val should not be Int");

// Test with special values
fn f() {}
var unit = f();
assert(null is Null, "null should be Null");
assert(unit is Unit, "unit should be Unit");
assert(null is not Unit, "null should not be Unit");
assert(unit is not Null, "unit should not be Null");

// Test identity comparison fallback for non-type values
var list1 = [1, 2, 3];
var list2 = [1, 2, 3];
var list3 = list1;
assert(list1 is not list2, "list1 should not be identical to list2 (different objects)");
assert(list1 is list3, "list1 should be identical to list3 (same object reference)");
```

# Output
None

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
        0: Token(Var),
        1: Token(Ident, "int_val"),
        2: Token(Equal),
        3: Token(Int, "42"),
        4: Token(Semicolon),
        5: Token(Var),
        6: Token(Ident, "float_val"),
        7: Token(Equal),
        8: Token(Float, "3.14"),
        9: Token(Semicolon),
        10: Token(Var),
        11: Token(Ident, "string_val"),
        12: Token(Equal),
        13: Token(String, "hello"),
        14: Token(Semicolon),
        15: Token(Var),
        16: Token(Ident, "bool_val"),
        17: Token(Equal),
        18: Token(True),
        19: Token(Semicolon),
        20: Token(Var),
        21: Token(Ident, "list_val"),
        22: Token(Equal),
        23: Token(LeftBracket),
        24: Token(Int, "1"),
        25: Token(Comma),
        26: Token(Int, "2"),
        27: Token(Comma),
        28: Token(Int, "3"),
        29: Token(RightBracket),
        30: Token(Semicolon),
        31: Token(Var),
        32: Token(Ident, "dict_val"),
        33: Token(Equal),
        34: Token(LeftBrace),
        35: Token(String, "key"),
        36: Token(Colon),
        37: Token(String, "value"),
        38: Token(RightBrace),
        39: Token(Semicolon),
        40: Token(Var),
        41: Token(Ident, "range_val"),
        42: Token(Equal),
        43: Token(Int, "1"),
        44: Token(DotDot),
        45: Token(Int, "10"),
        46: Token(Semicolon),
        47: Token(Var),
        48: Token(Ident, "lambda_val"),
        49: Token(Equal),
        50: Token(Pipe),
        51: Token(Ident, "x"),
        52: Token(Pipe),
        53: Token(Ident, "x"),
        54: Token(Plus),
        55: Token(Int, "1"),
        56: Token(Semicolon),
        57: Token(Ident, "assert"),
        58: Token(LeftParen),
        59: Token(Ident, "int_val"),
        60: Token(Is),
        61: Token(Ident, "Int"),
        62: Token(Comma),
        63: Token(String, "int_val should be Int"),
        64: Token(RightParen),
        65: Token(Semicolon),
        66: Token(Ident, "assert"),
        67: Token(LeftParen),
        68: Token(Ident, "float_val"),
        69: Token(Is),
        70: Token(Ident, "Float"),
        71: Token(Comma),
        72: Token(String, "float_val should be Float"),
        73: Token(RightParen),
        74: Token(Semicolon),
        75: Token(Ident, "assert"),
        76: Token(LeftParen),
        77: Token(Ident, "string_val"),
        78: Token(Is),
        79: Token(Ident, "String"),
        80: Token(Comma),
        81: Token(String, "string_val should be String"),
        82: Token(RightParen),
        83: Token(Semicolon),
        84: Token(Ident, "assert"),
        85: Token(LeftParen),
        86: Token(Ident, "bool_val"),
        87: Token(Is),
        88: Token(Ident, "Bool"),
        89: Token(Comma),
        90: Token(String, "bool_val should be Bool"),
        91: Token(RightParen),
        92: Token(Semicolon),
        93: Token(Ident, "assert"),
        94: Token(LeftParen),
        95: Token(Ident, "list_val"),
        96: Token(Is),
        97: Token(Ident, "List"),
        98: Token(Comma),
        99: Token(String, "list_val should be List"),
        100: Token(RightParen),
        101: Token(Semicolon),
        102: Token(Ident, "assert"),
        103: Token(LeftParen),
        104: Token(Ident, "dict_val"),
        105: Token(Is),
        106: Token(Ident, "Dict"),
        107: Token(Comma),
        108: Token(String, "dict_val should be Dict"),
        109: Token(RightParen),
        110: Token(Semicolon),
        111: Token(Ident, "assert"),
        112: Token(LeftParen),
        113: Token(Ident, "range_val"),
        114: Token(Is),
        115: Token(Ident, "Range"),
        116: Token(Comma),
        117: Token(String, "range_val should be Range"),
        118: Token(RightParen),
        119: Token(Semicolon),
        120: Token(Ident, "assert"),
        121: Token(LeftParen),
        122: Token(Ident, "lambda_val"),
        123: Token(Is),
        124: Token(Ident, "Function"),
        125: Token(Comma),
        126: Token(String, "lambda_val should be Function"),
        127: Token(RightParen),
        128: Token(Semicolon),
        129: Token(Ident, "assert"),
        130: Token(LeftParen),
        131: Token(Ident, "int_val"),
        132: Token(IsNot),
        133: Token(Ident, "String"),
        134: Token(Comma),
        135: Token(String, "int_val should not be String"),
        136: Token(RightParen),
        137: Token(Semicolon),
        138: Token(Ident, "assert"),
        139: Token(LeftParen),
        140: Token(Ident, "float_val"),
        141: Token(IsNot),
        142: Token(Ident, "Int"),
        143: Token(Comma),
        144: Token(String, "float_val should not be Int"),
        145: Token(RightParen),
        146: Token(Semicolon),
        147: Token(Ident, "assert"),
        148: Token(LeftParen),
        149: Token(Ident, "string_val"),
        150: Token(IsNot),
        151: Token(Ident, "Bool"),
        152: Token(Comma),
        153: Token(String, "string_val should not be Bool"),
        154: Token(RightParen),
        155: Token(Semicolon),
        156: Token(Ident, "assert"),
        157: Token(LeftParen),
        158: Token(Ident, "bool_val"),
        159: Token(IsNot),
        160: Token(Ident, "List"),
        161: Token(Comma),
        162: Token(String, "bool_val should not be List"),
        163: Token(RightParen),
        164: Token(Semicolon),
        165: Token(Ident, "assert"),
        166: Token(LeftParen),
        167: Token(Ident, "list_val"),
        168: Token(IsNot),
        169: Token(Ident, "Dict"),
        170: Token(Comma),
        171: Token(String, "list_val should not be Dict"),
        172: Token(RightParen),
        173: Token(Semicolon),
        174: Token(Ident, "assert"),
        175: Token(LeftParen),
        176: Token(Ident, "dict_val"),
        177: Token(IsNot),
        178: Token(Ident, "Range"),
        179: Token(Comma),
        180: Token(String, "dict_val should not be Range"),
        181: Token(RightParen),
        182: Token(Semicolon),
        183: Token(Ident, "assert"),
        184: Token(LeftParen),
        185: Token(Ident, "range_val"),
        186: Token(IsNot),
        187: Token(Ident, "Function"),
        188: Token(Comma),
        189: Token(String, "range_val should not be Function"),
        190: Token(RightParen),
        191: Token(Semicolon),
        192: Token(Ident, "assert"),
        193: Token(LeftParen),
        194: Token(Ident, "lambda_val"),
        195: Token(IsNot),
        196: Token(Ident, "Int"),
        197: Token(Comma),
        198: Token(String, "lambda_val should not be Int"),
        199: Token(RightParen),
        200: Token(Semicolon),
        201: Token(Fn),
        202: Token(Ident, "f"),
        203: Token(LeftParen),
        204: Token(RightParen),
        205: Token(LeftBrace),
        206: Token(RightBrace),
        207: Token(Var),
        208: Token(Ident, "unit"),
        209: Token(Equal),
        210: Token(Ident, "f"),
        211: Token(LeftParen),
        212: Token(RightParen),
        213: Token(Semicolon),
        214: Token(Ident, "assert"),
        215: Token(LeftParen),
        216: Token(Null),
        217: Token(Is),
        218: Token(Ident, "Null"),
        219: Token(Comma),
        220: Token(String, "null should be Null"),
        221: Token(RightParen),
        222: Token(Semicolon),
        223: Token(Ident, "assert"),
        224: Token(LeftParen),
        225: Token(Ident, "unit"),
        226: Token(Is),
        227: Token(Ident, "Unit"),
        228: Token(Comma),
        229: Token(String, "unit should be Unit"),
        230: Token(RightParen),
        231: Token(Semicolon),
        232: Token(Ident, "assert"),
        233: Token(LeftParen),
        234: Token(Null),
        235: Token(IsNot),
        236: Token(Ident, "Unit"),
        237: Token(Comma),
        238: Token(String, "null should not be Unit"),
        239: Token(RightParen),
        240: Token(Semicolon),
        241: Token(Ident, "assert"),
        242: Token(LeftParen),
        243: Token(Ident, "unit"),
        244: Token(IsNot),
        245: Token(Ident, "Null"),
        246: Token(Comma),
        247: Token(String, "unit should not be Null"),
        248: Token(RightParen),
        249: Token(Semicolon),
        250: Token(Var),
        251: Token(Ident, "list1"),
        252: Token(Equal),
        253: Token(LeftBracket),
        254: Token(Int, "1"),
        255: Token(Comma),
        256: Token(Int, "2"),
        257: Token(Comma),
        258: Token(Int, "3"),
        259: Token(RightBracket),
        260: Token(Semicolon),
        261: Token(Var),
        262: Token(Ident, "list2"),
        263: Token(Equal),
        264: Token(LeftBracket),
        265: Token(Int, "1"),
        266: Token(Comma),
        267: Token(Int, "2"),
        268: Token(Comma),
        269: Token(Int, "3"),
        270: Token(RightBracket),
        271: Token(Semicolon),
        272: Token(Var),
        273: Token(Ident, "list3"),
        274: Token(Equal),
        275: Token(Ident, "list1"),
        276: Token(Semicolon),
        277: Token(Ident, "assert"),
        278: Token(LeftParen),
        279: Token(Ident, "list1"),
        280: Token(IsNot),
        281: Token(Ident, "list2"),
        282: Token(Comma),
        283: Token(String, "list1 should not be identical to list2 (different objects)"),
        284: Token(RightParen),
        285: Token(Semicolon),
        286: Token(Ident, "assert"),
        287: Token(LeftParen),
        288: Token(Ident, "list1"),
        289: Token(Is),
        290: Token(Ident, "list3"),
        291: Token(Comma),
        292: Token(String, "list1 should be identical to list3 (same object reference)"),
        293: Token(RightParen),
        294: Token(Semicolon),
        295: Token(Eof)
    ],
)
```

# Parse
```rust
Ok(
    Program(
        [
            VarDecl {
                pattern: Variable(
                    "int_val",
                ),
                value: Some(
                    Literal(
                        Int(
                            42,
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "float_val",
                ),
                value: Some(
                    Literal(
                        Float(
                            3.14,
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "string_val",
                ),
                value: Some(
                    Literal(
                        String(
                            "hello",
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "bool_val",
                ),
                value: Some(
                    Literal(
                        Bool(
                            true,
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "list_val",
                ),
                value: Some(
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
                        ],
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "dict_val",
                ),
                value: Some(
                    Dict(
                        [
                            (
                                Literal(
                                    String(
                                        "key",
                                    ),
                                ),
                                Literal(
                                    String(
                                        "value",
                                    ),
                                ),
                            ),
                        ],
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "range_val",
                ),
                value: Some(
                    RangeExclusive(
                        Literal(
                            Int(
                                1,
                            ),
                        ),
                        Literal(
                            Int(
                                10,
                            ),
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "lambda_val",
                ),
                value: Some(
                    Lambda {
                        params: [
                            "x",
                        ],
                        body: Expression(
                            Add(
                                Identifier(
                                    "x",
                                ),
                                Literal(
                                    Int(
                                        1,
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            },
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Is(
                            Identifier(
                                "int_val",
                            ),
                            Identifier(
                                "Int",
                            ),
                        ),
                        Literal(
                            String(
                                "int_val should be Int",
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
                        Is(
                            Identifier(
                                "float_val",
                            ),
                            Identifier(
                                "Float",
                            ),
                        ),
                        Literal(
                            String(
                                "float_val should be Float",
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
                        Is(
                            Identifier(
                                "string_val",
                            ),
                            Identifier(
                                "String",
                            ),
                        ),
                        Literal(
                            String(
                                "string_val should be String",
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
                        Is(
                            Identifier(
                                "bool_val",
                            ),
                            Identifier(
                                "Bool",
                            ),
                        ),
                        Literal(
                            String(
                                "bool_val should be Bool",
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
                        Is(
                            Identifier(
                                "list_val",
                            ),
                            Identifier(
                                "List",
                            ),
                        ),
                        Literal(
                            String(
                                "list_val should be List",
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
                        Is(
                            Identifier(
                                "dict_val",
                            ),
                            Identifier(
                                "Dict",
                            ),
                        ),
                        Literal(
                            String(
                                "dict_val should be Dict",
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
                        Is(
                            Identifier(
                                "range_val",
                            ),
                            Identifier(
                                "Range",
                            ),
                        ),
                        Literal(
                            String(
                                "range_val should be Range",
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
                        Is(
                            Identifier(
                                "lambda_val",
                            ),
                            Identifier(
                                "Function",
                            ),
                        ),
                        Literal(
                            String(
                                "lambda_val should be Function",
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
                        IsNot(
                            Identifier(
                                "int_val",
                            ),
                            Identifier(
                                "String",
                            ),
                        ),
                        Literal(
                            String(
                                "int_val should not be String",
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
                        IsNot(
                            Identifier(
                                "float_val",
                            ),
                            Identifier(
                                "Int",
                            ),
                        ),
                        Literal(
                            String(
                                "float_val should not be Int",
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
                        IsNot(
                            Identifier(
                                "string_val",
                            ),
                            Identifier(
                                "Bool",
                            ),
                        ),
                        Literal(
                            String(
                                "string_val should not be Bool",
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
                        IsNot(
                            Identifier(
                                "bool_val",
                            ),
                            Identifier(
                                "List",
                            ),
                        ),
                        Literal(
                            String(
                                "bool_val should not be List",
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
                        IsNot(
                            Identifier(
                                "list_val",
                            ),
                            Identifier(
                                "Dict",
                            ),
                        ),
                        Literal(
                            String(
                                "list_val should not be Dict",
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
                        IsNot(
                            Identifier(
                                "dict_val",
                            ),
                            Identifier(
                                "Range",
                            ),
                        ),
                        Literal(
                            String(
                                "dict_val should not be Range",
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
                        IsNot(
                            Identifier(
                                "range_val",
                            ),
                            Identifier(
                                "Function",
                            ),
                        ),
                        Literal(
                            String(
                                "range_val should not be Function",
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
                        IsNot(
                            Identifier(
                                "lambda_val",
                            ),
                            Identifier(
                                "Int",
                            ),
                        ),
                        Literal(
                            String(
                                "lambda_val should not be Int",
                            ),
                        ),
                    ],
                ),
            ),
            FnDecl {
                name: "f",
                params: [],
                body: Block {
                    statements: [],
                    final_expr: None,
                },
                is_pub: false,
            },
            VarDecl {
                pattern: Variable(
                    "unit",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "f",
                        ),
                        [],
                    ),
                ),
            },
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Is(
                            Literal(
                                Null,
                            ),
                            Identifier(
                                "Null",
                            ),
                        ),
                        Literal(
                            String(
                                "null should be Null",
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
                        Is(
                            Identifier(
                                "unit",
                            ),
                            Identifier(
                                "Unit",
                            ),
                        ),
                        Literal(
                            String(
                                "unit should be Unit",
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
                        IsNot(
                            Literal(
                                Null,
                            ),
                            Identifier(
                                "Unit",
                            ),
                        ),
                        Literal(
                            String(
                                "null should not be Unit",
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
                        IsNot(
                            Identifier(
                                "unit",
                            ),
                            Identifier(
                                "Null",
                            ),
                        ),
                        Literal(
                            String(
                                "unit should not be Null",
                            ),
                        ),
                    ],
                ),
            ),
            VarDecl {
                pattern: Variable(
                    "list1",
                ),
                value: Some(
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
                        ],
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "list2",
                ),
                value: Some(
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
                        ],
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "list3",
                ),
                value: Some(
                    Identifier(
                        "list1",
                    ),
                ),
            },
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        IsNot(
                            Identifier(
                                "list1",
                            ),
                            Identifier(
                                "list2",
                            ),
                        ),
                        Literal(
                            String(
                                "list1 should not be identical to list2 (different objects)",
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
                        Is(
                            Identifier(
                                "list1",
                            ),
                            Identifier(
                                "list3",
                            ),
                        ),
                        Literal(
                            String(
                                "list1 should be identical to list3 (same object reference)",
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