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

### MAJOR-001: Unit Type Return Value Inconsistencies
- **Description**: Multiple conflicting descriptions of when unit is returned
- **Locations**: 
  - `specs/03-types.md:94-98` - Says unit is "returned by functions without explicit return value"
  - `specs/06-statements.md:366` - Says "Expression statements (the discarded value is not unit, but statements themselves produce unit)"
- **Impact**: Fundamental ambiguity about expression evaluation semantics
- **Priority**: High - affects core language semantics
- **Recommendation**: Clarify and standardize unit type semantics across all sections

### MAJOR-002: Grammar vs Implementation Mismatches
- **Description**: Appendix grammar conflicts with semantic descriptions
- **Specific Cases**:
  - `specs/18-appendices.md:75` shows `BreakStatement ::= "break" Expression? ";"`
  - `specs/06-statements.md:494` shows break statements have no expression support
- **Impact**: Grammar doesn't match semantic description, could cause parser implementation issues
- **Priority**: High - affects language implementation
- **Recommendation**: Align grammar with semantic specifications

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

### MINOR-002: Method vs Function Usage Ambiguity
- **Description**: Some examples use `.len()` method calls while others use `len()` function calls without clear rules
- **Locations**: 
  - `specs/12-standard-library.md:575-592` - Shows both forms as equivalent
  - Various other sections mix usage patterns
- **Impact**: Unclear when to use function vs method form
- **Priority**: Low - both forms work, but consistency would be better
- **Recommendation**: Establish clear guidelines for when to show function vs method forms

### MINOR-003: Try-Catch Syntax Variations
- **Description**: Try-catch syntax differs between statement and expression forms
- **Locations**:
  - `specs/06-statements.md:376` - Statement form syntax
  - `specs/05-expressions.md:578` - Expression form syntax
- **Impact**: Potential parser ambiguity, though likely resolved by context
- **Priority**: Low - likely not a real issue
- **Recommendation**: Verify syntax is truly identical or document differences

### MINOR-004: Operator Precedence Table Formatting
- **Description**: Section 5.5.5 and Appendix B show precedence with different formatting
- **Locations**:
  - `specs/05-expressions.md:434-446` - Numbered list format
  - `specs/18-appendices.md:232-247` - Table format
- **Impact**: Could lead to misinterpretation, though content is consistent
- **Priority**: Low - content is correct, formatting differs
- **Recommendation**: Use consistent formatting for precedence rules

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