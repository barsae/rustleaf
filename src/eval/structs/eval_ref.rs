/// Core evaluation types - simplified AST for execution
use crate::core::{ImportItems, ParameterKind, RustValue, Value};
use crate::eval::{EvalResult, Evaluator};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct EvalRef(Rc<RefCell<Box<dyn RustValue>>>);

impl EvalRef {
    pub fn new<T: RustValue + 'static>(eval: T) -> Self {
        Self(Rc::new(RefCell::new(Box::new(eval))))
    }

    pub fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        self.0.borrow().eval(evaluator)
    }

    pub fn str(&self) -> String {
        self.0.borrow().str()
    }

    pub fn as_rust_value(&self) -> Rc<RefCell<Box<dyn RustValue>>> {
        self.0.clone()
    }
}

/// Eval - now a simple wrapper around trait-based evaluation
/// This eliminates the massive match statement dispatch in favor of trait method calls
#[derive(Debug, Clone)]
pub struct Eval(pub EvalRef);

impl Eval {
    /// Create an Eval from any RustValue implementer
    pub fn new<T: crate::core::RustValue + 'static>(eval_impl: T) -> Self {
        Self(EvalRef::new(eval_impl))
    }

    /// Evaluate this Eval node
    pub fn eval(
        &self,
        evaluator: &mut crate::eval::Evaluator,
    ) -> anyhow::Result<crate::eval::EvalResult> {
        self.0.eval(evaluator)
    }

    // Helper constructors for backward compatibility
    pub fn literal(value: Value) -> Self {
        Self::new(crate::eval::EvalLiteral { value })
    }

    pub fn variable(name: String) -> Self {
        Self::new(crate::eval::EvalVariable { name })
    }

    pub fn call(func_expr: Eval, args: Vec<Eval>) -> Self {
        let func_ref = func_expr.0;
        let arg_refs = args.into_iter().map(|arg| arg.0).collect();
        Self::new(crate::eval::EvalCall {
            func_expr: func_ref,
            args: arg_refs,
        })
    }

    pub fn program(statements: Vec<Eval>) -> Self {
        let stmt_refs = statements.into_iter().map(|stmt| stmt.0).collect();
        Self::new(crate::eval::EvalProgram {
            statements: stmt_refs,
        })
    }

    pub fn block(statements: Vec<Eval>, final_expr: Option<Eval>) -> Self {
        let stmt_refs = statements.into_iter().map(|stmt| stmt.0).collect();
        let final_ref = final_expr.map(|expr| expr.0);
        Self::new(crate::eval::EvalBlock {
            statements: stmt_refs,
            final_expr: final_ref,
        })
    }

    pub fn declare(name: String, init_expr: Option<Eval>) -> Self {
        let init_ref = init_expr.map(|expr| expr.0);
        Self::new(crate::eval::EvalDeclare {
            name,
            init_expr: init_ref,
        })
    }

    pub fn if_expr(condition: Eval, then_expr: Eval, else_expr: Option<Eval>) -> Self {
        let cond_ref = condition.0;
        let then_ref = then_expr.0;
        let else_ref = else_expr.map(|expr| expr.0);
        Self::new(crate::eval::EvalIf {
            condition: cond_ref,
            then_expr: then_ref,
            else_expr: else_ref,
        })
    }

    pub fn return_expr(expr: Option<Eval>) -> Self {
        let expr_ref = expr.map(|e| e.0);
        Self::new(crate::eval::EvalReturn { expr: expr_ref })
    }

    pub fn break_expr(expr: Option<Eval>) -> Self {
        let expr_ref = expr.map(|e| e.0);
        Self::new(crate::eval::EvalBreak { expr: expr_ref })
    }

    pub fn continue_expr() -> Self {
        Self::new(crate::eval::EvalContinue)
    }

    pub fn assign(name: String, expr: Eval) -> Self {
        Self::new(crate::eval::EvalAssign { name, expr: expr.0 })
    }

    pub fn loop_expr(body: Eval) -> Self {
        Self::new(crate::eval::EvalLoop { body: body.0 })
    }

    pub fn while_expr(condition: Eval, body: Eval) -> Self {
        Self::new(crate::eval::EvalWhile {
            condition: condition.0,
            body: body.0,
        })
    }

    pub fn for_expr(var_name: String, iter_expr: Eval, body: Eval) -> Self {
        Self::new(crate::eval::EvalFor {
            var_name,
            iter_expr: iter_expr.0,
            body: body.0,
        })
    }

    pub fn logical_and(left: Eval, right: Eval) -> Self {
        Self::new(crate::eval::EvalLogicalAnd {
            left: left.0,
            right: right.0,
        })
    }

    pub fn logical_or(left: Eval, right: Eval) -> Self {
        Self::new(crate::eval::EvalLogicalOr {
            left: left.0,
            right: right.0,
        })
    }

    pub fn logical_not(expr: Eval) -> Self {
        Self::new(crate::eval::EvalLogicalNot { expr: expr.0 })
    }

    pub fn is(left: Eval, right: Eval) -> Self {
        Self::new(crate::eval::EvalIs {
            left: left.0,
            right: right.0,
        })
    }

    pub fn get_attr(obj_expr: Eval, attr_name: String) -> Self {
        Self::new(crate::eval::EvalGetAttr {
            obj_expr: obj_expr.0,
            attr_name,
        })
    }

    pub fn set_attr(obj_expr: Eval, attr_name: String, value_expr: Eval) -> Self {
        Self::new(crate::eval::EvalSetAttr {
            obj_expr: obj_expr.0,
            attr_name,
            value_expr: value_expr.0,
        })
    }

    pub fn get_item(obj_expr: Eval, index_expr: Eval) -> Self {
        Self::new(crate::eval::EvalGetItem {
            obj_expr: obj_expr.0,
            index_expr: index_expr.0,
        })
    }

    pub fn set_item(obj_expr: Eval, index_expr: Eval, value_expr: Eval) -> Self {
        Self::new(crate::eval::EvalSetItem {
            obj_expr: obj_expr.0,
            index_expr: index_expr.0,
            value_expr: value_expr.0,
        })
    }

    pub fn list(elements: Vec<Eval>) -> Self {
        let element_refs = elements.into_iter().map(|elem| elem.0).collect();
        Self::new(crate::eval::EvalList {
            elements: element_refs,
        })
    }

    pub fn dict(pairs: Vec<(Eval, Eval)>) -> Self {
        let pair_refs = pairs.into_iter().map(|(k, v)| (k.0, v.0)).collect();
        Self::new(crate::eval::EvalDict { pairs: pair_refs })
    }

    // Advanced constructors

    pub fn function(data: FunctionData) -> Self {
        Self::new(crate::eval::EvalFunction { data })
    }

    pub fn lambda(data: LambdaData) -> Self {
        Self::new(crate::eval::EvalLambda { data })
    }

    pub fn class_decl(data: ClassDeclData) -> Self {
        Self::new(crate::eval::EvalClassDecl { data })
    }

    pub fn import(data: ImportData) -> Self {
        Self::new(crate::eval::EvalImport { data })
    }

    pub fn match_expr(data: MatchData) -> Self {
        Self::new(crate::eval::EvalMatch { data })
    }

    pub fn macro_expr(data: MacroData) -> Self {
        Self::new(crate::eval::EvalMacro { data })
    }

    pub fn with_expr(data: WithData) -> Self {
        Self::new(crate::eval::EvalWith { data })
    }

    pub fn declare_pattern(pattern: EvalPattern, init_expr: Eval) -> Self {
        Self::new(crate::eval::EvalDeclarePattern {
            pattern,
            init_expr: init_expr.0,
        })
    }

    pub fn try_expr(body: Eval, catch_pattern: EvalPattern, catch_body: Eval) -> Self {
        Self::new(crate::eval::EvalTry {
            body: body.0,
            catch_pattern,
            catch_body: catch_body.0,
        })
    }
}

impl PartialEq for Eval {
    fn eq(&self, _other: &Self) -> bool {
        // TODO
        false
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassMethod {
    pub name: String,
    pub params: Vec<String>,
    pub body: Eval,
    pub is_static: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassDeclData {
    pub name: String,
    pub field_names: Vec<String>,
    pub field_defaults: Vec<Option<Eval>>,
    pub methods: Vec<ClassMethod>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImportData {
    pub module: String,
    pub items: ImportItems,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchData {
    pub expr: Box<Eval>,
    pub cases: Vec<EvalMatchCase>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MacroData {
    pub macro_fn: Box<Eval>, // The macro function to call
    pub target: Box<Eval>,   // The Eval node to transform
    pub args: Vec<Eval>,     // Macro arguments
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionData {
    pub name: String,
    pub params: Vec<(String, Option<Value>, ParameterKind)>,
    pub body: Box<Eval>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LambdaData {
    pub params: Vec<String>,
    pub body: Box<Eval>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WithData {
    pub resources: Vec<(String, Eval)>,
    pub body: Box<Eval>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EvalPattern {
    Variable(String),
    List(Vec<EvalPattern>),
    ListRest(Vec<EvalPattern>, Option<String>),
    Dict(Vec<EvalDictPattern>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct EvalDictPattern {
    pub key: String,
    pub alias: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EvalMatchCase {
    pub pattern: EvalMatchPattern,
    pub guard: Option<Eval>,
    pub body: Eval,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EvalMatchPattern {
    Literal(Value),
    Variable(String),
    Wildcard,
}
