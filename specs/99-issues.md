# 99. Known Issues and Inconsistencies

This document captures known issues and inconsistencies identified in the RustLeaf language specifications during comprehensive review. Issues are categorized by severity and impact on language implementation and usage.

## Status: Last Updated 2025-01-18

## Critical Issues (Implementation Blocking)

### RESOLVED Issues

#### ~~CRITICAL-001: Assignment Semantics Ambiguity~~ ✅ FIXED
- **Issue**: Assignment vs expression semantics were unclear
- **Resolution**: Section 5.6 now clearly states assignments are statements, with proper reference to Section 6.4
- **Files Updated**: `specs/05-expressions.md`, `specs/00-table-of-contents.md`

## Major Issues (Semantic Clarity)

### ~~MAJOR-001: Unit Type Return Value Inconsistencies~~ ✅ FIXED
- **Description**: Multiple conflicting descriptions of when unit is returned
- **Resolution**: Clarified that functions return unit when they end with a statement rather than an expression. Added clear examples showing the distinction.
- **Files Updated**: `specs/03-types.md`, `specs/06-statements.md`
- **Decision**: Adopted Rust-like semantics where statements perform actions, expressions produce values

### ~~MAJOR-002: Grammar vs Implementation Mismatches~~ ✅ FIXED
- **Description**: Appendix grammar conflicts with semantic descriptions for break statements and loop constructs
- **Resolution**: Implemented Rust-like loop expressions with break value support. Loops can now be used as both statements (when value isn't needed) and expressions (when value is needed).
- **Files Updated**: `specs/05-expressions.md`, `specs/06-statements.md`, `specs/18-appendices.md`, `specs/00-table-of-contents.md`
- **Decision**: Added loop expressions (while, for, loop) that return values via break statements or unit if they complete normally

### MAJOR-003: Iterator Protocol Inconsistencies
- **Description**: Iterator protocol mentions unit return but has inconsistent boolean context restrictions
- **Locations**: 
  - `specs/03-types.md:104` - Unit type boolean context restrictions
  - `specs/12-standard-library.md:434` - Iterator protocol using unit
  - `specs/06-statements.md:290` - For loop iterator usage
- **Impact**: Unclear how to properly implement and use iterators
- **Priority**: High - affects core language feature
- **Recommendation**: Standardize iterator protocol documentation across all mentions

## Minor Issues (Documentation Quality)

### MINOR-001: Raw String Literal Placement
- **Description**: Raw string literals break section numbering
- **Location**: `specs/02-lexical-structure.md:29` lists "2.10.6. Raw String Literals" but content appears after Section 2.10.5
- **Impact**: Structural inconsistency in specification organization
- **Priority**: Low - cosmetic issue
- **Recommendation**: Renumber sections or move raw string content to match ToC

### MINOR-002: Method vs Function Usage Ambiguity - RESOLVED
- **Description**: Some examples use `.len()` method calls while others use `len()` function calls without clear rules
- **Resolution**: Standardized all documentation to use `.len()` method form only. The `len()` function form is incorrect.
- **Changes Made**:
  - Updated `specs/18-appendices.md` to remove function form documentation
  - Fixed all `len()` function calls in `specs/03-types.md` to use `.len()` method
  - Clarified that collections provide `.len()` method, not global `len()` function

### MINOR-003: Try-Catch Syntax Variations - RESOLVED
- **Description**: Try-catch syntax differs between statement and expression forms
- **Resolution**: Intentional design choice - identical syntax with context-dependent parsing
- **Design Decision**: 
  - Same syntax (`try Block catch Pattern Block`) works in both statement and expression contexts
  - Parser distinguishes based on usage context (whether result is used)
  - Consistent with RustLeaf's pattern for if/for statements vs expressions
  - Maintains language simplicity while following established parsing practices

### MINOR-004: Operator Precedence Table Formatting - RESOLVED
- **Description**: Section 5.5.5 and Appendix B show precedence with different formatting
- **Resolution**: Replaced duplicated precedence list with cross-reference to appendix
- **Changes Made**:
  - Removed numbered list from `specs/05-expressions.md:431` 
  - Added cross-reference to "Appendix B: Operator Precedence"
  - Maintained comprehensive table format in appendix as single source of truth
- **Benefits**: Eliminates duplication, prevents inconsistencies, improves maintainability

## Resolved Issues

### ~~STRUCTURAL-001: Missing Section References~~ ✅ FIXED
- **Issue**: References to non-existent Section 4.4
- **Resolution**: Updated to reference correct Section 6.4
- **Files Updated**: `specs/05-expressions.md`

### ~~STRUCTURAL-002: Table of Contents Inconsistencies~~ ✅ PARTIALLY FIXED
- **Issue**: Section numbering in ToC didn't match headers
- **Resolution**: Assignment expressions properly marked as deprecated
- **Status**: Ongoing - other numbering issues may remain

## Implementation Guidelines

### For Language Implementers

1. **Unit Type Semantics**: Until MAJOR-001 is resolved, follow Section 3.2.2 semantics for unit returns
2. **Break Statements**: Follow Section 6.6.5 semantics (no expression support) rather than Appendix grammar
3. **Iterator Protocol**: Use `is_unit()` checks as shown in Section 12.5 examples
4. **Assignment**: Treat assignments as statements only, not expressions

### For Specification Writers

1. **Cross-Reference Validation**: Always verify section references exist and are correct
2. **Consistency Checks**: When updating one section, check related sections for consistency
3. **Example Alignment**: Ensure examples across sections use consistent patterns
4. **Grammar Sync**: Keep Appendix A grammar synchronized with semantic descriptions

## Tracking Guidelines

### Issue Lifecycle
- **Open**: Issue identified but not addressed
- **In Progress**: Issue being worked on
- **Resolved**: Issue fixed and verified
- **Verified**: Fix confirmed across all affected locations

### Priority Levels
- **Critical**: Blocks implementation or causes ambiguous behavior
- **Major**: Significant semantic issues that affect language understanding
- **Minor**: Documentation quality issues that don't affect semantics
- **Cosmetic**: Formatting or organizational issues only

### Update Process
1. When fixing an issue, update this document
2. Mark resolved issues with ~~strikethrough~~ and ✅ 
3. Update the status date at the top
4. Cross-reference any related specification changes

## Future Considerations

### Systematic Improvements Needed
1. **Automated Consistency Checking**: Consider tools to validate cross-references
2. **Example Standardization**: Develop style guide for code examples
3. **Regular Review Process**: Schedule periodic consistency reviews
4. **Reference Validation**: Implement checks for broken internal links

### Long-term Goals
- Zero critical and major consistency issues
- Standardized formatting across all sections  
- Comprehensive cross-reference validation
- Automated testing of specification examples

---

*This document should be updated whenever specification changes are made or new issues are discovered. All issues should be tracked through to resolution to ensure specification quality.*