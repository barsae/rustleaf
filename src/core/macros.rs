/// Macro expansion support for RustLeaf
use crate::core::*;
use anyhow::Result;
use std::collections::HashMap;

/// Registry for storing macro definitions during parsing
#[derive(Debug, Clone)]
pub struct MacroRegistry {
    macros: HashMap<String, MacroDefinition>,
}

impl Default for MacroRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl MacroRegistry {
    pub fn new() -> Self {
        Self {
            macros: HashMap::new(),
        }
    }

    /// Register a macro definition
    pub fn register_macro(&mut self, name: String, definition: MacroDefinition) {
        self.macros.insert(name, definition);
    }

    /// Get a macro definition by name
    pub fn get_macro(&self, name: &str) -> Option<&MacroDefinition> {
        self.macros.get(name)
    }

    /// Check if a macro is defined
    pub fn has_macro(&self, name: &str) -> bool {
        self.macros.contains_key(name)
    }
}

/// Represents a macro definition (a function marked with #[macro])
#[derive(Debug, Clone, PartialEq)]
pub struct MacroDefinition {
    pub name: String,
    pub params: Vec<Parameter>,
    pub body: Block,
}

impl MacroDefinition {
    pub fn new(name: String, params: Vec<Parameter>, body: Block) -> Self {
        Self { name, params, body }
    }
}

/// Extract macro definitions from parsed statements
pub fn extract_macro_definitions(
    statements: &[Statement],
) -> Result<(MacroRegistry, Vec<Statement>)> {
    let mut registry = MacroRegistry::new();
    let mut non_macro_statements = Vec::new();

    for stmt in statements {
        if let Some(macro_def) = extract_macro_definition(stmt) {
            registry.register_macro(macro_def.name.clone(), macro_def);
        } else {
            non_macro_statements.push(stmt.clone());
        }
    }

    Ok((registry, non_macro_statements))
}

/// Extract a macro definition from a statement if it's a macro
fn extract_macro_definition(stmt: &Statement) -> Option<MacroDefinition> {
    if let Statement::Macro {
        name,
        args: _,
        statement,
    } = stmt
    {
        if name == "macro" {
            // This is a macro definition: #[macro] fn name() {}
            if let Statement::FnDecl {
                name,
                params,
                body,
                is_pub: _,
            } = statement.as_ref()
            {
                return Some(MacroDefinition::new(
                    name.clone(),
                    params.clone(),
                    body.clone(),
                ));
            }
        }
    }
    None
}

/// Apply macro expansions to statements
pub fn apply_macro_expansions(
    statements: Vec<Statement>,
    registry: &MacroRegistry,
) -> Result<Vec<Statement>> {
    let mut expanded_statements = Vec::new();

    for stmt in statements {
        let expanded = expand_statement(stmt, registry)?;
        expanded_statements.extend(expanded);
    }

    Ok(expanded_statements)
}

/// Expand a single statement, applying any macro annotations
fn expand_statement(stmt: Statement, registry: &MacroRegistry) -> Result<Vec<Statement>> {
    match stmt {
        Statement::Macro {
            name,
            args: _,
            statement,
        } => {
            if let Some(_macro_def) = registry.get_macro(&name) {
                // For now, just return the original statement without the macro annotation
                // TODO: Implement actual macro expansion by calling the macro function
                // and transforming the AST
                expand_statement(*statement, registry)
            } else {
                // Unknown macro - return the statement as-is for now
                // In a full implementation, this might be an error
                expand_statement(*statement, registry)
            }
        }
        // Recursively process statements that can contain other statements
        Statement::Expression(expr) => {
            let expanded_expr = expand_expression(expr, registry)?;
            Ok(vec![Statement::Expression(expanded_expr)])
        }
        Statement::FnDecl {
            name,
            params,
            body,
            is_pub,
        } => {
            let expanded_body = expand_block(body, registry)?;
            Ok(vec![Statement::FnDecl {
                name,
                params,
                body: expanded_body,
                is_pub,
            }])
        }
        Statement::ClassDecl {
            name,
            members,
            is_pub,
        } => {
            let expanded_members = expand_class_members(members, registry)?;
            Ok(vec![Statement::ClassDecl {
                name,
                members: expanded_members,
                is_pub,
            }])
        }
        // For other statement types, return as-is
        _ => Ok(vec![stmt]),
    }
}

/// Expand expressions that may contain macro applications
fn expand_expression(expr: Expression, _registry: &MacroRegistry) -> Result<Expression> {
    // For now, return expressions as-is
    // TODO: Handle macro applications in expressions
    Ok(expr)
}

/// Expand a block of statements
fn expand_block(block: Block, registry: &MacroRegistry) -> Result<Block> {
    let expanded_statements = apply_macro_expansions(block.statements, registry)?;
    Ok(Block {
        statements: expanded_statements,
        final_expr: block.final_expr, // TODO: Expand final expression if present
    })
}

/// Expand class members
fn expand_class_members(
    members: Vec<ClassMember>,
    registry: &MacroRegistry,
) -> Result<Vec<ClassMember>> {
    let mut expanded_members = Vec::new();

    for member in members {
        let expanded_kind = match member.kind {
            ClassMemberKind::Method { params, body } => {
                let expanded_body = expand_block(body, registry)?;
                ClassMemberKind::Method {
                    params,
                    body: expanded_body,
                }
            }
            ClassMemberKind::StaticMethod { params, body } => {
                let expanded_body = expand_block(body, registry)?;
                ClassMemberKind::StaticMethod {
                    params,
                    body: expanded_body,
                }
            }
            // For other member types, return as-is
            other => other,
        };

        expanded_members.push(ClassMember {
            name: member.name,
            kind: expanded_kind,
        });
    }

    Ok(expanded_members)
}
