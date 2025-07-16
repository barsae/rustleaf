# 10. Modules

RustLeaf provides a module system for organizing code into reusable components. Modules enable namespace management, visibility control, and code separation. This chapter defines module declaration, import/export semantics, path resolution, and dependency management.

### 10.1. Module System Overview

**Design Principles:**
- File-based modules (one module per file)
- Explicit visibility with `pub` keyword
- Path-based import system with `use` statements
- Runtime circular dependency detection
- No module caching (modules imported fresh each time)

**Module Structure:**
```
project/
├── main.rustleaf           // Entry point
├── utils.rustleaf          // Module: utils
├── math/
│   ├── geometry.rustleaf   // Module: math::geometry
│   └── algebra.rustleaf    // Module: math::algebra
└── graphics/
    ├── shapes/
    │   └── circle.rustleaf // Module: graphics::shapes::circle
    └── colors.rustleaf     // Module: graphics::colors
```

### 10.2. Import Statements

Import statements use the `use` keyword to bring items from other modules into scope.

**Basic Import Syntax:**
```
use module_path;                    // Import module
use module_path::item;              // Import specific item
use module_path::{item1, item2};    // Import multiple items
use module_path::*;                 // Import all public items (discouraged)
```

**Path Resolution:**
- **Parent paths**: `use super::sibling_module` (parent directory)
- **Root paths**: `use root::top_level_module` (project root, explicit)
- **Absolute paths**: `use math::geometry` (from project root, default)

**Import Examples:**
```
// File: graphics/renderer.rustleaf

// Import from project root
use math::geometry::Point;
use math::algebra::{Vector, Matrix};

// Import from parent directory
use super::colors::{RED, GREEN, BLUE};

// Import from same directory (just use the name)
use effects::Shader;

// Import from root explicitly
use root::utils::Logger;

// Multiple imports
use math::geometry::{Point, Line, Circle};

// Import all (discouraged)
use math::constants::*;
```

### 10.3. Export Functions

Exports use the `pub` keyword to make items visible outside the current module.

**Visibility Levels:**
- **Private** (default): Only accessible within the current module
- **Public** (`pub`): Accessible from other modules that import this module

**Export Syntax:**
```
// Public function
pub fn public_function() { }

// Private function (default)
fn private_function() { }

// Public variable
pub var PUBLIC_CONSTANT = 42;

// Private variable (default)
var private_data = [];

// Public class
pub class PublicClass {
    pub var public_field;     // Public field
    var private_field;        // Private field
    
    pub fn public_method() { } // Public method
    fn private_method() { }    // Private method
}

// Private class (default)
class PrivateClass { }
```

**Module Example:**
```
// File: math/geometry.rustleaf

// Public exports - available to importers
pub class Point {
    pub var x;
    pub var y;
    
    pub static fn new(x, y) {
        var p = Point()
        p.x = x
        p.y = y
        p
    }
    
    pub fn distance_to(other) {
        var dx = self.x - other.x
        var dy = self.y - other.y
        sqrt(dx * dx + dy * dy)
    }
    
    // Private method - only accessible within this module
    fn validate() {
        if type(self.x) != "float" or type(self.y) != "float" {
            raise("Point coordinates must be numbers")
        }
    }
}

pub fn distance(p1, p2) {
    p1.distance_to(p2)
}

pub var ORIGIN = Point.new(0.0, 0.0);

// Private helper - not accessible from other modules
fn sqrt(x) {
    x ** 0.5
}

var cache = {};  // Private module variable
```

**Usage:**
```
// File: main.rustleaf
use math::geometry::{Point, distance, ORIGIN};

var p1 = Point.new(3.0, 4.0)
var p2 = Point.new(0.0, 0.0)

print(distance(p1, ORIGIN))     // OK - public function
print(p1.distance_to(p2))       // OK - public method
// print(p1.validate())         // Error - private method
// var c = cache                // Error - private variable
```

### 10.4. Module Resolution

Module paths are resolved based on the filesystem structure and special keywords.

**Resolution Rules:**
1. **Absolute paths** start from project root (default)
2. **Parent paths** start with `super::` (parent directory)
3. **Root paths** start with `root::` (project root, explicit)

**Path Resolution Algorithm:**
```
use path::to::module
     ↓
1. If path starts with "super::" → resolve relative to parent directory
2. If path starts with "root::" → resolve from project root
3. Otherwise → resolve from project root (absolute)
```

**Examples:**
```
// Project structure:
// src/
// ├── main.rustleaf
// ├── utils.rustleaf
// ├── graphics/
// │   ├── renderer.rustleaf
// │   └── shapes/
// │       └── circle.rustleaf
// └── math/
//     └── geometry.rustleaf

// File: src/graphics/renderer.rustleaf

use utils                          // → src/utils.rustleaf
use math::geometry                 // → src/math/geometry.rustleaf  
use shapes::circle                 // → src/graphics/shapes/circle.rustleaf
use super::utils                   // → src/utils.rustleaf
use root::math::geometry           // → src/math/geometry.rustleaf
```

**Module File Mapping:**
- `use utils` → `utils.rustleaf`
- `use math::geometry` → `math/geometry.rustleaf`
- `use graphics::shapes::circle` → `graphics/shapes/circle.rustleaf`

### 10.5. Module Loading

Modules are loaded and executed when first imported.

**Loading Behavior:**
- Each module file is executed once when first imported
- Module-level code runs during import (initialization)
- No caching - modules are re-loaded on each import
- Circular dependencies cause runtime errors

**Module Initialization:**
```
// File: database.rustleaf

// Module-level initialization code
print("Loading database module...")

var connection_pool = [];
var default_config = {
    host: "localhost",
    port: 5432,
    timeout: 30
};

// Initialize connection pool
for i in range(0, 10) {
    connection_pool.append(create_connection(default_config))
}

print("Database module loaded with ${connection_pool.length} connections")

// Public API
pub fn get_connection() {
    if connection_pool.length > 0 {
        connection_pool.pop()
    } else {
        create_connection(default_config)
    }
}

pub fn release_connection(conn) {
    connection_pool.append(conn)
}

// Private helper
fn create_connection(config) {
    // Implementation details...
    {host: config.host, port: config.port, active: true}
}
```

**Import Execution:**
```
// File: main.rustleaf
print("Starting application...")

use database  // This triggers database.rustleaf execution
// Output: "Loading database module..."
//         "Database module loaded with 10 connections"

var conn = database.get_connection()
print("Got connection: ${conn}")
```

### 10.6. Module Scope

Each module has its own scope separate from other modules.

**Scope Rules:**
- Module-level variables are private by default
- Only `pub` items are accessible from other modules
- Imported items are available in the importing module's scope
- No global namespace pollution

**Scope Isolation:**
```
// File: module_a.rustleaf
var private_var = "A's private data"
pub var public_var = "A's public data"

pub fn get_private() {
    private_var  // OK - same module
}

// File: module_b.rustleaf  
var private_var = "B's private data"  // Different from A's private_var
pub var public_var = "B's public data"

use module_a

pub fn test() {
    print(module_a.public_var)     // OK - public
    print(module_a.get_private())  // OK - returns A's private data
    // print(module_a.private_var) // Error - not public
    
    print(private_var)             // "B's private data" - local scope
}
```

**Import Scope:**
```
// File: graphics.rustleaf
use math::geometry::Point
use math::algebra::{Vector, Matrix}
use utils::*

// Point, Vector, Matrix, and all utils items are now in scope
pub fn render_scene() {
    var origin = Point.new(0, 0)     // Point from math::geometry
    var transform = Matrix.identity() // Matrix from math::algebra
    log("Rendering...")              // log from utils (via *)
}
```

**Standard Library Scope:**
All standard library functions and types are available globally without explicit import:
```
// These are always available:
print("Hello")           // Built-in function
var x = type(42)         // Built-in function  
var list = [1, 2, 3]     // Built-in type
var dict = {a: 1}        // Built-in type

// No need for:
// use std::io::print
// use std::types::type
```

**Circular Dependency Detection:**
The runtime maintains an import stack to detect circular dependencies during module loading.

```
// File: module_a.rustleaf
use module_b  // ← Circular dependency!

pub fn a_function() {
    module_b.b_function()
}

// File: module_b.rustleaf  
use module_a  // ← Circular dependency!

pub fn b_function() {
    module_a.a_function()
}

// Runtime execution:
// 1. Start loading module_a
// 2. Import stack: [module_a]
// 3. module_a imports module_b
// 4. Import stack: [module_a, module_b]  
// 5. module_b imports module_a
// 6. module_a already in stack → Runtime error!

// Runtime error:
// "Circular dependency detected: module_a → module_b → module_a"
```

**Resolution:**
```
// File: shared.rustleaf
pub fn shared_function() {
    "shared logic"
}

// File: module_a.rustleaf
use shared

pub fn a_function() {
    shared.shared_function() + " from A"
}

// File: module_b.rustleaf
use shared
use module_a  // OK - no circularity

pub fn b_function() {
    module_a.a_function() + " via B"
}
```

