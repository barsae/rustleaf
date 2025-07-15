# RustLeaf Language Specification

Version 1.0

## Table of Contents

### 1. Introduction
[1.1. Scope](#11-scope)  
[1.2. Conformance](#12-conformance)  
[1.3. Normative References](#13-normative-references)  
[1.4. Terms and Definitions](#14-terms-and-definitions)  

### 2. Lexical Structure
2.1. Source Text  
2.2. Character Set  
2.3. Lexical Analysis  
2.4. Tokens  
2.5. Comments  
2.6. Whitespace  
2.7. Line Terminators  
2.8. Identifiers  
2.9. Keywords  
2.10. Literals  
   2.10.1. Integer Literals  
   2.10.2. Floating-Point Literals  
   2.10.3. String Literals  
   2.10.4. Boolean Literals  
   2.10.5. Null Literal  

### 3. Types
3.1. Type System Overview  
3.2. Primitive Types  
   3.2.1. Null Type  
   3.2.2. Boolean Type  
   3.2.3. Numeric Types  
   3.2.4. String Type  
3.3. Composite Types  
   3.3.1. List Type  
   3.3.2. Dict Type  
   3.3.3. Object Type  
3.4. Function Types  
3.5. RustValue Type  
3.6. Type Conversions  
3.7. Type Checking  

### 4. Variables
4.1. Variable Declarations  
4.2. Variable Initialization  
4.3. Scope and Lifetime  
4.4. Assignment  

### 5. Expressions
5.1. Expression Evaluation  
5.2. Primary Expressions  
   5.2.1. Literals  
   5.2.2. Identifiers  
   5.2.3. Parenthesized Expressions  
5.3. Postfix Expressions  
   5.3.1. Property Access  
   5.3.2. Index Access  
   5.3.3. Function Call  
   5.3.4. Method Call  
5.4. Unary Expressions  
5.5. Binary Expressions  
   5.5.1. Arithmetic Operators  
   5.5.2. Comparison Operators  
   5.5.3. Logical Operators  
   5.5.4. Bitwise Operators  
5.6. Assignment Expressions  
5.7. Conditional Expressions (if)  
5.8. Match Expressions  
5.9. Try Expressions  
5.10. Block Expressions  
5.11. Anonymous Function Expressions  
5.12. Object Literal Expressions  
5.13. List Literal Expressions  
5.14. Dict Literal Expressions  

### 6. Statements
6.1. Statement Evaluation  
6.2. Expression Statements  
6.3. Variable Declaration Statements  
6.4. Empty Statements  
6.5. Block Statements  
6.6. Control Flow Statements  
   6.6.1. If Statements  
   6.6.2. While Statements  
   6.6.3. For Statements  
   6.6.4. Match Statements  
   6.6.5. Try-Catch-Finally Statements  
   6.6.6. With Statements  
   6.6.7. Break Statements  
   6.6.8. Continue Statements  
   6.6.9. Return Statements  

### 7. Functions
7.1. Function Declarations  
7.2. Function Parameters  
7.3. Function Body  
7.4. Return Values  
7.5. Closures  
7.6. Anonymous Functions  
7.7. Function Scope  
7.8. Recursion  

### 8. Pattern Matching
8.1. Pattern Syntax  
8.2. Literal Patterns  
8.3. Variable Patterns  
8.4. Wildcard Patterns  
8.5. List Patterns  
8.6. Dict Patterns  
8.7. Range Patterns  
8.8. Guard Expressions  
8.9. Pattern Evaluation  

### 9. Error Handling
9.1. Error Types  
9.2. Raising Errors  
9.3. Try-Catch Blocks  
9.4. Finally Blocks  
9.5. Error Propagation  
9.6. Error Objects  

### 10. Modules
10.1. Module System Overview  
10.2. Import Statements  
10.3. Export Functions  
10.4. Module Resolution  
10.5. Module Loading  
10.6. Module Scope  

### 11. Built-in Functions
11.1. Type Functions  
11.2. Collection Functions  
11.3. String Functions  
11.4. I/O Functions  
11.5. Error Functions  
11.6. Module Functions  

### 12. Standard Library
12.1. String Methods  
12.2. List Methods  
12.3. Dict Methods  
12.4. Object Methods  

### 13. Documentation Comments and Docstrings
13.1. Overview  
13.2. Documentation Comment Syntax  
13.3. Docstring Syntax  
13.4. Function Docstrings  
13.5. Module Docstrings  
13.6. Variable and Constant Docstrings  
13.7. Object and Type Docstrings  
13.8. Docstring Format Conventions  
13.9. Runtime Access to Docstrings  
13.10. Tooling Integration  

### 14. Memory Model
14.1. Value Semantics  
14.2. Reference Semantics  
14.3. Garbage Collection  
14.4. Resource Management  

### 15. Execution Model
15.1. Program Execution  
15.2. Expression Evaluation Order  
15.3. Function Call Semantics  
15.4. Exception Handling  
15.5. Resource Cleanup  

### 16. RustValue Integration
16.1. RustValue Trait  
16.2. Value Enum  
16.3. Field Access  
16.4. Method Dispatch  
16.5. Type Coercion  
16.6. Lifetime Management  

### Appendices
A. Grammar Summary  
B. Operator Precedence  
C. Reserved Words  
D. Built-in Function Reference  
E. Error Codes  
F. Implementation Limits  

---

## 1. Introduction

RustLeaf is a lightweight, dynamically-typed scripting language designed to be embedded within the editor application. It provides a Rust-inspired syntax with expression-oriented semantics, enabling users to extend and automate the editor through scripts.

The language serves multiple purposes:
- **Automated testing** of editor features, allowing comprehensive test suites to verify editor functionality
- **Task automation** for repetitive editing tasks and workflow optimization  
- **Configuration** through code, enabling dynamic and conditional editor settings
- **Plugin development** to extend editor capabilities with custom functionality

RustLeaf scripts use the `.rustleaf` file extension and can be executed in several contexts:
- Embedded within the editor binary as part of the standard library
- Loaded from a `.rustleaf` configuration file in the user's home directory
- Explicitly executed via command-line arguments for automation and testing
- Evaluated interactively in the editor's integrated REPL

This specification defines the complete syntax, semantics, and runtime behavior of RustLeaf. All behavior is fully specified to ensure consistent implementation and reliable execution across different contexts.

### 1.1. Scope

This specification describes the syntax, semantics, and runtime behavior of the RustLeaf programming language version 1.0. It defines:

- The lexical structure and grammar of valid RustLeaf programs
- The type system and value semantics
- Expression evaluation and control flow
- The module system and standard library
- Integration with Rust through the RustValue trait
- Error handling and resource management

This specification does not define:
- The editor API accessible to RustLeaf scripts
- Performance characteristics or optimization strategies
- Debugging interfaces or development tools
- Binary format or bytecode representation

**Note:** This specification is subject to breaking changes in future versions as the language evolves.

### 1.2. Conformance

A conforming implementation of RustLeaf must:

1. **Implement all language features** exactly as specified in this document
2. **Report all errors** as specified, with error messages that clearly identify the nature and location of the error
3. **Provide all built-in functions** with the exact signatures and behaviors defined
4. **Follow the specified evaluation order** for all expressions and statements
5. **Handle Unicode text** according to the requirements in Section 2.2

Conforming implementations must not:
- Add additional keywords or operators not defined in this specification
- Modify the behavior of any specified feature
- Introduce additional built-in functions or types
- Allow syntax not explicitly permitted by the grammar

Any deviation from this specification renders an implementation non-conformant.

### 1.3. Normative References

The following documents are referenced by this specification:

- **Unicode Standard, Version 15.0**: The Unicode Consortium. The Unicode Standard, Version 15.0.0, (Mountain View, CA: The Unicode Consortium, 2022. ISBN 978-1-936213-32-0)
  - Used for: Character encoding, string normalization, and text processing

- **IEEE 754-2019**: IEEE Standard for Floating-Point Arithmetic
  - Referenced for: Floating-point number representation and operations (as implemented by Rust f64)

- **Rust Language Semantics**: The semantics of Rust f64 operations and type conversions
  - Used for: Ensuring floating-point behavior matches the host language

### 1.4. Terms and Definitions

For the purposes of this specification, the following terms and definitions apply:

**Chapter**: A major division of this specification, identified by a single number (e.g., Chapter 5)

**Section**: A subdivision of a chapter, identified by two numbers separated by a period (e.g., Section 5.3)

**Subsection**: A subdivision of a section, identified by three numbers separated by periods (e.g., Subsection 5.3.1)

**Clause**: A subdivision of a subsection, identified by four numbers separated by periods (e.g., Clause 5.3.1.1)

**must**: Indicates an absolute requirement. Implementations that do not follow requirements marked with "must" are non-conformant.

**must not**: Indicates an absolute prohibition. Implementations that perform actions marked "must not" are non-conformant.

**error**: A condition that prevents the successful execution of a program. All errors must be reported to the user with sufficient information to identify their cause and location.

**value**: An instance of one of RustLeaf's types (null, boolean, integer, float, string, list, dict, function, object, or RustValue).

**expression**: A syntactic construct that evaluates to a value.

**statement**: A syntactic construct that performs an action but does not produce a value (though it may contain expressions).

**scope**: A region of program text where a binding is visible.

**identifier**: A sequence of characters used to name variables, functions, and fields.

**token**: A lexical unit produced by tokenizing source text.

**evaluation**: The process of computing the value of an expression.

**runtime**: The period during which a RustLeaf program is executing.

**host**: The Rust application (editor) that embeds the RustLeaf interpreter.

**module**: A RustLeaf source file that can export values for use by other modules.

**REPL**: Read-Eval-Print Loop - the interactive RustLeaf environment within the editor.