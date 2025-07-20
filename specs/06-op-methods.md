# 6. Operator Methods

RustLeaf unifies expression evaluation through a comprehensive operator method system. Instead of having special cases for different types of operations, nearly all syntax desugars to method calls on the left operand. This chapter defines the complete operator method system and how it simplifies the language implementation.

### 6.1. Overview

The operator method system transforms syntax into uniform method calls, allowing types to customize behavior while simplifying the evaluator implementation.

**Core Principle:**
Most RustLeaf syntax desugars to method calls at parse time, creating a uniform AST structure where the evaluator primarily performs method dispatch.

**Benefits:**
- **Simplified parser/evaluator**: Fewer special cases, more uniform handling
- **Extensible behavior**: Types can customize operations by implementing methods
- **Consistent semantics**: All operations follow the same dispatch rules
- **Performance opportunities**: Uniform structure enables optimization

**Example:**
```
// Syntax                    Desugars to
user.name                 → user.op_get_attr("name")
user.name = "Alice"       → user.op_set_attr("name", "Alice")
numbers[0]                → numbers.op_get_item(0)
point + vector            → point.op_get_attr("op_add").op_call(vector)
calculate()               → calculate.op_call()
```

### 6.2. Bootstrap Model

The operator method system requires exactly one special method to avoid infinite recursion: `op_get_attr`.

**The Bootstrap Method:**
`op_get_attr(name: string) -> value` is the only method resolved through direct/intrinsic lookup rather than through the operator method system itself.

**Recursion Breaking:**
```
a.foo                     → a.op_get_attr("foo")         # Direct lookup
a.op_get_attr             → a.op_get_attr("op_get_attr") # Would recurse!
```

To break this recursion, `op_get_attr` itself is resolved through a separate mechanism:
- For built-in types: intrinsic implementation provided by the runtime
- For user classes: direct method table lookup 
- For RustValues: the `get_attr` trait method

**All Other Operations Flow Through Bootstrap:**
Once `op_get_attr` is resolved, all other operator methods are looked up through it:
```
a + b    → a.op_get_attr("op_add").op_call(b)
a[b]     → a.op_get_attr("op_get_item").op_call(b)
a()      → a.op_get_attr("op_call").op_call()
```

### 6.3. Desugaring Rules

This section defines the complete mapping from syntax to operator method calls.

#### 6.3.1. Attribute Access

**Property Access:**
```
a.property              → a.op_get_attr("property")
a.property = value      → a.op_set_attr("property", value)
```

**Method Calls:**
```
a.method()              → a.op_call_method("method")
a.method(x, y)          → a.op_call_method("method", x, y)
```

**Dynamic Access:**
```
a[property_name]        → a.op_get_item(property_name)  # when used for properties
a[property_name] = val  → a.op_set_item(property_name, val)
```

#### 6.3.2. Function Calls

**Direct Calls:**
```
func()                  → func.op_call()
func(x, y)              → func.op_call(x, y)
```


#### 6.3.3. Indexing Operations

**Item Access:**
```
container[key]          → container.op_get_item(key)
container[key] = value  → container.op_set_item(key, value)
```


#### 6.3.4. Arithmetic Operations

**Binary Arithmetic:**
```
a + b                   → a.op_get_attr("op_add").op_call(b)
a - b                   → a.op_get_attr("op_sub").op_call(b)
a * b                   → a.op_get_attr("op_mul").op_call(b)
a / b                   → a.op_get_attr("op_div").op_call(b)
a % b                   → a.op_get_attr("op_mod").op_call(b)
a ** b                  → a.op_get_attr("op_pow").op_call(b)
```

**Unary Arithmetic:**
```
-a                      → a.op_get_attr("op_neg").op_call()
```

**Augmenting Assignment:**
```
a += b                  → a = a.op_get_attr("op_add").op_call(b)
a -= b                  → a = a.op_get_attr("op_sub").op_call(b)
a *= b                  → a = a.op_get_attr("op_mul").op_call(b)
a /= b                  → a = a.op_get_attr("op_div").op_call(b)
a %= b                  → a = a.op_get_attr("op_mod").op_call(b)
a **= b                 → a = a.op_get_attr("op_pow").op_call(b)
```

#### 6.3.5. Comparison Operations

**Binary Comparison:**
```
a == b                  → a.op_get_attr("op_eq").op_call(b)
a != b                  → a.op_get_attr("op_ne").op_call(b)
a < b                   → a.op_get_attr("op_lt").op_call(b)
a > b                   → a.op_get_attr("op_gt").op_call(b)
a <= b                  → a.op_get_attr("op_le").op_call(b)
a >= b                  → a.op_get_attr("op_ge").op_call(b)
```

#### 6.3.6. Bitwise Operations

**Binary Bitwise:**
```
a & b                   → a.op_get_attr("op_bitwise_and").op_call(b)
a | b                   → a.op_get_attr("op_bitwise_or").op_call(b)
a ^ b                   → a.op_get_attr("op_bitwise_xor").op_call(b)
a << b                  → a.op_get_attr("op_lshift").op_call(b)
a >> b                  → a.op_get_attr("op_rshift").op_call(b)
```

**Unary Bitwise:**
```
~a                      → a.op_get_attr("op_bitwise_not").op_call()
```

**Bitwise Augmenting Assignment:**
```
a &= b                  → a = a.op_get_attr("op_bitwise_and").op_call(b)
a |= b                  → a = a.op_get_attr("op_bitwise_or").op_call(b)
a ^= b                  → a = a.op_get_attr("op_bitwise_xor").op_call(b)
a <<= b                 → a = a.op_get_attr("op_lshift").op_call(b)
a >>= b                 → a = a.op_get_attr("op_rshift").op_call(b)
```

#### 6.3.7. Utility Operations

**Type and String Conversion:**
```
str(a)                  → a.op_get_attr("op_str").op_call()
```

**Membership Testing:**
```
item in container       → container.op_get_attr("op_contains").op_call(item)
```

#### 6.3.8. Iterator Protocol

**Iteration Setup:**
```
for item in iterable    → iterator = iterable.op_get_attr("op_iter").op_call()
                          # Built-in iteration logic using iterator.op_next()
```

**Iterator Operations:**
```
next_value = iter.op_next()  # Called by built-in iteration logic
```

### 6.4. Built-in Operations

Some operations remain built-in to the evaluator and cannot be overridden, either for semantic reasons or implementation simplicity.

#### 6.4.1. Logical Operations

Logical operations remain built-in because they have specific truthiness requirements and short-circuit evaluation:

**Logical Operations (Built-in):**
```
a and b                 # Built-in truthiness check + short-circuit evaluation
a or b                  # Built-in truthiness check + short-circuit evaluation  
a xor b                 # Built-in truthiness check + full evaluation
not a                   # Built-in truthiness inversion
```

**Rationale:**
- Only `null` and `bool` have truthiness in RustLeaf
- Short-circuit evaluation requires special control flow
- These operations are fundamental to the language's logic system

#### 6.4.2. Control Flow

Control flow constructs remain built-in:
```
if condition { ... }    # Built-in condition evaluation
while condition { ... } # Built-in loop control
for item in iter { ... } # Built-in iteration (though uses op_iter/op_next)
match value { ... }     # Built-in pattern matching
```

#### 6.4.3. Type Functions

Core type functions remain built-in:
```
type(value)             # Built-in type identification
is_unit(value)          # Built-in unit testing
int(value)              # Built-in conversion (for now)
float(value)            # Built-in conversion (for now)
bool(value)             # Built-in conversion (for now)
```

### 6.5. Implementation Requirements

This section defines what each type category must provide to participate in the operator method system.

#### 6.5.1. All Types

**Required Bootstrap Method:**
Every type must provide `op_get_attr(name: string) -> value` through its intrinsic method resolution mechanism.

**Error Handling:**
When `op_get_attr` is called with an unknown method name, it raises a descriptive error:
```
object.unknown_method()
# Error: No method 'unknown_method' on type 'ClassName'
```

#### 6.5.2. Built-in Types

Built-in types (int, float, string, list, dict, bool, null, unit) have intrinsic implementations of relevant operator methods.

**Example - String Type:**
```
"hello".op_get_attr("op_add")     # Returns built-in string concatenation method
"hello".op_get_attr("op_mul")     # Returns built-in string repetition method
"hello".op_get_attr("op_str")     # Returns identity function
"hello".op_get_attr("op_len")     # Returns built-in length function
"hello".op_get_attr("op_iter")    # Returns built-in string iterator
```

**Example - List Type:**
```
[1,2,3].op_get_attr("op_get_item") # Returns built-in list indexing method
[1,2,3].op_get_attr("op_set_item") # Returns built-in list assignment method
[1,2,3].op_get_attr("op_add")      # Returns built-in list concatenation method
[1,2,3].op_get_attr("op_contains") # Returns built-in membership test method
```

#### 6.5.3. User-Defined Classes

User-defined classes automatically get basic `op_get_attr` implementation that:
1. Looks up methods defined in the class
2. Looks up fields defined in the class
3. Raises an error for unknown attributes

**Basic Implementation:**
```
class Point {
    var x = 0;
    var y = 0;
    
    fn distance() {
        (self.x ** 2 + self.y ** 2) ** 0.5
    }
    
    fn op_add(other) {
        var result = Point();
        result.x = self.x + other.x;
        result.y = self.y + other.y;
        result
    }
}

// Automatic op_get_attr behavior:
var p = Point();
p.op_get_attr("x")        # Returns field value: 0
p.op_get_attr("distance") # Returns bound method
p.op_get_attr("op_add")   # Returns bound method for custom addition
```

#### 6.5.4. RustValue Types

RustValue types implement the operator method system through their existing `get_attr` trait method, which serves as `op_get_attr`.

**RustValue Integration:**
```rust
impl RustValue for CustomType {
    fn get_attr(&self, name: &str) -> Option<Value> {
        match name {
            "op_add" => Some(/* return method object */),
            "op_str" => Some(/* return method object */),
            "field_name" => Some(/* return field value */),
            _ => None
        }
    }
}
```

### 6.6. Examples

This section provides practical examples of the operator method system in action.

#### 6.6.1. Custom Vector Class

```
class Vector {
    var x;
    var y;
    
    static fn new(x, y) {
        var v = Vector();
        v.x = x;
        v.y = y;
        v
    }
    
    fn op_add(other) {
        Vector.new(self.x + other.x, self.y + other.y)
    }
    
    fn op_mul(scalar) {
        Vector.new(self.x * scalar, self.y * scalar)
    }
    
    fn op_str() {
        "Vector(${self.x}, ${self.y})"
    }
    
    fn op_eq(other) {
        self.x == other.x and self.y == other.y
    }
}

// Usage examples:
var v1 = Vector.new(1, 2);
var v2 = Vector.new(3, 4);

// These desugar to operator method calls:
var sum = v1 + v2;          # v1.op_get_attr("op_add").op_call(v2)
var scaled = v1 * 3;        # v1.op_get_attr("op_mul").op_call(3)
var display = str(v1);      # v1.op_get_attr("op_str").op_call()
var equal = v1 == v2;       # v1.op_get_attr("op_eq").op_call(v2)
```

#### 6.6.2. Property Access Chain

```
class User {
    var profile;
    
    fn op_get_attr(name) {
        match name {
            case "name" {
                if self.profile != null {
                    self.profile.name
                } else {
                    "Anonymous"
                }
            }
            case _ {
                # Default attribute lookup
                # (implementation-specific)
            }
        }
    }
}

var user = User();
var name = user.name;  # user.op_get_attr("name") - custom logic
```

#### 6.6.3. Method Call Optimization

```
class Calculator {
    var value = 0;
    
    fn add(n) {
        self.value += n;
        self  # Return self for chaining
    }
    
    fn multiply(n) {
        self.value *= n;
        self  # Return self for chaining
    }
    
    fn op_call_method(method_name, *args) {
        # Optimized method dispatch
        match method_name {
            case "add" { self.add(*args) }
            case "multiply" { self.multiply(*args) }
            case _ { 
                # Fall back to normal method lookup
                self.op_get_attr(method_name).op_call(*args)
            }
        }
    }
}

var calc = Calculator();
calc.add(5).multiply(3);  # Optimized method chaining
```

#### 6.6.4. Container Implementation

```
class Dict {
    var data;
    
    fn new() {
        var d = Dict();
        d.data = {};  # Use built-in dict for storage
        d
    }
    
    fn op_get_item(key) {
        if key in self.data {
            self.data[key]
        } else {
            raise("Key not found: ${key}")
        }
    }
    
    fn op_set_item(key, value) {
        self.data[key] = value
    }
    
    fn op_contains(key) {
        key in self.data
    }
    
    fn op_iter() {
        self.data.op_iter()  # Delegate to built-in dict iterator
    }
}

var custom_dict = Dict.new();
custom_dict["key"] = "value";     # custom_dict.op_set_item("key", "value")
var value = custom_dict["key"];   # custom_dict.op_get_item("key")
var has_key = "key" in custom_dict; # custom_dict.op_contains("key")
```

---