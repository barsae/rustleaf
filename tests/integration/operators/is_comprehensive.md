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
```
parse_program: starting
parse_program: parsing statement at position 0 (Var)
parse_statement: starting at position 0 (Var)
consume_token: position 0 consumed Var
consume_token: position 1 consumed Ident
consume_token: position 2 consumed Equal
parse_expression: starting at position 3 (Int(42))
consume_token: position 3 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 4 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 5 (Var)
parse_statement: starting at position 5 (Var)
consume_token: position 5 consumed Var
consume_token: position 6 consumed Ident
consume_token: position 7 consumed Equal
parse_expression: starting at position 8 (Float(3.14))
consume_token: position 8 consumed Float
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 9 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 10 (Var)
parse_statement: starting at position 10 (Var)
consume_token: position 10 consumed Var
consume_token: position 11 consumed Ident
consume_token: position 12 consumed Equal
parse_expression: starting at position 13 (String(hello))
consume_token: position 13 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 14 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 15 (Var)
parse_statement: starting at position 15 (Var)
consume_token: position 15 consumed Var
consume_token: position 16 consumed Ident
consume_token: position 17 consumed Equal
parse_expression: starting at position 18 (True)
consume_token: position 18 consumed True
parse_primary: success - parsed boolean literal (true)
parse_expression: success - parsed precedence expression
consume_token: position 19 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 20 (Var)
parse_statement: starting at position 20 (Var)
consume_token: position 20 consumed Var
consume_token: position 21 consumed Ident
consume_token: position 22 consumed Equal
parse_expression: starting at position 23 (LeftBracket)
consume_token: position 23 consumed LeftBracket
parse_primary: success - parsing list literal
parse_list_literal: starting at position 24
parse_list_literal: parsing element at position 24
parse_expression: starting at position 24 (Int(1))
consume_token: position 24 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 25 consumed Comma
parse_list_literal: found comma, checking for more elements
parse_list_literal: parsing element at position 26
parse_expression: starting at position 26 (Int(2))
consume_token: position 26 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 27 consumed Comma
parse_list_literal: found comma, checking for more elements
parse_list_literal: parsing element at position 28
parse_expression: starting at position 28 (Int(3))
consume_token: position 28 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_list_literal: no comma, expecting ]
parse_list_literal: expecting ] at position 29
consume_token: position 29 consumed RightBracket
parse_list_literal: success
parse_expression: success - parsed precedence expression
consume_token: position 30 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 31 (Var)
parse_statement: starting at position 31 (Var)
consume_token: position 31 consumed Var
consume_token: position 32 consumed Ident
consume_token: position 33 consumed Equal
parse_expression: starting at position 34 (LeftBrace)
parse_primary: success - parsing block or dict
consume_token: position 34 consumed LeftBrace
consume_token: position 35 consumed String
parse_primary: success - parsed numeric/string literal
consume_token: position 36 consumed Colon
parse_expression: starting at position 37 (String(value))
consume_token: position 37 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 38 consumed RightBrace
parse_expression: success - parsed precedence expression
consume_token: position 39 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 40 (Var)
parse_statement: starting at position 40 (Var)
consume_token: position 40 consumed Var
consume_token: position 41 consumed Ident
consume_token: position 42 consumed Equal
parse_expression: starting at position 43 (Int(1))
consume_token: position 43 consumed Int
parse_primary: success - parsed numeric/string literal
consume_token: position 44 consumed DotDot
consume_token: position 45 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 46 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 47 (Var)
parse_statement: starting at position 47 (Var)
consume_token: position 47 consumed Var
consume_token: position 48 consumed Ident
consume_token: position 49 consumed Equal
parse_expression: starting at position 50 (Pipe)
parse_primary: success - parsing lambda expression
consume_token: position 50 consumed Pipe
consume_token: position 51 consumed Ident
consume_token: position 52 consumed Pipe
parse_expression: starting at position 53 (Ident(x))
consume_token: position 53 consumed Ident
parse_primary: success - parsed identifier (x)
consume_token: position 54 consumed Plus
consume_token: position 55 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
consume_token: position 56 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 57 (Ident(assert))
parse_statement: starting at position 57 (Ident(assert))
consume_token: position 57 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 57 (Ident(assert))
consume_token: position 57 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 58 consumed LeftParen
parse_expression: starting at position 59 (Ident(int_val))
consume_token: position 59 consumed Ident
parse_primary: success - parsed identifier (int_val)
consume_token: position 60 consumed Is
consume_token: position 61 consumed Ident
parse_primary: success - parsed identifier (Int)
parse_expression: success - parsed precedence expression
consume_token: position 62 consumed Comma
parse_expression: starting at position 63 (String(int_val should be Int))
consume_token: position 63 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 64 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 65 consumed Semicolon
parse_program: parsing statement at position 66 (Ident(assert))
parse_statement: starting at position 66 (Ident(assert))
consume_token: position 66 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 66 (Ident(assert))
consume_token: position 66 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 67 consumed LeftParen
parse_expression: starting at position 68 (Ident(float_val))
consume_token: position 68 consumed Ident
parse_primary: success - parsed identifier (float_val)
consume_token: position 69 consumed Is
consume_token: position 70 consumed Ident
parse_primary: success - parsed identifier (Float)
parse_expression: success - parsed precedence expression
consume_token: position 71 consumed Comma
parse_expression: starting at position 72 (String(float_val should be Float))
consume_token: position 72 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 73 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 74 consumed Semicolon
parse_program: parsing statement at position 75 (Ident(assert))
parse_statement: starting at position 75 (Ident(assert))
consume_token: position 75 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 75 (Ident(assert))
consume_token: position 75 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 76 consumed LeftParen
parse_expression: starting at position 77 (Ident(string_val))
consume_token: position 77 consumed Ident
parse_primary: success - parsed identifier (string_val)
consume_token: position 78 consumed Is
consume_token: position 79 consumed Ident
parse_primary: success - parsed identifier (String)
parse_expression: success - parsed precedence expression
consume_token: position 80 consumed Comma
parse_expression: starting at position 81 (String(string_val should be String))
consume_token: position 81 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 82 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 83 consumed Semicolon
parse_program: parsing statement at position 84 (Ident(assert))
parse_statement: starting at position 84 (Ident(assert))
consume_token: position 84 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 84 (Ident(assert))
consume_token: position 84 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 85 consumed LeftParen
parse_expression: starting at position 86 (Ident(bool_val))
consume_token: position 86 consumed Ident
parse_primary: success - parsed identifier (bool_val)
consume_token: position 87 consumed Is
consume_token: position 88 consumed Ident
parse_primary: success - parsed identifier (Bool)
parse_expression: success - parsed precedence expression
consume_token: position 89 consumed Comma
parse_expression: starting at position 90 (String(bool_val should be Bool))
consume_token: position 90 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 91 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 92 consumed Semicolon
parse_program: parsing statement at position 93 (Ident(assert))
parse_statement: starting at position 93 (Ident(assert))
consume_token: position 93 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 93 (Ident(assert))
consume_token: position 93 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 94 consumed LeftParen
parse_expression: starting at position 95 (Ident(list_val))
consume_token: position 95 consumed Ident
parse_primary: success - parsed identifier (list_val)
consume_token: position 96 consumed Is
consume_token: position 97 consumed Ident
parse_primary: success - parsed identifier (List)
parse_expression: success - parsed precedence expression
consume_token: position 98 consumed Comma
parse_expression: starting at position 99 (String(list_val should be List))
consume_token: position 99 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 100 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 101 consumed Semicolon
parse_program: parsing statement at position 102 (Ident(assert))
parse_statement: starting at position 102 (Ident(assert))
consume_token: position 102 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 102 (Ident(assert))
consume_token: position 102 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 103 consumed LeftParen
parse_expression: starting at position 104 (Ident(dict_val))
consume_token: position 104 consumed Ident
parse_primary: success - parsed identifier (dict_val)
consume_token: position 105 consumed Is
consume_token: position 106 consumed Ident
parse_primary: success - parsed identifier (Dict)
parse_expression: success - parsed precedence expression
consume_token: position 107 consumed Comma
parse_expression: starting at position 108 (String(dict_val should be Dict))
consume_token: position 108 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 109 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 110 consumed Semicolon
parse_program: parsing statement at position 111 (Ident(assert))
parse_statement: starting at position 111 (Ident(assert))
consume_token: position 111 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 111 (Ident(assert))
consume_token: position 111 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 112 consumed LeftParen
parse_expression: starting at position 113 (Ident(range_val))
consume_token: position 113 consumed Ident
parse_primary: success - parsed identifier (range_val)
consume_token: position 114 consumed Is
consume_token: position 115 consumed Ident
parse_primary: success - parsed identifier (Range)
parse_expression: success - parsed precedence expression
consume_token: position 116 consumed Comma
parse_expression: starting at position 117 (String(range_val should be Range))
consume_token: position 117 consumed String
parse_primary: success - parsed numeric/string literal
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
parse_expression: starting at position 122 (Ident(lambda_val))
consume_token: position 122 consumed Ident
parse_primary: success - parsed identifier (lambda_val)
consume_token: position 123 consumed Is
consume_token: position 124 consumed Ident
parse_primary: success - parsed identifier (Function)
parse_expression: success - parsed precedence expression
consume_token: position 125 consumed Comma
parse_expression: starting at position 126 (String(lambda_val should be Function))
consume_token: position 126 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 127 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 128 consumed Semicolon
parse_program: parsing statement at position 129 (Ident(assert))
parse_statement: starting at position 129 (Ident(assert))
consume_token: position 129 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 129 (Ident(assert))
consume_token: position 129 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 130 consumed LeftParen
parse_expression: starting at position 131 (Ident(int_val))
consume_token: position 131 consumed Ident
parse_primary: success - parsed identifier (int_val)
consume_token: position 132 consumed IsNot
consume_token: position 133 consumed Ident
parse_primary: success - parsed identifier (String)
parse_expression: success - parsed precedence expression
consume_token: position 134 consumed Comma
parse_expression: starting at position 135 (String(int_val should not be String))
consume_token: position 135 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 136 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 137 consumed Semicolon
parse_program: parsing statement at position 138 (Ident(assert))
parse_statement: starting at position 138 (Ident(assert))
consume_token: position 138 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 138 (Ident(assert))
consume_token: position 138 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 139 consumed LeftParen
parse_expression: starting at position 140 (Ident(float_val))
consume_token: position 140 consumed Ident
parse_primary: success - parsed identifier (float_val)
consume_token: position 141 consumed IsNot
consume_token: position 142 consumed Ident
parse_primary: success - parsed identifier (Int)
parse_expression: success - parsed precedence expression
consume_token: position 143 consumed Comma
parse_expression: starting at position 144 (String(float_val should not be Int))
consume_token: position 144 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 145 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 146 consumed Semicolon
parse_program: parsing statement at position 147 (Ident(assert))
parse_statement: starting at position 147 (Ident(assert))
consume_token: position 147 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 147 (Ident(assert))
consume_token: position 147 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 148 consumed LeftParen
parse_expression: starting at position 149 (Ident(string_val))
consume_token: position 149 consumed Ident
parse_primary: success - parsed identifier (string_val)
consume_token: position 150 consumed IsNot
consume_token: position 151 consumed Ident
parse_primary: success - parsed identifier (Bool)
parse_expression: success - parsed precedence expression
consume_token: position 152 consumed Comma
parse_expression: starting at position 153 (String(string_val should not be Bool))
consume_token: position 153 consumed String
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
parse_expression: starting at position 158 (Ident(bool_val))
consume_token: position 158 consumed Ident
parse_primary: success - parsed identifier (bool_val)
consume_token: position 159 consumed IsNot
consume_token: position 160 consumed Ident
parse_primary: success - parsed identifier (List)
parse_expression: success - parsed precedence expression
consume_token: position 161 consumed Comma
parse_expression: starting at position 162 (String(bool_val should not be List))
consume_token: position 162 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 163 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 164 consumed Semicolon
parse_program: parsing statement at position 165 (Ident(assert))
parse_statement: starting at position 165 (Ident(assert))
consume_token: position 165 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 165 (Ident(assert))
consume_token: position 165 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 166 consumed LeftParen
parse_expression: starting at position 167 (Ident(list_val))
consume_token: position 167 consumed Ident
parse_primary: success - parsed identifier (list_val)
consume_token: position 168 consumed IsNot
consume_token: position 169 consumed Ident
parse_primary: success - parsed identifier (Dict)
parse_expression: success - parsed precedence expression
consume_token: position 170 consumed Comma
parse_expression: starting at position 171 (String(list_val should not be Dict))
consume_token: position 171 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 172 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 173 consumed Semicolon
parse_program: parsing statement at position 174 (Ident(assert))
parse_statement: starting at position 174 (Ident(assert))
consume_token: position 174 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 174 (Ident(assert))
consume_token: position 174 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 175 consumed LeftParen
parse_expression: starting at position 176 (Ident(dict_val))
consume_token: position 176 consumed Ident
parse_primary: success - parsed identifier (dict_val)
consume_token: position 177 consumed IsNot
consume_token: position 178 consumed Ident
parse_primary: success - parsed identifier (Range)
parse_expression: success - parsed precedence expression
consume_token: position 179 consumed Comma
parse_expression: starting at position 180 (String(dict_val should not be Range))
consume_token: position 180 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 181 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 182 consumed Semicolon
parse_program: parsing statement at position 183 (Ident(assert))
parse_statement: starting at position 183 (Ident(assert))
consume_token: position 183 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 183 (Ident(assert))
consume_token: position 183 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 184 consumed LeftParen
parse_expression: starting at position 185 (Ident(range_val))
consume_token: position 185 consumed Ident
parse_primary: success - parsed identifier (range_val)
consume_token: position 186 consumed IsNot
consume_token: position 187 consumed Ident
parse_primary: success - parsed identifier (Function)
parse_expression: success - parsed precedence expression
consume_token: position 188 consumed Comma
parse_expression: starting at position 189 (String(range_val should not be Function))
consume_token: position 189 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 190 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 191 consumed Semicolon
parse_program: parsing statement at position 192 (Ident(assert))
parse_statement: starting at position 192 (Ident(assert))
consume_token: position 192 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 192 (Ident(assert))
consume_token: position 192 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 193 consumed LeftParen
parse_expression: starting at position 194 (Ident(lambda_val))
consume_token: position 194 consumed Ident
parse_primary: success - parsed identifier (lambda_val)
consume_token: position 195 consumed IsNot
consume_token: position 196 consumed Ident
parse_primary: success - parsed identifier (Int)
parse_expression: success - parsed precedence expression
consume_token: position 197 consumed Comma
parse_expression: starting at position 198 (String(lambda_val should not be Int))
consume_token: position 198 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 199 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 200 consumed Semicolon
parse_program: parsing statement at position 201 (Fn)
parse_statement: starting at position 201 (Fn)
consume_token: position 201 consumed Fn
consume_token: position 202 consumed Ident
consume_token: position 203 consumed LeftParen
consume_token: position 204 consumed RightParen
consume_token: position 205 consumed LeftBrace
consume_token: position 206 consumed RightBrace
parse_statement: success - parsed function declaration
parse_program: parsing statement at position 207 (Var)
parse_statement: starting at position 207 (Var)
consume_token: position 207 consumed Var
consume_token: position 208 consumed Ident
consume_token: position 209 consumed Equal
parse_expression: starting at position 210 (Ident(f))
consume_token: position 210 consumed Ident
parse_primary: success - parsed identifier (f)
consume_token: position 211 consumed LeftParen
consume_token: position 212 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 213 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 214 (Ident(assert))
parse_statement: starting at position 214 (Ident(assert))
consume_token: position 214 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 214 (Ident(assert))
consume_token: position 214 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 215 consumed LeftParen
parse_expression: starting at position 216 (Null)
consume_token: position 216 consumed Null
parse_primary: success - parsed null literal
consume_token: position 217 consumed Is
consume_token: position 218 consumed Ident
parse_primary: success - parsed identifier (Null)
parse_expression: success - parsed precedence expression
consume_token: position 219 consumed Comma
parse_expression: starting at position 220 (String(null should be Null))
consume_token: position 220 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 221 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 222 consumed Semicolon
parse_program: parsing statement at position 223 (Ident(assert))
parse_statement: starting at position 223 (Ident(assert))
consume_token: position 223 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 223 (Ident(assert))
consume_token: position 223 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 224 consumed LeftParen
parse_expression: starting at position 225 (Ident(unit))
consume_token: position 225 consumed Ident
parse_primary: success - parsed identifier (unit)
consume_token: position 226 consumed Is
consume_token: position 227 consumed Ident
parse_primary: success - parsed identifier (Unit)
parse_expression: success - parsed precedence expression
consume_token: position 228 consumed Comma
parse_expression: starting at position 229 (String(unit should be Unit))
consume_token: position 229 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 230 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 231 consumed Semicolon
parse_program: parsing statement at position 232 (Ident(assert))
parse_statement: starting at position 232 (Ident(assert))
consume_token: position 232 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 232 (Ident(assert))
consume_token: position 232 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 233 consumed LeftParen
parse_expression: starting at position 234 (Null)
consume_token: position 234 consumed Null
parse_primary: success - parsed null literal
consume_token: position 235 consumed IsNot
consume_token: position 236 consumed Ident
parse_primary: success - parsed identifier (Unit)
parse_expression: success - parsed precedence expression
consume_token: position 237 consumed Comma
parse_expression: starting at position 238 (String(null should not be Unit))
consume_token: position 238 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 239 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 240 consumed Semicolon
parse_program: parsing statement at position 241 (Ident(assert))
parse_statement: starting at position 241 (Ident(assert))
consume_token: position 241 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 241 (Ident(assert))
consume_token: position 241 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 242 consumed LeftParen
parse_expression: starting at position 243 (Ident(unit))
consume_token: position 243 consumed Ident
parse_primary: success - parsed identifier (unit)
consume_token: position 244 consumed IsNot
consume_token: position 245 consumed Ident
parse_primary: success - parsed identifier (Null)
parse_expression: success - parsed precedence expression
consume_token: position 246 consumed Comma
parse_expression: starting at position 247 (String(unit should not be Null))
consume_token: position 247 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 248 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 249 consumed Semicolon
parse_program: parsing statement at position 250 (Var)
parse_statement: starting at position 250 (Var)
consume_token: position 250 consumed Var
consume_token: position 251 consumed Ident
consume_token: position 252 consumed Equal
parse_expression: starting at position 253 (LeftBracket)
consume_token: position 253 consumed LeftBracket
parse_primary: success - parsing list literal
parse_list_literal: starting at position 254
parse_list_literal: parsing element at position 254
parse_expression: starting at position 254 (Int(1))
consume_token: position 254 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 255 consumed Comma
parse_list_literal: found comma, checking for more elements
parse_list_literal: parsing element at position 256
parse_expression: starting at position 256 (Int(2))
consume_token: position 256 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 257 consumed Comma
parse_list_literal: found comma, checking for more elements
parse_list_literal: parsing element at position 258
parse_expression: starting at position 258 (Int(3))
consume_token: position 258 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_list_literal: no comma, expecting ]
parse_list_literal: expecting ] at position 259
consume_token: position 259 consumed RightBracket
parse_list_literal: success
parse_expression: success - parsed precedence expression
consume_token: position 260 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 261 (Var)
parse_statement: starting at position 261 (Var)
consume_token: position 261 consumed Var
consume_token: position 262 consumed Ident
consume_token: position 263 consumed Equal
parse_expression: starting at position 264 (LeftBracket)
consume_token: position 264 consumed LeftBracket
parse_primary: success - parsing list literal
parse_list_literal: starting at position 265
parse_list_literal: parsing element at position 265
parse_expression: starting at position 265 (Int(1))
consume_token: position 265 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 266 consumed Comma
parse_list_literal: found comma, checking for more elements
parse_list_literal: parsing element at position 267
parse_expression: starting at position 267 (Int(2))
consume_token: position 267 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 268 consumed Comma
parse_list_literal: found comma, checking for more elements
parse_list_literal: parsing element at position 269
parse_expression: starting at position 269 (Int(3))
consume_token: position 269 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_list_literal: no comma, expecting ]
parse_list_literal: expecting ] at position 270
consume_token: position 270 consumed RightBracket
parse_list_literal: success
parse_expression: success - parsed precedence expression
consume_token: position 271 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 272 (Var)
parse_statement: starting at position 272 (Var)
consume_token: position 272 consumed Var
consume_token: position 273 consumed Ident
consume_token: position 274 consumed Equal
parse_expression: starting at position 275 (Ident(list1))
consume_token: position 275 consumed Ident
parse_primary: success - parsed identifier (list1)
parse_expression: success - parsed precedence expression
consume_token: position 276 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 277 (Ident(assert))
parse_statement: starting at position 277 (Ident(assert))
consume_token: position 277 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 277 (Ident(assert))
consume_token: position 277 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 278 consumed LeftParen
parse_expression: starting at position 279 (Ident(list1))
consume_token: position 279 consumed Ident
parse_primary: success - parsed identifier (list1)
consume_token: position 280 consumed IsNot
consume_token: position 281 consumed Ident
parse_primary: success - parsed identifier (list2)
parse_expression: success - parsed precedence expression
consume_token: position 282 consumed Comma
parse_expression: starting at position 283 (String(list1 should not be identical to list2 (different objects)))
consume_token: position 283 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 284 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 285 consumed Semicolon
parse_program: parsing statement at position 286 (Ident(assert))
parse_statement: starting at position 286 (Ident(assert))
consume_token: position 286 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 286 (Ident(assert))
consume_token: position 286 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 287 consumed LeftParen
parse_expression: starting at position 288 (Ident(list1))
consume_token: position 288 consumed Ident
parse_primary: success - parsed identifier (list1)
consume_token: position 289 consumed Is
consume_token: position 290 consumed Ident
parse_primary: success - parsed identifier (list3)
parse_expression: success - parsed precedence expression
consume_token: position 291 consumed Comma
parse_expression: starting at position 292 (String(list1 should be identical to list3 (same object reference)))
consume_token: position 292 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 293 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 294 consumed Semicolon
parse_program: parsed 35 statements
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