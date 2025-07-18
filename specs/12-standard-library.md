# 12. Standard Library

RustLeaf provides a rich standard library of methods for builtin types. These methods are available on all instances of their respective types and provide common operations for strings, lists, dictionaries, and objects. This chapter defines all standard library methods, their signatures, behavior, and usage patterns.

### 12.1. String Methods

String methods provide text manipulation and query operations. All string methods return new strings (strings are immutable).

**len() → int**
Returns the length of the string in characters.

```
"hello".len()          // 5
"".len()               // 0
"🚀".len()             // 1 (Unicode character)
```

**split(delimiter=" ") → list**
Splits the string into a list of substrings.

```
"a,b,c".split(",")         // ["a", "b", "c"]
"hello world".split()      // ["hello", "world"] (default: whitespace)
"a::b::c".split("::")      // ["a", "b", "c"]
"abc".split("")            // ["a", "b", "c"] (split each character)
```

**trim() → string**
Removes whitespace from both ends.

```
"  hello  ".trim()         // "hello"
"\n\tworld\n".trim()       // "world"
"no spaces".trim()         // "no spaces"
```

**upper() → string**
Converts to uppercase.

```
"hello".upper()            // "HELLO"
"Hello World".upper()      // "HELLO WORLD"
```

**lower() → string**
Converts to lowercase.

```
"HELLO".lower()            // "hello"
"Hello World".lower()      // "hello world"
```

**replace(old, new) → string**
Replaces all occurrences of old with new.

```
"hello world".replace("world", "universe")  // "hello universe"
"aaa".replace("a", "b")                     // "bbb"
"test".replace("x", "y")                    // "test" (no change)
```

**contains(substring) → bool**
Tests if string contains the substring.

```
"hello world".contains("world")   // true
"hello world".contains("planet")  // false
"test".contains("")               // true (empty string always found)
```

**starts_with(prefix) → bool**
Tests if string starts with the prefix.

```
"hello world".starts_with("hello")  // true
"hello world".starts_with("world")  // false
"test".starts_with("")               // true
```

**ends_with(suffix) → bool**
Tests if string ends with the suffix.

```
"hello world".ends_with("world")    // true
"hello world".ends_with("hello")    // false
"test".ends_with("")                 // true
```

**to_list() → list**
Converts string to list of characters.

```
"hello".to_list()          // ["h", "e", "l", "l", "o"]
"".to_list()               // []
```

### 12.2. List Methods

List methods provide collection manipulation operations. Methods that modify the list return `self` for chaining, while query methods return the requested values.

**len() → int**
Returns the number of elements in the list.

```
[1, 2, 3].len()            // 3
[].len()                   // 0
```

**append(item) → self**
Adds an item to the end of the list.

```
var list = [1, 2]
list.append(3)             // [1, 2, 3]
list.append(4).append(5)   // [1, 2, 3, 4, 5] (chaining)
```

**extend(other) → self**
Adds all items from another iterable to the end.

```
var list = [1, 2]
list.extend([3, 4])        // [1, 2, 3, 4]
list.extend("ab")          // [1, 2, 3, 4, "a", "b"]
```

**insert(index, item) → self**
Inserts an item at the specified index.

```
var list = [1, 3]
list.insert(1, 2)          // [1, 2, 3]
list.insert(0, 0)          // [0, 1, 2, 3]
list.insert(-1, 2.5)       // [0, 1, 2, 2.5, 3] (negative index)
```

**pop(index=-1) → value**
Removes and returns item at index (default: last item).

```
var list = [1, 2, 3]
list.pop()                 // 3, list is now [1, 2]
list.pop(0)                // 1, list is now [2]
list.pop(5)                // Error: Index out of bounds
```

**remove(item) → self**
Removes the first occurrence of item.

```
var list = [1, 2, 3, 2]
list.remove(2)             // [1, 3, 2] (removes first 2)
list.remove(5)             // Error: Item not found
```

**clear() → self**
Removes all items from the list.

```
var list = [1, 2, 3]
list.clear()               // []
```

**map(function) → list**
Returns new list with function applied to each element.

```
[1, 2, 3].map(fn(x) { x * 2 })        // [2, 4, 6]
["a", "b"].map(fn(s) { s.upper() })   // ["A", "B"]
```

**filter(function) → list**
Returns new list with elements where function returns true.

```
[1, 2, 3, 4].filter(fn(x) { x % 2 == 0 })  // [2, 4]
["", "a", ""].filter(fn(s) { s.len() > 0 }) // ["a"]
```

**reduce(function, initial=null) → value**
Reduces list to single value using function.

```
[1, 2, 3, 4].reduce(fn(acc, x) { acc + x }, 0)     // 10
[1, 2, 3].reduce(fn(acc, x) { acc * x }, 1)        // 6
["a", "b", "c"].reduce(fn(acc, x) { acc + x }, "") // "abc"
```

**slice(start, end=null) → list**
Returns new list with elements from start (inclusive) to end (exclusive).

```
[1, 2, 3, 4, 5].slice(1, 4)    // [2, 3, 4]
[1, 2, 3, 4, 5].slice(2)        // [3, 4, 5] (end defaults to length)
[1, 2, 3, 4, 5].slice(-2)       // [4, 5] (negative start)
```

**contains(item) → bool**
Tests if list contains the item.

```
[1, 2, 3].contains(2)      // true
[1, 2, 3].contains(4)      // false
```

**index(item) → int**
Returns index of first occurrence of item.

```
[1, 2, 3, 2].index(2)      // 1
[1, 2, 3].index(4)         // Error: Item not found
```

**reverse() → self**
Reverses the list in place.

```
var list = [1, 2, 3]
list.reverse()             // [3, 2, 1]
```

**sort(key=null) → self**
Sorts the list in place.

```
var list = [3, 1, 2]
list.sort()                           // [1, 2, 3]

var words = ["banana", "apple", "cherry"]
words.sort()                          // ["apple", "banana", "cherry"]

var items = [{a: 3}, {a: 1}, {a: 2}]
items.sort(fn(x) { x.a })            // [{a: 1}, {a: 2}, {a: 3}]
```

**is_empty() → bool**
Tests if list is empty.

```
[].is_empty()              // true
[1].is_empty()             // false
```

**to_dict() → dict**
Converts list of [key, value] pairs to dictionary.

```
[["a", 1], ["b", 2]].to_dict()       // {a: 1, b: 2}
enumerate(["x", "y"]).to_dict()      // {0: "x", 1: "y"}
```

### 12.3. Dict Methods

Dictionary methods provide key-value operations. Methods that modify the dict return `self` for chaining, while query methods return the requested values.

**len() → int**
Returns the number of key-value pairs.

```
{a: 1, b: 2}.len()         // 2
{}.len()                   // 0
```

**keys() → list**
Returns list of all keys in insertion order.

```
{a: 1, b: 2, c: 3}.keys()  // ["a", "b", "c"]
{}.keys()                  // []
```

**values() → list**
Returns list of all values in insertion order.

```
{a: 1, b: 2, c: 3}.values()  // [1, 2, 3]
{}.values()                   // []
```

**items() → list**
Returns list of [key, value] pairs in insertion order.

```
{a: 1, b: 2}.items()       // [["a", 1], ["b", 2]]
{}.items()                 // []
```

**get(key, default=null) → value**
Returns value for key, or default if key not found.

```
{a: 1, b: 2}.get("a")      // 1
{a: 1, b: 2}.get("c")      // null
{a: 1, b: 2}.get("c", 0)   // 0
```

**pop(key, default=null) → value**
Removes and returns value for key.

```
var dict = {a: 1, b: 2}
dict.pop("a")              // 1, dict is now {b: 2}
dict.pop("c")              // null (key not found)
dict.pop("c", "missing")   // "missing"
```

**clear() → self**
Removes all key-value pairs.

```
var dict = {a: 1, b: 2}
dict.clear()               // {}
```

**merge(other) → self**
Merges another dictionary into this one.

```
var dict = {a: 1, b: 2}
dict.merge({b: 3, c: 4})   // {a: 1, b: 3, c: 4}
```

**contains(key) → bool**
Tests if dictionary contains the key.

```
{a: 1, b: 2}.contains("a")  // true
{a: 1, b: 2}.contains("c")  // false
```

**is_empty() → bool**
Tests if dictionary is empty.

```
{}.is_empty()              // true
{a: 1}.is_empty()          // false
```

**to_list() → list**
Converts dictionary to list of [key, value] pairs.

```
{a: 1, b: 2}.to_list()     // [["a", 1], ["b", 2]]
{}.to_list()               // []
```

### 12.4. Object Methods

All objects (including class instances) have these fundamental methods.

**type() → string**
Returns the type name of the object.

```
var obj = MyClass()
obj.type()                 // "MyClass"
42.type()                  // "int"
"hello".type()             // "string"
```

**op_str() → string**
Returns string representation (called by `str()` function).

```
// Default implementation uses type name
var obj = MyClass()
obj.op_str()               // "MyClass()"

// Custom implementation
class Point {
    var x, y;
    fn op_str() { "(${self.x}, ${self.y})" }
}
Point.new(1, 2).op_str()   // "(1, 2)"
```

**op_eq(other) → bool**
Tests equality (called by `==` operator).

```
// Default implementation uses identity
var a = MyClass()
var b = MyClass()
a.op_eq(b)                 // false (different instances)
a.op_eq(a)                 // true (same instance)

// Custom implementation
class Point {
    var x, y;
    fn op_eq(other) {
        type(other) == "Point" and self.x == other.x and self.y == other.y
    }
}
Point.new(1, 2).op_eq(Point.new(1, 2))  // true
```

**has_method(name) → bool**
Tests if object has a method with the given name.

```
"hello".has_method("upper")     // true
"hello".has_method("missing")   // false
[1, 2, 3].has_method("append")  // true
```

**has_field(name) → bool**
Tests if object has a field with the given name.

```
class Person {
    var name;
    var age = 0;
}
var p = Person()
p.has_field("name")        // true
p.has_field("age")         // true
p.has_field("missing")     // false
```

### 12.5. Iterator Protocol

The iterator protocol enables objects to be used in `for` loops and other iteration contexts. Any object can become iterable by implementing the required methods.

**Protocol Methods:**

**op_iter() → iterator**
Returns an iterator object for this collection. Called once at the start of iteration.

**op_next() → value|unit**  
Advances the iterator and returns the next value. Returns unit when iteration is complete.

**Protocol Requirements:**
- `op_iter()` must return an object with an `op_next()` method
- `op_next()` must return unit (not null) when iteration is complete
- Iterators should be stateful - each call to `op_next()` advances position
- Multiple calls to `op_iter()` should return independent iterators

**Built-in Implementations:**

**String Iterator:**
Iterates over individual characters.
```
for ch in "hello" {
    print(ch)  // Prints: h, e, l, l, o
}
```

**List Iterator:**
Iterates over list elements in order.
```
for item in [1, 2, 3] {
    print(item)  // Prints: 1, 2, 3
}
```

**Dict Iterator:**
Iterates over [key, value] pairs.
```
for key, value in {a: 1, b: 2} {
    print("${key} = ${value}")  // Prints: a = 1, b = 2
}
```

**Custom Iterator Example:**
```
class Range {
    var start;
    var end;
    
    static fn new(start, end) {
        var r = Range();
        r.start = start;
        r.end = end;
        r
    }
    
    fn op_iter() {
        var iter = RangeIterator();
        iter.current = self.start;
        iter.end = self.end;
        iter
    }
}

class RangeIterator {
    var current;
    var end;
    
    fn op_next() {
        if self.current >= self.end {
            return;  // Returns unit - iteration complete
        }
        var value = self.current;
        self.current += 1;
        value
    }
}

// Usage
for i in Range.new(0, 5) {
    print(i)  // Prints: 0, 1, 2, 3, 4
}
```

**Iterator Consumption:**
Always check for unit to detect completion:
```
var iter = collection.op_iter();
while true {
    var next = iter.op_next();
    if is_unit(next) {
        break;  // Iteration complete
    }
    process(next);
}
```

### 12.6. Method Resolution and Inheritance

**Method Call Resolution:**
1. Look for method on object's class
2. Look for method on builtin type (if applicable)
3. Raise error if method not found

**Method Chaining:**
Methods that modify objects return `self` to enable chaining:
```
var list = []
list.append(1).append(2).extend([3, 4]).reverse()  // [4, 3, 2, 1]

var dict = {}
dict.merge({a: 1}).merge({b: 2}).clear()           // {}
```

**Error Handling:**
Standard library methods provide clear error messages:
```
[].pop()
// Error: Cannot pop from empty list
//   at List.pop() (standard library)
//   at main.rustleaf:3:5

{}.pop("missing")
// Error: Key 'missing' not found in dictionary
//   at Dict.pop() (standard library)
//   at main.rustleaf:5:8
```

