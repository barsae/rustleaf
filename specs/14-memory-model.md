# 14. Memory Model

RustLeaf provides automatic memory management with predictable semantics for value copying, reference sharing, and garbage collection. This chapter defines how values are stored, copied, and shared in memory, ensuring memory safety while maintaining performance and predictability.

### 14.1. Value Semantics

RustLeaf uses different semantics for different types to balance performance and intuitive behavior.

**Primitive Value Copying:**
Primitive types are copied by value on assignment and parameter passing.

```
// Primitives are copied
var a = 42
var b = a        // b gets a copy of a's value
a = 100          // a changes, b remains 42
print(b)         // 42

// String copying (strings are immutable)
var str1 = "hello"
var str2 = str1  // str2 gets a copy of the string
str1 = "world"   // str1 points to new string, str2 unchanged
print(str2)      // "hello"

// Boolean and null copying
var flag1 = true
var flag2 = flag1  // Copy
flag1 = false      // flag2 remains true
```

**Primitive Types (Copy Semantics):**
- `int` - Integer values
- `float` - Floating-point values  
- `bool` - Boolean values
- `null` - Null value
- `string` - Immutable strings (copy-on-write)

### 14.2. Reference Semantics

Collections and objects use reference semantics for efficiency and expected behavior.

**Collection Reference Sharing:**
Lists and dictionaries are passed by reference.

```
// Lists are shared by reference
var list1 = [1, 2, 3]
var list2 = list1    // list2 references the same list
list1.append(4)      // Modifies the shared list
print(list2)         // [1, 2, 3, 4] - sees the change

// Dictionaries are shared by reference
var dict1 = {a: 1, b: 2}
var dict2 = dict1    // dict2 references the same dict
dict1.merge({c: 3})  // Modifies the shared dict
print(dict2)         // {a: 1, b: 2, c: 3} - sees the change

// Objects are shared by reference
class Point {
    var x, y;
    static fn new(x, y) {
        var p = Point()
        p.x = x
        p.y = y
        p
    }
}

var p1 = Point.new(1, 2)
var p2 = p1          // p2 references the same object
p1.x = 10            // Modifies the shared object
print(p2.x)          // 10 - sees the change
```

**Reference Types:**
- `list` - Mutable sequences
- `dict` - Mutable key-value mappings
- `function` - Function objects
- `object` - Class instances
- `RustValue` - Custom Rust-implemented types

**Function Parameter Passing:**
```
fn modify_primitive(x) {
    x = 999          // Only modifies local copy
}

fn modify_collection(list) {
    list.append(4)   // Modifies the shared list
}

var num = 42
var arr = [1, 2, 3]

modify_primitive(num)    // num remains 42
modify_collection(arr)   // arr becomes [1, 2, 3, 4]

print(num)  // 42
print(arr)  // [1, 2, 3, 4]
```

### 14.3. Garbage Collection

RustLeaf provides automatic memory management with garbage collection to prevent memory leaks and ensure memory safety.

**Automatic Collection:**
Memory is automatically reclaimed when values are no longer reachable.

```
fn create_data() {
    var large_list = range(0, 10000)  // Allocates memory
    large_list.map(fn(x) { x * x })   // More allocation
    // large_list becomes unreachable when function returns
    // Memory is automatically reclaimed by GC
}

create_data()
// All memory from create_data() is eligible for collection
```

**Circular Reference Handling:**
The garbage collector automatically handles circular references.

```
// Create circular references
var node1 = {value: 1, next: null}
var node2 = {value: 2, next: node1}
node1.next = node2  // Circular reference: node1 -> node2 -> node1

// Later, when nodes go out of scope
node1 = null
node2 = null
// GC will collect both nodes despite the cycle
```

**Collection Triggers:**
- Automatic collection during memory pressure
- Collection when heap size thresholds are exceeded
- Collection during idle periods
- No manual collection interface (fully automatic)

**Memory Safety Guarantees:**
- No use-after-free errors
- No memory leaks from unreachable objects
- No null pointer dereferences
- Automatic cleanup of circular references

### 14.4. Resource Management

Resources beyond memory require explicit management using structured cleanup patterns.

**Automatic Resource Cleanup:**
The `with` statement provides deterministic resource cleanup.

```
// File resources are cleaned up automatically
with file = open("data.txt") {
    var content = file.read()
    process_content(content)
}  // file.close() called automatically, even if error occurs

// Multiple resources cleaned up in reverse order
with db = connect_database(), cache = connect_cache() {
    var data = db.query("SELECT * FROM users")
    cache.store(data)
}  // cache.close() then db.close() called automatically
```

**Resource Cleanup Protocol:**
Objects can implement cleanup behavior:

```
class DatabaseConnection {
    var connection_handle;
    
    fn close() {
        if self.connection_handle != null {
            release_connection(self.connection_handle)
            self.connection_handle = null
        }
    }
}

// Used with 'with' statement
with conn = DatabaseConnection.new("localhost") {
    conn.execute("INSERT INTO logs VALUES ('action')")
}  // conn.close() called automatically
```

**Error Safety:**
Resource cleanup occurs even when errors are raised:

```
with file = open("output.txt") {
    try {
        dangerous_operation(file)
    } catch e {
        print("Error occurred: ${e}")
        raise(e)  // Re-raise the error
    }
}  // file.close() still called despite the error
```

**Manual Resource Management:**
Note: RustLeaf does not have finally blocks. Use `with` statements for cleanup:

```
// Preferred approach
with connection = create_connection() {
    use_connection(connection)
}  // connection.close() called automatically

// Or manual cleanup with explicit calls
var connection = create_connection()
try {
    use_connection(connection)
} catch e {
    connection.close()  // Manual cleanup
    raise(e)
}
connection.close()  // Manual cleanup on success
```

**Resource Lifetime Rules:**
1. Resources should be acquired as late as possible
2. Resources should be released as early as possible  
3. Use `with` statements for automatic cleanup
4. Implement `close()` methods for custom resources
5. Resources are not garbage collected - they require explicit cleanup

**Best Practices:**
```
// Good: Use with statement
with file = open("data.txt") {
    process_file(file)
}

// Avoid: Manual resource management
var file = open("data.txt")
process_file(file)
file.close()  // Easy to forget, won't happen on errors

// Good: Custom resource with cleanup
class TempDirectory {
    var path;
    
    static fn new(prefix) {
        var temp = TempDirectory()
        temp.path = create_temp_dir(prefix)
        temp
    }
    
    fn close() {
        remove_directory(self.path)
    }
}

with temp = TempDirectory.new("work") {
    create_files_in(temp.path)
}  // Directory automatically removed
```

**Memory vs. Resource Distinction:**
- **Memory**: Managed automatically by garbage collection
- **Resources**: Require explicit management with deterministic cleanup
- **Files, network connections, locks**: Always resources, never just memory
- **Large data structures**: Memory, cleaned up by GC
- **Temporary objects**: Memory, cleaned up by GC

