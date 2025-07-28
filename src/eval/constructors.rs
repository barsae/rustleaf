/// Helper constructors for building evaluation AST nodes
use crate::core::{RustValueRef, Value};
use std::cell::RefCell;
use std::rc::Rc;

/// Empty struct serving as a namespace for constructor functions
pub struct Eval;

impl Eval {
    /// Helper to convert a RustValue into a RustValueRef
    fn make_ref<T: crate::core::RustValue + 'static>(value: T) -> RustValueRef {
        RustValueRef::new(Rc::new(RefCell::new(Box::new(value))))
    }

    pub fn literal(value: Value) -> RustValueRef {
        Self::make_ref(crate::eval::EvalLiteral { value })
    }

    pub fn variable(name: String) -> RustValueRef {
        Self::make_ref(crate::eval::EvalVariable { name })
    }

    pub fn call(func_expr: RustValueRef, args: Vec<RustValueRef>) -> RustValueRef {
        Self::make_ref(crate::eval::EvalCall { func_expr, args })
    }

    pub fn program(statements: Vec<RustValueRef>) -> RustValueRef {
        Self::make_ref(crate::eval::EvalProgram { statements })
    }

    pub fn block(statements: Vec<RustValueRef>, final_expr: Option<RustValueRef>) -> RustValueRef {
        Self::make_ref(crate::eval::EvalBlock {
            statements,
            final_expr,
        })
    }

    pub fn declare(name: String, init_expr: Option<RustValueRef>) -> RustValueRef {
        Self::make_ref(crate::eval::EvalDeclare { name, init_expr })
    }

    pub fn if_expr(
        condition: RustValueRef,
        then_expr: RustValueRef,
        else_expr: Option<RustValueRef>,
    ) -> RustValueRef {
        Self::make_ref(crate::eval::EvalIf {
            condition,
            then_expr,
            else_expr,
        })
    }

    pub fn return_expr(expr: Option<RustValueRef>) -> RustValueRef {
        Self::make_ref(crate::eval::EvalReturn { expr })
    }

    pub fn break_expr(expr: Option<RustValueRef>) -> RustValueRef {
        Self::make_ref(crate::eval::EvalBreak { expr })
    }

    pub fn continue_expr() -> RustValueRef {
        Self::make_ref(crate::eval::EvalContinue)
    }

    pub fn assign(name: String, expr: RustValueRef) -> RustValueRef {
        Self::make_ref(crate::eval::EvalAssign { name, expr })
    }

    pub fn loop_expr(body: RustValueRef) -> RustValueRef {
        Self::make_ref(crate::eval::EvalLoop { body })
    }

    pub fn while_expr(condition: RustValueRef, body: RustValueRef) -> RustValueRef {
        Self::make_ref(crate::eval::EvalWhile { condition, body })
    }

    pub fn for_expr(var_name: String, iter_expr: RustValueRef, body: RustValueRef) -> RustValueRef {
        Self::make_ref(crate::eval::EvalFor {
            var_name,
            iter_expr,
            body,
        })
    }

    pub fn logical_and(left: RustValueRef, right: RustValueRef) -> RustValueRef {
        Self::make_ref(crate::eval::EvalLogicalAnd { left, right })
    }

    pub fn logical_or(left: RustValueRef, right: RustValueRef) -> RustValueRef {
        Self::make_ref(crate::eval::EvalLogicalOr { left, right })
    }

    pub fn logical_not(expr: RustValueRef) -> RustValueRef {
        Self::make_ref(crate::eval::EvalLogicalNot { expr })
    }

    pub fn is(left: RustValueRef, right: RustValueRef) -> RustValueRef {
        Self::make_ref(crate::eval::EvalIs { left, right })
    }

    pub fn get_attr(obj_expr: RustValueRef, attr_name: String) -> RustValueRef {
        Self::make_ref(crate::eval::EvalGetAttr {
            obj_expr,
            attr_name,
        })
    }

    pub fn set_attr(
        obj_expr: RustValueRef,
        attr_name: String,
        value_expr: RustValueRef,
    ) -> RustValueRef {
        Self::make_ref(crate::eval::EvalSetAttr {
            obj_expr,
            attr_name,
            value_expr,
        })
    }

    pub fn get_item(obj_expr: RustValueRef, index_expr: RustValueRef) -> RustValueRef {
        Self::make_ref(crate::eval::EvalGetItem {
            obj_expr,
            index_expr,
        })
    }

    pub fn set_item(
        obj_expr: RustValueRef,
        index_expr: RustValueRef,
        value_expr: RustValueRef,
    ) -> RustValueRef {
        Self::make_ref(crate::eval::EvalSetItem {
            obj_expr,
            index_expr,
            value_expr,
        })
    }

    pub fn list(elements: Vec<RustValueRef>) -> RustValueRef {
        Self::make_ref(crate::eval::EvalList { elements })
    }

    pub fn dict(pairs: Vec<(RustValueRef, RustValueRef)>) -> RustValueRef {
        Self::make_ref(crate::eval::EvalDict { pairs })
    }

    pub fn function(data: crate::eval::FunctionData) -> RustValueRef {
        Self::make_ref(crate::eval::EvalFunction { data })
    }

    pub fn lambda(data: crate::eval::LambdaData) -> RustValueRef {
        Self::make_ref(crate::eval::EvalLambda { data })
    }

    pub fn class_decl(data: crate::eval::ClassDeclData) -> RustValueRef {
        Self::make_ref(crate::eval::EvalClassDecl { data })
    }

    pub fn import(data: crate::eval::ImportData) -> RustValueRef {
        Self::make_ref(crate::eval::EvalImport { data })
    }

    pub fn match_expr(data: crate::eval::MatchData) -> RustValueRef {
        Self::make_ref(crate::eval::EvalMatch { data })
    }

    pub fn macro_expr(data: crate::eval::MacroData) -> RustValueRef {
        Self::make_ref(crate::eval::EvalMacro { data })
    }

    pub fn with_expr(data: crate::eval::WithData) -> RustValueRef {
        Self::make_ref(crate::eval::EvalWith { data })
    }

    pub fn declare_pattern(
        pattern: crate::eval::EvalPattern,
        init_expr: RustValueRef,
    ) -> RustValueRef {
        Self::make_ref(crate::eval::EvalDeclarePattern { pattern, init_expr })
    }

    pub fn try_expr(
        body: RustValueRef,
        catch_pattern: crate::eval::EvalPattern,
        catch_body: RustValueRef,
    ) -> RustValueRef {
        Self::make_ref(crate::eval::EvalTry {
            body,
            catch_pattern,
            catch_body,
        })
    }
}
