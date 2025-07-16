# 3. Types

RustLeaf is a dynamically typed language where types are determined and checked at runtime. This chapter defines the type system, including primitive types, composite types, and the rules governing type conversions and type checking.

### 3.1. Type System Overview

RustLeaf employs dynamic typing with strong type checking at runtime. Variables can hold values of any type, and the type of a value is determined when it is created.

**Core Type System Properties:**
- **Dynamic Typing**: Variables do not have declared types; they acquire the type of their assigned value
- **Strong Typing**: Operations check types at runtime and raise errors for invalid type combinations
- **No Implicit Conversions**: Values are never automatically converted between types (except for string interpolation and display)
- **Runtime Type Information**: Full type information is available at runtime via the `type()` function

**Type Categories:**
1. **Primitive Types**: null, bool, int, float, string
2. **Composite Types**: list, dict, object (class instances)
3. **Callable Types**: function (including closures and methods)
4. **Extension Types**: RustValue (Rust-implemented types)

**Type Identity:**
- Each value has exactly one type
- Types are identified by their type name (a string)
- Type names are compared using string equality

**Example:**
```
var x = 42          // x holds an int
var t = type(x)     // t is "int"
x = "hello"         // x now holds a string
print(type(x))      // "string"

// Type checking
if type(value) == "int" {
    // value is definitely an integer
}
```

### 3.2. Primitive Types

Primitive types are the fundamental building blocks of the type system. They are immutable (except for their container in variables) and have value semantics.

#### 3.2.1. Null Type

The null type has exactly one value: `null`.

**Properties:**
- Type name: `"null"`
- Represents the absence of a value
- Used as a default return value when no explicit value is provided
- Truthiness: `null` is falsy in boolean contexts

**Operations:**
- Equality: `null == null` is `true`
- Type check: `type(null) == "null"`
- No other operations are valid on null

**Example:**
```
var x = null
print(type(x))      // "null"
if not x {          // null is falsy
    print("x is null")
}
```

#### 3.2.2. Boolean Type  

The boolean type represents logical truth values.

**Properties:**
- Type name: `"bool"`
- Exactly two values: `true` and `false`
- Result type of comparison and logical operations
- Truthiness: `false` is falsy, `true` is truthy

**Operations:**
- Logical: `and`, `or`, `not`
- Equality: `==`, `!=`
- Type check: `type(true) == "bool"`

**Example:**
```
var a = true
var b = false
print(a and b)      // false
print(a or b)       // true
print(not a)        // false
```

#### 3.2.3. Numeric Types

RustLeaf has two numeric types: integers and floating-point numbers.

**Integer Type (int):**
- Type name: `"int"`
- 64-bit signed integers (-2^63 to 2^63-1)
- Overflow/underflow raises a runtime error
- No implicit conversion to float

**Floating-Point Type (float):**
- Type name: `"float"`
- IEEE 754 double-precision (64-bit)
- Supports special values: Infinity, -Infinity, NaN
- NaN propagates through operations
- No implicit conversion to int

**Numeric Operations:**
```
// Arithmetic (int and float)
+   // Addition
-   // Subtraction/negation  
*   // Multiplication
/   // Division (int division truncates, float division is exact)
%   // Modulo
**  // Exponentiation

// Bitwise (int only)
&   // Bitwise AND
|   // Bitwise OR
^   // Bitwise XOR
~   // Bitwise NOT
<<  // Left shift
>>  // Right shift

// Comparison (int and float)
<   // Less than
>   // Greater than
<=  // Less than or equal
>=  // Greater than or equal
==  // Equal
!=  // Not equal
```

**Overflow Behavior:**
```
var max_int = 9223372036854775807
var overflow = max_int + 1  // Error: Integer overflow

var min_int = -9223372036854775808  
var underflow = min_int - 1  // Error: Integer underflow
```

**Example:**
```
// Integers
var x = 42
var y = 13
print(x + y)        // 55
print(x / y)        // 3 (integer division)
print(x % y)        // 3 (remainder)

// Floats
var a = 3.14
var b = 2.0
print(a / b)        // 1.57
print(type(a))      // "float"

// NaN and Infinity
var inf = 1.0 / 0.0   // Infinity
var nan = 0.0 / 0.0   // NaN
print(inf > 1000000)  // true
print(nan == nan)     // false (NaN != NaN)
```

#### 3.2.4. String Type

Strings are immutable sequences of Unicode characters.

**Properties:**
- Type name: `"string"`
- UTF-8 encoded internally
- Immutable (operations return new strings)
- Support for string interpolation

**String Operations:**
- Concatenation: `+` operator
- Repetition: `*` operator with integer
- Comparison: lexicographic ordering
- Indexing: `str[index]` returns single-character string
- Slicing: `str[start:end]` returns substring
- Length: `len(str)` returns character count

**String Methods:**
- `split(delimiter)` - Split into list of strings
- `trim()` - Remove leading/trailing whitespace
- `upper()` - Convert to uppercase
- `lower()` - Convert to lowercase
- `replace(old, new)` - Replace all occurrences
- `contains(substring)` - Test for substring presence
- `starts_with(prefix)` - Test prefix
- `ends_with(suffix)` - Test suffix
- `is_empty()` - Test if length is 0

**Example:**
```
var s = "Hello"
var t = "World"
var combined = s + ", " + t + "!"  // "Hello, World!"

// String interpolation
var name = "Alice"
var age = 30
var message = "My name is ${name} and I am ${age} years old"

// String methods
var text = "  Hello, World!  "
print(text.trim())          // "Hello, World!"
print(text.upper())         // "  HELLO, WORLD!  "
print(text.contains("World")) // true
```

### 3.3. Composite Types

Composite types are mutable containers that can hold multiple values.

#### 3.3.1. List Type

Lists are ordered, mutable sequences that can contain values of any type.

**Properties:**
- Type name: `"list"`
- Zero-indexed
- Heterogeneous (can mix types)
- Mutable (can be modified in place)
- Dynamic size

**List Operations:**
- Creation: `[expr1, expr2, ...]`
- Indexing: `list[index]` (negative indices count from end)
- Slicing: `list[start:end]`
- Length: `len(list)`
- Membership: `value in list`
- Concatenation: `list1 + list2` (returns new list)

**List Methods:**
- `append(value)` - Add to end
- `extend(other_list)` - Add all elements from other list
- `insert(index, value)` - Insert at position
- `pop(index?)` - Remove and return element (default: last)
- `remove(value)` - Remove first occurrence
- `clear()` - Remove all elements
- `sort()` - Sort in place
- `reverse()` - Reverse in place
- `map(function)` - Transform elements (returns new list)
- `filter(function)` - Select elements (returns new list)
- `reduce(function, initial?)` - Reduce to single value
- `is_empty()` - Test if length is 0

**Example:**
```
var nums = [1, 2, 3]
nums.append(4)              // [1, 2, 3, 4]
nums[0] = 10               // [10, 2, 3, 4]

var mixed = [1, "hello", true, [2, 3]]
print(len(mixed))          // 4
print(mixed[3][1])         // 3

// Functional methods
var doubled = nums.map(fn(x) { x * 2 })
var evens = nums.filter(fn(x) { x % 2 == 0 })
var sum = nums.reduce(fn(a, b) { a + b }, 0)
```

#### 3.3.2. Dict Type

Dictionaries are mutable mappings from keys to values with preserved insertion order.

**Properties:**
- Type name: `"dict"`
- Keys must be immutable types (string, int, float, bool, null)
- Values can be any type
- Preserves insertion order
- Mutable

**Dict Operations:**
- Creation: `{key1: value1, key2: value2, ...}`
- Access: `dict[key]` (raises error if key not found)
- Assignment: `dict[key] = value`
- Membership: `key in dict`
- Length: `len(dict)`

**Dict Methods:**
- `get(key, default?)` - Get value or default if not found
- `set(key, value)` - Set key-value pair
- `pop(key, default?)` - Remove and return value
- `clear()` - Remove all entries
- `keys()` - Return list of keys
- `values()` - Return list of values  
- `items()` - Return list of [key, value] pairs
- `update(other_dict)` - Merge other dict into this one
- `has(key)` - Test if key exists
- `is_empty()` - Test if length is 0

**Example:**
```
var person = {
    "name": "Alice",
    "age": 30,
    "hobbies": ["reading", "coding"]
}

person["city"] = "New York"
print(person.get("age"))           // 30
print(person.get("phone", "N/A"))  // "N/A"

for key, value in person.items() {
    print("${key}: ${value}")
}
```

#### 3.3.3. Object Type

Objects are instances of user-defined classes.

**Properties:**
- Type name: The class name
- Contains fields (data) and methods (behavior)
- Mutable
- Each class creates a distinct type

**Object Creation:**
- Call class as constructor: `ClassName()`
- Fields initialize to default values or null
- No automatic constructor parameters

**Field Access:**
- Get: `object.field`
- Set: `object.field = value`
- Dynamic: `object[fieldname]` where fieldname is a string

**Method Calls:**
- `object.method(args...)`
- Methods receive implicit `self` parameter
- Static methods called on class: `ClassName.static_method(args...)`

**Example:**
```
class Point {
    var x = 0;
    var y = 0;
    
    fn distance() {
        (self.x ** 2 + self.y ** 2) ** 0.5
    }
}

var p = Point()
p.x = 3
p.y = 4
print(type(p))         // "Point"
print(p.distance())    // 5.0
```

### 3.4. Function Types

Functions are first-class values that encapsulate executable code.

**Properties:**
- Type name: `"function"`
- Includes regular functions, methods, closures, and lambdas
- Can be assigned to variables, passed as arguments, and returned from functions
- Capture variables from enclosing scope (closures)

**Function Categories:**
1. **Regular Functions**: Defined with `fn` at module or class level
2. **Lambda Functions**: Anonymous functions created with `fn(params) { body }`
3. **Methods**: Functions defined within a class (receive implicit `self`)
4. **Bound Methods**: Methods bound to a specific instance
5. **Built-in Functions**: Functions provided by the runtime

**Function Operations:**
- Call: `function(args...)`
- Type check: `type(function) == "function"`
- Equality: Functions are compared by identity

**Example:**
```
fn add(a, b) {
    a + b
}

var f = add
print(type(f))         // "function"
print(f(2, 3))        // 5

// Lambda
var square = fn(x) { x * x }
print(square(4))      // 16

// Closure
fn make_counter() {
    var count = 0
    fn() {
        count = count + 1
        count
    }
}

var counter = make_counter()
print(counter())      // 1
print(counter())      // 2
```

### 3.5. RustValue Type

RustValue is the extension mechanism for implementing custom types in Rust.

**Properties:**
- Type name: Determined by `RustValue::type_name()`
- Enables custom behavior implemented in Rust
- Integrates seamlessly with RustLeaf's type system
- Can have fields and methods like regular objects

**RustValue Interface:**
```rust
trait RustValue {
    fn type_name(&self) -> &str;
    fn get_field(&self, name: &str) -> Option<Value>;
    fn set_field(&mut self, name: &str, value: Value) -> Result<(), String>;
    fn call_method(&self, name: &str, args: Vec<Value>) -> Result<Value, String>;
}
```

**Type Identity:**
- Each RustValue implementation provides its own type name
- Type checking uses the string returned by `type_name()`
- No inheritance or subtyping relationships

**Equality:**
- Default: Reference equality (same Rust object)
- Can be overridden by implementing custom equality in Rust

**Example (from RustLeaf perspective):**
```
// Assuming a Point type implemented in Rust
var p = Point(3.0, 4.0)
print(type(p))           // "Point"
print(p.x)               // 3.0
print(p.distance())      // 5.0

// Type checking
if type(p) == "Point" {
    print("p is a Point")
}
```

### 3.6. Type Conversions

RustLeaf performs no implicit type conversions. All conversions must be explicit.

**No Implicit Conversions:**
- Numeric types are never automatically converted
- No automatic string coercion (except in string interpolation)
- Boolean contexts require actual boolean values

**Explicit Conversion Functions:**
- `int(value)` - Convert to integer
  - From float: truncates toward zero
  - From string: parses decimal integer
  - From bool: true→1, false→0
- `float(value)` - Convert to float
  - From int: exact conversion
  - From string: parses floating-point number
- `str(value)` - Convert to string
  - All types have string representations
  - Used for display and debugging
- `bool(value)` - Convert to boolean
  - Only valid for types with truthiness

**String Interpolation:**
The only implicit conversion occurs in string interpolation, where values are automatically converted to strings:
```
var n = 42
var s = "The answer is ${n}"  // n implicitly converted to "42"
```

**Conversion Errors:**
Invalid conversions raise runtime errors:
```
int("hello")    // Error: Invalid integer format
float("abc")    // Error: Invalid float format
bool([1, 2, 3]) // Error: List has no truthiness
```

### 3.7. Type Checking

Type checking in RustLeaf occurs at runtime when operations are performed.

**Type Checking Mechanisms:**

1. **type() Function**:
   ```
   var t = type(value)  // Returns type name as string
   ```

2. **Direct Comparison**:
   ```
   if type(x) == "int" {
       // x is an integer
   }
   ```

3. **Pattern Matching**:
   ```
   match value {
       case n if type(n) == "int" {
           // Handle integer
       }
       case s if type(s) == "string" {
           // Handle string  
       }
   }
   ```

**Type Errors:**
Operations that receive invalid types raise descriptive errors:
```
"hello" + 42        // Error: Cannot add string and int
[1, 2] * "abc"      // Error: Cannot multiply list and string
null.foo()          // Error: null has no method 'foo'
```

**Error Message Format:**
Type errors include:
- The operation attempted
- The actual type(s) received
- The expected type(s)
- Source location (line, column)

**Duck Typing:**
RustLeaf supports duck typing—if an object has the required methods or fields, operations succeed regardless of its type:
```
class Duck {
    fn quack() { "Quack!" }
}

class Person {
    fn quack() { "I'm quacking!" }
}

fn make_it_quack(thing) {
    print(thing.quack())
}

make_it_quack(Duck())     // "Quack!"
make_it_quack(Person())   // "I'm quacking!"
```

**Truthiness:**
Only `null` and boolean values have truthiness. All other types raise an error in boolean contexts:
```
// Valid
if true { }
if false { }
if null { }          // null is falsy
if not null { }      // true

// Invalid - these raise errors
if 0 { }             // Error: int has no truthiness
if "" { }            // Error: string has no truthiness  
if [] { }            // Error: list has no truthiness

// Must use explicit tests
if x == 0 { }
if s.is_empty() { }
if list.is_empty() { }
```

---

