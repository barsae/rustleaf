# 17. Macros

RustLeaf provides a powerful macro system for AST transformation and code generation. Macros operate on the parse tree during module loading, enabling compile-time code transformation, boilerplate elimination, and domain-specific language creation. This chapter defines macro syntax, processing, and capabilities.

### 17.1. Macro Syntax

Macros use Rust-style syntax with `#[macro_name]` annotations placed above declarations.

**Basic Macro Syntax:**
```
#[macro_name]
fn target_function() {
    // Original function
}
```

**Parameterized Macros:**
```
#[macro_name(param1, param2)]
fn target_function() {
    // Function with macro parameters
}

#[macro_name(key: "value", flag: true)]
class TargetClass {
    // Class with named parameters
}
```

**Multiple Macros:**
```
#[first_macro]
#[second_macro(config: "production")]
#[third_macro]
fn heavily_decorated_function() {
    // Multiple macros applied in declaration order
}
```

**Macro Targets:**
Macros can be applied to various language constructs:

```
// Functions
#[test]
fn test_addition() {
    assert(2 + 2 == 4)
}

// Classes
#[serializable]
class User {
    var name;
    var email;
}

// Methods
class Database {
    #[cached(ttl: 300)]
    fn expensive_query(sql) {
        // Database query implementation
    }
}

// Variables
#[config("database.host")]
var db_host = "localhost";

// Modules (at file level)
#[api_version("v1")]
// Module-level macro at top of file
```

### 17.2. Macro Application Rules

Macros are applied during module loading, before code execution.

**Processing Phase:**
Macros transform the AST before any code runs.

```
// File: math_utils.rustleaf

#[benchmark]
fn calculate_fibonacci(n) {
    if n <= 1 {
        n
    } else {
        calculate_fibonacci(n - 1) + calculate_fibonacci(n - 2)
    }
}

// When this module is imported:
// 1. Parse the file into AST
// 2. Process #[benchmark] macro - transforms the function
// 3. Execute module top-level code with transformed AST
```

**Macro Composition:**
Multiple macros on the same item are processed in declaration order.

```
#[validate_params]
#[cache_result]
#[log_calls]
fn complex_operation(data) {
    // Processed in order:
    // 1. validate_params transforms the function
    // 2. cache_result transforms the result from step 1
    // 3. log_calls transforms the result from step 2
}
```

**Scope and Access:**
Macros have unrestricted access to all language constructs and scope.

```
// Macro can access and modify:
var private_config = {secret: "hidden"}

#[custom_macro]
fn public_function() {
    // Macro can read/modify private_config
    // Macro can access any module-level variables
    // Macro can generate code that uses any identifiers
}
```

### 17.3. Built-in Macros

RustLeaf provides standard macros for common development tasks.

**Testing Macros:**
```
#[test]
fn test_user_creation() {
    var user = User.new("Alice", "alice@example.com")
    assert(user.name == "Alice")
    assert(user.email == "alice@example.com")
}

#[test]
fn test_error_handling() {
    try {
        invalid_operation()
        assert(false, "Should have raised an error")
    } catch e {
        assert(str(e).contains("Invalid"))
    }
}
```

**Deprecation Macros:**
```
#[deprecated("Use new_api() instead")]
fn old_api() {
    // Legacy implementation
    new_api()
}

#[deprecated("This class will be removed in v2.0", since: "1.5.0")]
class LegacyProcessor {
    // Deprecated class
}
```

**Performance Macros:**
```
#[benchmark]
fn performance_critical_function() {
    // Function execution is automatically timed
    // Results logged or collected for analysis
}

#[profile]
fn memory_intensive_operation() {
    // Memory usage tracked during execution
}
```

**Documentation Macros:**
```
#[example]
fn api_usage_example() {
    // This function serves as a living example
    // May be extracted for documentation
    var client = ApiClient.new()
    client.authenticate("token")
    var data = client.fetch_data()
    data.process()
}
```

### 17.4. User-defined Macros

Users can define custom macros that transform the AST.

**Macro Declaration:**
Custom macros are functions marked with `#[macro]`.

```
#[macro]
fn log_execution(ast_node) {
    if ast_node.type != "function" {
        raise("@log_execution can only be applied to functions")
    }
    
    var function_name = ast_node.name
    var original_body = ast_node.body
    
    // Create new function body with logging
    var new_body = parse("""
        {
            print("Entering function: ${function_name}")
            var start_time = current_time()
            
            var result = ${original_body}
            
            var end_time = current_time()
            print("Exiting function: ${function_name}, took: ${end_time - start_time}ms")
            result
        }
    """)
    
    // Return modified AST node
    ast_node.body = new_body
    ast_node
}
```

**Macro Usage:**
```
#[log_execution]
fn calculate_result(data) {
    // Original function implementation
    process_data(data)
}

// After macro processing, equivalent to:
fn calculate_result(data) {
    print("Entering function: calculate_result")
    var start_time = current_time()
    
    var result = {
        process_data(data)  // Original body
    }
    
    var end_time = current_time()
    print("Exiting function: calculate_result, took: ${end_time - start_time}ms")
    result
}
```

**Parameterized Custom Macros:**
```
#[macro]
fn retry(ast_node, max_attempts: 3, delay: 1000) {
    if ast_node.type != "function" {
        raise("@retry can only be applied to functions")
    }
    
    var original_body = ast_node.body
    
    var new_body = parse("""
        {
            var attempts = 0
            while attempts < ${max_attempts} {
                try {
                    ${original_body}
                } catch e {
                    attempts = attempts + 1
                    if attempts >= ${max_attempts} {
                        raise(e)
                    }
                    sleep(${delay})
                }
            }
        }
    """)
    
    ast_node.body = new_body
    ast_node
}

// Usage
#[retry(max_attempts: 5, delay: 2000)]
fn unreliable_network_call() {
    fetch_data_from_api()
}
```

### 17.5. Macro Processing

Macros receive AST nodes and return transformed AST nodes.

**AST Node Structure:**
Macros work with structured representations of code.

```
// Example AST node for a function
{
    type: "function",
    name: "example_function",
    parameters: [
        {name: "param1", default: null},
        {name: "param2", default: "default_value"}
    ],
    body: {
        type: "block",
        statements: [...]
    },
    modifiers: ["pub", "static"],  // If applicable
    location: {file: "example.rustleaf", line: 10, column: 1}
}
```

**Code Generation:**
Macros can generate new code using the `parse()` function.

```
#[macro]
fn property_accessor(ast_node) {
    if ast_node.type != "class" {
        raise("@property_accessor only applies to classes")
    }
    
    var new_methods = []
    
    // Generate getter and setter for each field
    for field in ast_node.fields {
        var getter = parse("""
            fn get_${field.name}() {
                self.${field.name}
            }
        """)
        
        var setter = parse("""
            fn set_${field.name}(value) {
                self.${field.name} = value
                self
            }
        """)
        
        new_methods.append(getter)
        new_methods.append(setter)
    }
    
    // Add generated methods to the class
    ast_node.methods.extend(new_methods)
    ast_node
}

// Usage
#[property_accessor]
class User {
    var name;
    var email;
    var age;
}

// Generates methods: get_name(), set_name(), get_email(), set_email(), etc.
```

### 17.6. AST Transformation

Macros can perform complex AST transformations.

**Code Injection:**
```
#[macro]
fn inject_validation(ast_node) {
    if ast_node.type != "function" {
        raise("@inject_validation only applies to functions")
    }
    
    // Inject validation at the beginning of function
    var validation_code = parse("""
        if args.len() != ${ast_node.parameters.len()} {
            raise("Invalid number of arguments")
        }
    """)
    
    // Prepend validation to function body
    ast_node.body.statements.insert(0, validation_code)
    ast_node
}
```

**Code Wrapping:**
```
#[macro]
fn transaction(ast_node) {
    var original_body = ast_node.body
    
    var wrapped_body = parse("""
        {
            var tx = database.begin_transaction()
            try {
                var result = ${original_body}
                tx.commit()
                result
            } catch e {
                tx.rollback()
                raise(e)
            }
        }
    """)
    
    ast_node.body = wrapped_body
    ast_node
}
```

**Multiple Item Generation:**
```
#[macro]
fn crud_operations(ast_node) {
    if ast_node.type != "class" {
        raise("@crud_operations only applies to classes")
    }
    
    var class_name = ast_node.name
    var module_items = []
    
    // Generate CRUD functions
    var create_fn = parse("""
        fn create_${class_name}(data) {
            var instance = ${class_name}()
            for key, value in data.items() {
                instance[key] = value
            }
            database.save(instance)
            instance
        }
    """)
    
    var read_fn = parse("""
        fn read_${class_name}(id) {
            database.find(${class_name}, id)
        }
    """)
    
    module_items.append(create_fn)
    module_items.append(read_fn)
    
    // Return the original class plus generated functions
    [ast_node] + module_items
}
```

### 17.7. Macro Evaluation Order

Macros are processed in a predictable order during module loading.

**Processing Order:**
1. **File parsing**: Source code parsed into initial AST
2. **Macro resolution**: Macro functions are identified and loaded
3. **Macro application**: Macros applied in declaration order
4. **Final AST**: Transformed AST ready for execution

**Declaration Order Processing:**
```
#[first_macro]
#[second_macro]
#[third_macro]
fn example() {
    // Processing order:
    // 1. first_macro receives original AST
    // 2. second_macro receives result from first_macro
    // 3. third_macro receives result from second_macro
    // 4. Final transformed function is used
}
```

**Module Dependencies:**
Macros from imported modules are available for use.

```
// File: macros/logging.rustleaf
#[macro]
pub fn debug_trace(ast_node) {
    // Macro implementation
}

// File: main.rustleaf
use macros::logging::debug_trace;

#[debug_trace]
fn main_function() {
    // Uses imported macro
}
```

**Error Handling:**
Macro errors halt module loading with detailed error messages.

```
#[invalid_macro_usage]
var not_a_function = 42

// Error: Macro 'invalid_macro_usage' cannot be applied to variable declarations
//   at main.rustleaf:2:1
//   Macro defined at macros/validation.rustleaf:15:1
```

**Macro Scope and Hygiene:**
Macros have unlimited access to generate any code.

```
#[macro]
fn full_access_macro(ast_node) {
    // Can access any variable in scope
    var secret_value = get_module_variable("secret_config")
    
    // Can generate code that uses private functions
    var new_code = parse("""
        {
            var private_data = access_private_function()
            var result = ${ast_node.body}
            log_to_private_channel(result)
            result
        }
    """)
    
    ast_node.body = new_code
    ast_node
}
```

**Complex Transformation Example:**
```
#[macro]
fn state_machine(ast_node) {
    // Transform a class into a state machine
    var states = []
    var transitions = []
    
    // Analyze class to extract state information
    for method in ast_node.methods {
        if method.name.starts_with("state_") {
            states.append(method.name.replace("state_", ""))
        }
        if method.name.starts_with("transition_") {
            transitions.append(parse_transition(method))
        }
    }
    
    // Generate state machine logic
    var state_machine_code = generate_state_machine(states, transitions)
    
    // Inject state machine into class
    ast_node.methods.extend(state_machine_code)
    ast_node.fields.append(parse("var current_state = \"${states[0]}\";"))
    
    ast_node
}

#[state_machine]
class OrderProcessor {
    fn state_pending() { /* ... */ }
    fn state_processing() { /* ... */ }
    fn state_completed() { /* ... */ }
    
    fn transition_start_processing() { /* ... */ }
    fn transition_complete_order() { /* ... */ }
}
// Becomes a full state machine with transition validation
```
