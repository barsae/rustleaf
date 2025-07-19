# 13. Documentation Comments and Docstrings

RustLeaf provides a comprehensive documentation system using Rust-style documentation comments and docstrings. Documentation can be attached to functions, classes, modules, and variables, and is accessible at runtime for dynamic introspection. This chapter defines the documentation syntax, format conventions, and runtime access mechanisms.

### 13.1. Overview

**Documentation Types:**
- **Documentation comments**: `///` for single-line, `/** */` for multi-line
- **Docstrings**: Located above declarations, not inside function bodies
- **Runtime access**: Documentation available via reflection functions
- **Markdown support**: Rich formatting with Markdown syntax
- **Structured format**: Support for `@param`, `@returns`, `@example` tags

**Documentation Targets:**
- Functions and methods
- Classes and their members
- Modules (file-level)
- Variables and constants
- Type definitions

### 13.2. Documentation Comment Syntax

Documentation comments use Rust-style syntax with special comment markers.

**Single-line Documentation Comments:**
```
/// This is a single-line documentation comment
/// It can span multiple lines by repeating the marker
fn documented_function() {
    // Implementation
}
```

**Multi-line Documentation Comments:**
```
/**
 * This is a multi-line documentation comment
 * It can contain rich formatting and examples
 * 
 * @param x The input value
 * @returns The processed result
 */
fn complex_function(x) {
    // Implementation
}
```

**Mixed Documentation:**
```
/// Brief description of the function
/**
 * Extended description with more details
 * and formatting options.
 */
fn mixed_docs_function() {
    // Implementation
}
```

### 13.3. Docstring Syntax

Docstrings use Rust-style placement above declarations rather than inside function bodies.

**Function Docstrings:**
```
/// Calculates the factorial of a number
/// 
/// @param n The number to calculate factorial for
/// @returns The factorial result
/// @example
/// ```
/// factorial(5)  // Returns 120
/// factorial(0)  // Returns 1
/// ```
fn factorial(n) {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}
```

**Class Docstrings:**
```
/// Represents a point in 2D space
/// 
/// This class provides basic geometric operations
/// for working with 2D coordinates.
/// 
/// @example
/// ```
/// var p = Point.new(3, 4)
/// print(p.distance_to_origin())  // 5.0
/// ```
class Point {
    /// The x-coordinate of the point
    var x;
    
    /// The y-coordinate of the point  
    var y;
    
    /// Creates a new point with given coordinates
    /// 
    /// @param x The x-coordinate
    /// @param y The y-coordinate
    /// @returns A new Point instance
    static fn new(x, y) {
        var p = Point();
        p.x = x;
        p.y = y;
        p
    }
    
    /// Calculates the distance from this point to the origin
    /// 
    /// @returns The Euclidean distance to (0, 0)
    fn distance_to_origin() {
        (self.x * self.x + self.y * self.y) ** 0.5
    }
}
```

### 13.4. Function Docstrings

Function documentation supports structured tags for parameters, return values, and examples.

**Basic Function Documentation:**
```
/// Adds two numbers together
/// 
/// @param a The first number
/// @param b The second number
/// @returns The sum of a and b
fn add(a, b) {
    a + b
}
```

**Complex Function Documentation:**
```
/// Processes a list of items with a transformation function
/// 
/// This function applies the given transformation to each item
/// in the input list and returns a new list with the results.
/// The original list is not modified.
/// 
/// @param items The list of items to process
/// @param transform_fn The function to apply to each item
/// @returns A new list with transformed items
/// 
/// @example
/// ```
/// var numbers = [1, 2, 3]
/// var doubled = process_items(numbers, |x| x * 2)
/// print(doubled)  // [2, 4, 6]
/// ```
/// 
/// @example
/// ```
/// var words = ["hello", "world"]
/// var upper = process_items(words, |s| s.upper())
/// print(upper)  // ["HELLO", "WORLD"]
/// ```
fn process_items(items, transform_fn) {
    items.map(transform_fn)
}
```

**Function with Default Parameters:**
```
/// Creates a range of numbers
/// 
/// @param start The starting number (inclusive)
/// @param end The ending number (exclusive)  
/// @param step The step size (default: 1)
/// @returns A list of numbers in the specified range
/// 
/// @example
/// ```
/// create_range(0, 5)     // [0, 1, 2, 3, 4]
/// create_range(1, 10, 2) // [1, 3, 5, 7, 9]
/// ```
fn create_range(start, end, step = 1) {
    range(start, end, step)
}
```

### 13.5. Module Docstrings

Module-level documentation appears at the top of files to describe the module's purpose and contents.

**Module Documentation:**
```
/// Mathematical utility functions
/// 
/// This module provides common mathematical operations
/// and constants for use throughout the application.
/// 
/// @example
/// ```
/// use math_utils::{PI, circle_area}
/// 
/// var area = circle_area(5.0)
/// print("Area: ${area}")
/// ```

/// The mathematical constant π
pub var PI = 3.14159265359;

/// Calculates the area of a circle
/// 
/// @param radius The radius of the circle
/// @returns The area of the circle
pub fn circle_area(radius) {
    PI * radius * radius
}

/// Calculates the circumference of a circle
/// 
/// @param radius The radius of the circle
/// @returns The circumference of the circle  
pub fn circle_circumference(radius) {
    2 * PI * radius
}
```

### 13.6. Variable and Constant Docstrings

Variables and constants can be documented for API clarity.

**Variable Documentation:**
```
/// The default timeout for network operations (in seconds)
var DEFAULT_TIMEOUT = 30;

/// Configuration options for the application
var CONFIG = {
    /// The server host address
    host: "localhost",
    
    /// The server port number
    port: 8080,
    
    /// Whether to enable debug logging
    debug: false
};

/// List of supported file extensions
pub var SUPPORTED_EXTENSIONS = [".txt", ".md", ".rs"];
```

**Class Member Documentation:**
```
class DatabaseConnection {
    /// The connection string used to connect to the database
    var connection_string;
    
    /// Whether the connection is currently active
    var is_connected = false;
    
    /// The maximum number of retry attempts
    var max_retries = 3;
}
```

### 13.7. Object and Type Docstrings

Classes and their members support hierarchical documentation.

**Class with Comprehensive Documentation:**
```
/// A configuration management system
/// 
/// This class provides a structured way to manage application
/// configuration with support for nested values, defaults,
/// and environment variable overrides.
/// 
/// @example
/// ```
/// var config = ConfigManager.new("app.json")
/// config.set("database.host", "localhost")
/// var host = config.get("database.host", "default")
/// ```
class ConfigManager {
    /// The path to the configuration file
    var config_path;
    
    /// The loaded configuration data
    var data = {};
    
    /// Creates a new configuration manager
    /// 
    /// @param path The path to the configuration file
    /// @returns A new ConfigManager instance
    static fn new(path) {
        var manager = ConfigManager();
        manager.config_path = path;
        manager.load();
        manager
    }
    
    /// Loads configuration from the file
    /// 
    /// Reads the configuration file and parses it as JSON.
    /// If the file doesn't exist, starts with empty configuration.
    fn load() {
        // Implementation details...
    }
    
    /// Gets a configuration value
    /// 
    /// Retrieves a value from the configuration using dot notation
    /// for nested keys. Returns the default value if the key is not found.
    /// 
    /// @param key The configuration key (supports dot notation)
    /// @param default_value The value to return if key is not found
    /// @returns The configuration value or default
    /// 
    /// @example
    /// ```
    /// config.get("database.host")           // "localhost"
    /// config.get("missing.key", "default")  // "default"
    /// ```
    fn get(key, default_value = null) {
        // Implementation details...
    }
    
    /// Sets a configuration value
    /// 
    /// @param key The configuration key (supports dot notation)
    /// @param value The value to set
    /// @returns Self for method chaining
    fn set(key, value) {
        // Implementation details...
        self
    }
}
```

### 13.8. Docstring Format Conventions

**Markdown Formatting:**
Documentation supports Markdown for rich formatting.

```
/// # Math Utilities
/// 
/// This module provides **essential** mathematical functions:
/// 
/// - Basic arithmetic operations
/// - Trigonometric functions  
/// - Statistical calculations
/// 
/// ## Usage
/// 
/// ```rustleaf
/// use math::{sin, cos, mean}
/// 
/// var angle = PI / 4
/// var x = cos(angle)
/// var y = sin(angle)
/// 
/// var numbers = [1, 2, 3, 4, 5]
/// var average = mean(numbers)
/// ```
/// 
/// > **Note**: All angle parameters are in radians.
```

**Structured Tags:**
```
/// Brief description of the function
/// 
/// Longer description with multiple paragraphs
/// explaining the behavior and use cases.
/// 
/// @param name description
/// @param name description (optional)
/// @returns description
/// @throws ErrorType description of when this error occurs
/// @example
/// ```
/// code example
/// ```
/// @see related_function
/// @since version 1.2.0
/// @deprecated Use new_function instead
```

### 13.9. Runtime Access to Docstrings

Documentation is accessible at runtime for dynamic introspection and tooling.

**Documentation Access Functions:**
```
/// Gets the documentation for a function
/// 
/// @param function_ref The function to get documentation for
/// @returns The documentation string or null if not documented
fn get_doc(function_ref) {
    // Implementation provided by runtime
}

/// Gets documentation for a class
/// 
/// @param class_ref The class to get documentation for  
/// @returns The class documentation or null
fn get_class_doc(class_ref) {
    // Implementation provided by runtime
}

/// Gets documentation for a specific method
/// 
/// @param class_ref The class containing the method
/// @param method_name The name of the method
/// @returns The method documentation or null
fn get_method_doc(class_ref, method_name) {
    // Implementation provided by runtime
}
```

**Runtime Documentation Examples:**
```
/// Calculates the square of a number
/// 
/// @param x The number to square
/// @returns The square of x
fn square(x) {
    x * x
}

// Access documentation at runtime
var doc = get_doc(square)
print(doc)
// Output: "Calculates the square of a number\n\n@param x The number to square\n@returns The square of x"

// Documentation for classes
var class_doc = get_class_doc(Point)
print(class_doc)
// Output: "Represents a point in 2D space\n\nThis class provides basic geometric operations..."

// Documentation for methods
var method_doc = get_method_doc(Point, "distance_to_origin")
print(method_doc)
// Output: "Calculates the distance from this point to the origin..."
```

### 13.10. Tooling Integration

Documentation integrates with development tools for enhanced developer experience.

**IDE Integration:**
- Hover tooltips show formatted documentation
- Auto-completion includes parameter information
- Quick documentation lookup with keyboard shortcuts
- Inline parameter hints during function calls

**Documentation Generation:**
- Export documentation to HTML/Markdown formats
- Generate API reference documentation
- Extract examples for testing
- Validate documentation completeness

**Linting and Validation:**
- Warn about undocumented public functions
- Validate `@param` tags match function parameters
- Check that `@returns` is present for non-void functions
- Verify example code syntax

**Example Tool Usage:**
```
/// Validates an email address
/// 
/// @param email The email address to validate
/// @returns True if the email is valid, false otherwise
/// 
/// @example
/// ```
/// validate_email("user@example.com")  // true
/// validate_email("invalid-email")     // false
/// ```
fn validate_email(email) {
    // Implementation
}

// IDE shows on hover:
// validate_email(email)
// 
// Validates an email address
// 
// Parameters:
//   email - The email address to validate
// 
// Returns:
//   True if the email is valid, false otherwise
```

**Documentation Coverage:**
```bash
