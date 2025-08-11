/// Helper constructors for building evaluation AST nodes
use crate::core::Value;

/// Empty struct serving as a namespace for constructor functions
pub struct Eval;

impl Eval {
    /// Helper to convert a RustValue into a Value
    fn make_ref<T: crate::core::RustValue + 'static>(value: T) -> Value {
        Value::from_rust(value)
    }

    pub fn literal(value: Value) -> Value {
        Self::make_ref(crate::eval::EvalLiteral { value })
    }

    pub fn variable(name: String) -> Value {
        Self::make_ref(crate::eval::EvalVariable { name })
    }

    pub fn call(func_expr: Value, args: Vec<Value>) -> Value {
        Self::make_ref(crate::eval::EvalCall { func_expr, args })
    }

    pub fn program(statements: Vec<Value>) -> Value {
        Self::make_ref(crate::eval::EvalProgram { statements })
    }

    pub fn block(statements: Vec<Value>, final_expr: Option<Value>) -> Value {
        Self::make_ref(crate::eval::EvalBlock {
            statements,
            final_expr,
        })
    }

    pub fn declare(name: String, init_expr: Option<Value>) -> Value {
        Self::make_ref(crate::eval::EvalDeclare { name, init_expr })
    }

    pub fn if_expr(condition: Value, then_expr: Value, else_expr: Option<Value>) -> Value {
        Self::make_ref(crate::eval::EvalIf {
            condition,
            then_expr,
            else_expr,
        })
    }

    pub fn return_expr(expr: Option<Value>) -> Value {
        Self::make_ref(crate::eval::EvalReturn { expr })
    }

    pub fn break_expr(expr: Option<Value>) -> Value {
        Self::make_ref(crate::eval::EvalBreak { expr })
    }

    pub fn continue_expr() -> Value {
        Self::make_ref(crate::eval::EvalContinue)
    }

    pub fn assign(name: String, expr: Value) -> Value {
        Self::make_ref(crate::eval::EvalAssign { name, expr })
    }

    pub fn loop_expr(body: Value) -> Value {
        Self::make_ref(crate::eval::EvalLoop { body })
    }

    pub fn while_expr(condition: Value, body: Value) -> Value {
        Self::make_ref(crate::eval::EvalWhile { condition, body })
    }

    pub fn for_expr(var_name: String, iter_expr: Value, body: Value) -> Value {
        Self::make_ref(crate::eval::EvalFor {
            var_name,
            iter_expr,
            body,
        })
    }

    pub fn logical_and(left: Value, right: Value) -> Value {
        Self::make_ref(crate::eval::EvalLogicalAnd { left, right })
    }

    pub fn logical_or(left: Value, right: Value) -> Value {
        Self::make_ref(crate::eval::EvalLogicalOr { left, right })
    }

    pub fn logical_not(expr: Value) -> Value {
        Self::make_ref(crate::eval::EvalLogicalNot { expr })
    }

    pub fn is(left: Value, right: Value) -> Value {
        Self::make_ref(crate::eval::EvalIs { left, right })
    }

    pub fn get_attr(obj_expr: Value, attr_name: String) -> Value {
        Self::make_ref(crate::eval::EvalGetAttr {
            obj_expr,
            attr_name,
        })
    }

    pub fn set_attr(obj_expr: Value, attr_name: String, value_expr: Value) -> Value {
        Self::make_ref(crate::eval::EvalSetAttr {
            obj_expr,
            attr_name,
            value_expr,
        })
    }

    pub fn get_item(obj_expr: Value, index_expr: Value) -> Value {
        Self::make_ref(crate::eval::EvalGetItem {
            obj_expr,
            index_expr,
        })
    }

    pub fn set_item(obj_expr: Value, index_expr: Value, value_expr: Value) -> Value {
        Self::make_ref(crate::eval::EvalSetItem {
            obj_expr,
            index_expr,
            value_expr,
        })
    }

    pub fn list(elements: Vec<Value>) -> Value {
        Self::make_ref(crate::eval::EvalList { elements })
    }

    pub fn dict(pairs: Vec<(Value, Value)>) -> Value {
        Self::make_ref(crate::eval::EvalDict { pairs })
    }

    pub fn function(data: crate::eval::FunctionData) -> Value {
        Self::make_ref(crate::eval::EvalFunction { data })
    }

    pub fn lambda(data: crate::eval::LambdaData) -> Value {
        Self::make_ref(crate::eval::EvalLambda { data })
    }

    pub fn class_decl(data: crate::eval::ClassDeclData) -> Value {
        Self::make_ref(crate::eval::EvalClassDecl { data })
    }

    pub fn import(data: crate::eval::ImportData) -> Value {
        Self::make_ref(crate::eval::EvalImport { data })
    }

    pub fn match_expr(data: crate::eval::MatchData) -> Value {
        Self::make_ref(crate::eval::EvalMatch { data })
    }

    pub fn macro_expr(data: crate::eval::MacroData) -> Value {
        Self::make_ref(crate::eval::EvalMacro { data })
    }

    pub fn with_expr(data: crate::eval::WithData) -> Value {
        Self::make_ref(crate::eval::EvalWith { data })
    }

    pub fn declare_pattern(pattern: crate::eval::EvalPattern, init_expr: Value) -> Value {
        Self::make_ref(crate::eval::EvalDeclarePattern { pattern, init_expr })
    }

    pub fn try_expr(
        body: Value,
        catch_pattern: crate::eval::EvalPattern,
        catch_body: Value,
    ) -> Value {
        Self::make_ref(crate::eval::EvalTry {
            body,
            catch_pattern,
            catch_body,
        })
    }
}
