use crate::parser::ast::*;
use std::fmt;

// Custom Debug implementations that omit SourceLocation for cleaner test output

impl fmt::Debug for AstNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AstNode::Literal(value, _) => f.debug_tuple("Literal").field(value).finish(),
            AstNode::Identifier(name, _) => f.debug_tuple("Identifier").field(name).finish(),
            AstNode::BinaryOp {
                left,
                operator,
                right,
                ..
            } => f
                .debug_struct("BinaryOp")
                .field("left", left)
                .field("operator", operator)
                .field("right", right)
                .finish(),
            AstNode::UnaryOp {
                operator, operand, ..
            } => f
                .debug_struct("UnaryOp")
                .field("operator", operator)
                .field("operand", operand)
                .finish(),
            AstNode::PropertyAccess {
                object, property, ..
            } => f
                .debug_struct("PropertyAccess")
                .field("object", object)
                .field("property", property)
                .finish(),
            AstNode::IndexAccess { object, index, .. } => f
                .debug_struct("IndexAccess")
                .field("object", object)
                .field("index", index)
                .finish(),
            AstNode::FunctionCall {
                function,
                arguments,
                ..
            } => f
                .debug_struct("FunctionCall")
                .field("function", function)
                .field("arguments", arguments)
                .finish(),
            AstNode::Assignment {
                target,
                operator,
                value,
                ..
            } => f
                .debug_struct("Assignment")
                .field("target", target)
                .field("operator", operator)
                .field("value", value)
                .finish(),
            AstNode::If {
                condition,
                then_branch,
                else_ifs,
                else_branch,
                ..
            } => f
                .debug_struct("If")
                .field("condition", condition)
                .field("then_branch", then_branch)
                .field("else_ifs", else_ifs)
                .field("else_branch", else_branch)
                .finish(),
            AstNode::Block { statements, .. } => f
                .debug_struct("Block")
                .field("statements", statements)
                .finish(),
            AstNode::ListLiteral { elements, .. } => f
                .debug_struct("ListLiteral")
                .field("elements", elements)
                .finish(),
            AstNode::DictLiteral { entries, .. } => f
                .debug_struct("DictLiteral")
                .field("entries", entries)
                .finish(),
            AstNode::ExpressionStatement { expression, .. } => f
                .debug_struct("ExpressionStatement")
                .field("expression", expression)
                .finish(),
            AstNode::VariableDeclaration { name, value, .. } => f
                .debug_struct("VariableDeclaration")
                .field("name", name)
                .field("value", value)
                .finish(),
            AstNode::FunctionDeclaration {
                visibility,
                name,
                parameters,
                body,
                ..
            } => f
                .debug_struct("FunctionDeclaration")
                .field("visibility", visibility)
                .field("name", name)
                .field("parameters", parameters)
                .field("body", body)
                .finish(),
            AstNode::ClassDeclaration {
                visibility,
                name,
                members,
                ..
            } => f
                .debug_struct("ClassDeclaration")
                .field("visibility", visibility)
                .field("name", name)
                .field("members", members)
                .finish(),
            AstNode::ImportStatement { path, clause, .. } => f
                .debug_struct("ImportStatement")
                .field("path", path)
                .field("clause", clause)
                .finish(),
            AstNode::WhileStatement {
                condition, body, ..
            } => f
                .debug_struct("WhileStatement")
                .field("condition", condition)
                .field("body", body)
                .finish(),
            AstNode::ForStatement {
                variable,
                index_variable,
                iterable,
                body,
                ..
            } => f
                .debug_struct("ForStatement")
                .field("variable", variable)
                .field("index_variable", index_variable)
                .field("iterable", iterable)
                .field("body", body)
                .finish(),
            AstNode::Program { items, .. } => {
                f.debug_struct("Program").field("items", items).finish()
            }
            AstNode::ReturnStatement { value, .. } => f
                .debug_struct("ReturnStatement")
                .field("value", value)
                .finish(),
            AstNode::Empty { .. } => f.debug_struct("Empty").finish(),
            // Add other variants as needed
            _ => {
                // Fallback for any variants we haven't explicitly handled
                write!(f, "<unhandled AstNode variant>")
            }
        }
    }
}
