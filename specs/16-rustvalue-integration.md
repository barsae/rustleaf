# 16. RustValue Integration

RustLeaf provides seamless integration with custom types implemented in Rust through the RustValue system. This enables extending the language with high-performance, native functionality while maintaining type safety and consistent behavior. This chapter defines how RustValues behave from RustLeaf code perspective.

### 16.1. RustValue Trait

RustValues are custom types implemented in Rust that integrate seamlessly with RustLeaf's type system.

**Core Capabilities:**
RustValues provide a minimal, generic interface for attribute access and type identification.

```
// RustValue types behave like regular RustLeaf objects
var point = Point.new(3.0, 4.0)  // Point is a RustValue type

// Field access works transparently
print(point.x)           // 3.0
print(point.y)           // 4.0

// Method calls work transparently  
print(point.distance())  // 5.0

// Type identification
print(type(point))       // "Point"
```

**Attribute Resolution:**
RustValues handle all attribute access through a generic resolution system.

```
// Field access
var value = custom_object.some_field    // Calls get_attr("some_field")
custom_object.some_field = new_value    // Calls set_attr("some_field", new_value)

// Method access
var method = custom_object.some_method  // Calls get_attr("some_method") 
var result = custom_object.some_method(args)  // Method call

// Dynamic access
var field_name = "dynamic_field"
var value = custom_object[field_name]   // Dynamic attribute access
```

**Error Handling:**
RustValue operations integrate with RustLeaf's error system.

```
var obj = CustomType.new()

try {
    var value = obj.nonexistent_field
} catch e {
    print("Error: ${e}")  // "Error: Unknown attribute: nonexistent_field"
}

try {
    obj.read_only_field = "new value"  
} catch e {
    print("Error: ${e}")  // "Error: Cannot modify read-only field: read_only_field"
}
```

### 16.2. Value Enum

RustValues are integrated into RustLeaf's value system as first-class citizens.

**Type Integration:**
RustValues work seamlessly with all RustLeaf operations.

```
// RustValues in collections
var objects = [Point.new(1, 2), Point.new(3, 4), Point.new(5, 6)]
var distances = objects.map(fn(p) { p.distance() })

// RustValues in dictionaries
var registry = {
    "origin": Point.new(0, 0),
    "unit_x": Point.new(1, 0),
    "unit_y": Point.new(0, 1)
}

// RustValues as function parameters
fn process_point(point) {
    print("Processing point at (${point.x}, ${point.y})")
    point.distance()
}

process_point(Point.new(2, 3))
```

**Memory Semantics:**
RustValues follow reference semantics like other RustLeaf objects.

```
var p1 = Point.new(1, 2)
var p2 = p1          // p2 references the same RustValue

// Mutation affects both references (if the RustValue allows mutation)
p1.move_to(5, 6)     // Hypothetical mutating method
print(p2.x)          // 5 (sees the change if RustValue is mutable)

// Immutable RustValues work like expected
var config = Config.load("app.json")
var backup = config  // Both reference the same immutable config
```

**Garbage Collection:**
RustValues are managed by the garbage collector like other values.

```
fn create_temporary() {
    var temp = LargeDataStructure.new()
    temp.process()
    // temp becomes unreachable when function exits
    // GC will clean up the RustValue
}

create_temporary()
// LargeDataStructure is eligible for collection
```

### 16.3. Field Access

RustValues can expose fields with configurable access patterns.

**Read-Only Fields:**
Some fields may be read-only to maintain invariants.

```
var file = File.open("data.txt")
print(file.path)         // "/path/to/data.txt" (read-only)
print(file.size)         // 1024 (read-only, computed)

// file.path = "new.txt"  // Error: Cannot modify read-only field
```

**Read-Write Fields:**
Other fields may allow modification.

```
var config = Config.new()
config.timeout = 30      // OK - read-write field
config.host = "localhost" // OK - read-write field

print(config.timeout)    // 30
print(config.host)       // "localhost"
```

**Computed Fields:**
Fields can be computed dynamically.

```
var rect = Rectangle.new(10, 20)
print(rect.width)        // 10 (stored field)
print(rect.height)       // 20 (stored field)  
print(rect.area)         // 200 (computed: width * height)
print(rect.perimeter)    // 60 (computed: 2 * (width + height))
```

**Field Validation:**
Field assignments can include validation.

```
var person = Person.new()
person.age = 25          // OK
person.age = -5          // Error: Age cannot be negative
person.email = "invalid" // Error: Invalid email format
```

### 16.4. Method Dispatch

RustValues can provide methods that integrate with RustLeaf's method system.

**Native Methods:**
RustValues can expose Rust-implemented methods.

```
var vector = Vector3.new(1.0, 2.0, 3.0)
var length = vector.magnitude()      // Native Rust method
var normalized = vector.normalize()  // Returns new Vector3
var dot_product = vector.dot(other_vector)
```

**Method Overriding:**
RustValue methods can override built-in methods for custom behavior.

```
// Custom string representation
var point = Point.new(3, 4)
print(str(point))        // "(3, 4)" - custom op_str implementation

// Custom equality
var p1 = Point.new(1, 2)
var p2 = Point.new(1, 2)
print(p1 == p2)          // true - custom op_eq implementation

// Custom arithmetic
var v1 = Vector.new(1, 2)
var v2 = Vector.new(3, 4)
var sum = v1 + v2        // Vector(4, 6) - custom op_add implementation
```

**Method Chaining:**
RustValue methods support chaining where appropriate.

```
var text = TextProcessor.new()
var result = text
    .load_file("input.txt")
    .remove_comments()
    .normalize_whitespace()  
    .save_to("output.txt")
```

**Variadic Methods:**
RustValue methods can accept variable arguments.

```
var logger = Logger.new()
logger.log("info", "User logged in")
logger.log("error", "Database error", error_code: 500, retry: true)
```

### 16.5. Type Coercion

RustValues use explicit conversion rather than automatic coercion.

**Explicit Conversion:**
RustValues convert to primitives through explicit methods or operators.

```
var number_wrapper = NumberWrapper.new(42)

// Explicit conversion required
var as_int = int(number_wrapper)     // 42 (if conversion supported)
var as_str = str(number_wrapper)     // "NumberWrapper(42)"
var as_float = float(number_wrapper) // 42.0 (if conversion supported)

// No automatic coercion
// var result = number_wrapper + 5   // Error: Cannot add NumberWrapper and int
var result = int(number_wrapper) + 5 // 47 (explicit conversion)
```

**Collection Conversion:**
RustValues can convert to standard collections.

```
var rust_list = RustList.new([1, 2, 3, 4, 5])
var rustleaf_list = list(rust_list)  // [1, 2, 3, 4, 5]

var rust_map = RustMap.new()
rust_map.insert("a", 1)
rust_map.insert("b", 2)
var rustleaf_dict = dict(rust_map)   // {a: 1, b: 2}
```

**Truthiness:**
RustValues follow strict truthiness rules like other types.

```
var custom_obj = CustomType.new()

// Only null and bool have truthiness
// if custom_obj { }              // Error: Only null and bool have truthiness
if custom_obj != null { }         // OK - explicit null check
if custom_obj.is_valid() { }      // OK - explicit method call
```

### 16.6. Lifetime Management

RustValues are managed consistently with RustLeaf's memory model.

**Automatic Management:**
RustValues are automatically managed by the garbage collector.

```
fn process_data() {
    var processor = DataProcessor.new()
    processor.load_large_dataset()
    var results = processor.analyze()
    results  // Return results, processor becomes unreachable
}

var analysis = process_data()
// DataProcessor is cleaned up automatically
// Results remain accessible
```

**Resource Cleanup:**
RustValues that manage resources should support the cleanup protocol.

```
// RustValues can implement close() for resource cleanup
with connection = DatabaseConnection.connect("localhost") {
    var data = connection.query("SELECT * FROM users")
    process_data(data)
}  // connection.close() called automatically

// Custom resources
with temp_file = TempFile.create() {
    temp_file.write("temporary data")
    var content = temp_file.read()
}  // temp_file cleanup removes the file
```

**Shared Ownership:**
Multiple RustLeaf variables can reference the same RustValue.

```
var original = SharedResource.new()
var reference1 = original
var reference2 = original

// All variables reference the same RustValue
original.modify_state()
print(reference1.get_state())  // Sees the modification
print(reference2.get_state())  // Sees the modification
```

**Integration Examples:**

**File System Integration:**
```
var file = File.open("config.json")
print(file.path)          // "/path/to/config.json"
print(file.size)          // 1024
print(file.exists)        // true

var content = file.read()
file.write("new content")
file.close()
```

**Network Integration:**
```
var client = HttpClient.new()
client.timeout = 30
client.headers.set("User-Agent", "RustLeaf/1.0")

var response = client.get("https://api.example.com/data")
print(response.status)    // 200
print(response.body)      // Response content
```

**Data Processing Integration:**
```
var parser = JsonParser.new()
var data = parser.parse(json_string)

// RustValue seamlessly works with RustLeaf collections
for item in data.items {
    print("${item.name}: ${item.value}")
}

var filtered = data.items.filter(fn(item) { item.active })
```

**Performance Integration:**
```
var matrix = Matrix.new(1000, 1000)
matrix.fill_random()

// Native performance for compute-heavy operations
var determinant = matrix.determinant()    // Fast native computation
var inverse = matrix.invert()             // Fast native computation

// Seamless integration with RustLeaf
var matrices = [matrix, inverse]
var results = matrices.map(fn(m) { m.trace() })
```

