# 1. Introduction

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

---

