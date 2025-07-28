/// Individual evaluation structs implementing RustValue trait
/// This replaces the large Eval enum with modular, extensible structs

use crate::core::{RustValue, Value};
use crate::eval::{EvalResult, Evaluator, ControlFlow, ErrorKind};
use anyhow::anyhow;
use std::rc::Rc;

// EvalRef - simplified ref pattern since Eval is immutable
#[derive(Clone, Debug)]
pub struct EvalRef(Rc<dyn RustValue>);

impl EvalRef {
    pub fn new<T: RustValue + 'static>(eval: T) -> Self {
        Self(Rc::new(eval))
    }
    
    pub fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        self.0.eval(evaluator)
    }
    
    pub fn str(&self) -> String {
        self.0.str()
    }
}

// Basic evaluation structs

#[derive(Debug, Clone)]
pub struct EvalLiteral {
    pub value: Value,
}

impl RustValue for EvalLiteral {
    fn eval(&self, _evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        Ok(Ok(self.value.clone()))
    }
    
    fn str(&self) -> String {
        format!("Literal({:?})", self.value)
    }
}

#[derive(Debug, Clone)]
pub struct EvalVariable {
    pub name: String,
}

impl RustValue for EvalVariable {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        let result = evaluator.current_env.get(&self.name).ok_or_else(|| {
            ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                "Undefined variable: {}",
                self.name
            )))
        });
        Ok(result)
    }
    
    fn str(&self) -> String {
        format!("Variable({})", self.name)
    }
}

#[derive(Debug, Clone)]
pub struct EvalCall {
    pub func_expr: EvalRef,
    pub args: Vec<EvalRef>,
}

impl RustValue for EvalCall {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        // Get the function value
        let func_result = self.func_expr.eval(evaluator)?;
        let func_value = match func_result {
            Ok(val) => val,
            Err(e) => return Ok(Err(e)),
        };

        // Handle class constructor calls
        if let Value::Class(class_ref) = &func_value {
            let class = class_ref.borrow();
            
            // Evaluate arguments for constructor
            let mut arg_evals = Vec::new();
            for arg in &self.args {
                let arg_result = arg.eval(evaluator)?;
                match arg_result {
                    Ok(val) => {
                        // Convert back to old Eval for compatibility
                        // TODO: Remove this conversion once we fully migrate
                        arg_evals.push(crate::eval::Eval::literal(val));
                    },
                    Err(e) => return Ok(Err(e)),
                }
            }
            
            return Ok(evaluator.handle_class_constructor(&class, &arg_evals));
        }

        // Evaluate all arguments
        let mut arg_values = Vec::new();
        for arg in &self.args {
            let arg_result = arg.eval(evaluator)?;
            match arg_result {
                Ok(val) => arg_values.push(val),
                Err(e) => return Ok(Err(e)),
            }
        }

        let args_obj = crate::core::Args::positional(arg_values);

        let result = func_value
            .call(args_obj)
            .map_err(|e| ControlFlow::Error(ErrorKind::SystemError(e)));

        match result {
            Ok(value) => {
                if let Value::Raised(error_value) = value {
                    return Ok(Err(ControlFlow::Error(ErrorKind::RaisedError(*error_value))));
                }
                Ok(Ok(value))
            }
            Err(control_flow) => Ok(Err(control_flow)),
        }
    }
    
    fn str(&self) -> String {
        format!("Call({}, {} args)", self.func_expr.str(), self.args.len())
    }
}

#[derive(Debug, Clone)]
pub struct EvalBlock {
    pub statements: Vec<EvalRef>,
    pub final_expr: Option<EvalRef>,
}

impl RustValue for EvalBlock {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        let block_scope = evaluator.current_env.child();
        let previous_env = std::mem::replace(&mut evaluator.current_env, block_scope);

        let mut result = Value::Unit;

        // Execute all statements
        for stmt in &self.statements {
            match stmt.eval(evaluator)? {
                Ok(_) => {}
                Err(e) => {
                    evaluator.current_env = previous_env;
                    return Ok(Err(e));
                }
            }
        }

        // Evaluate final expression if present
        if let Some(final_expr) = &self.final_expr {
            result = match final_expr.eval(evaluator)? {
                Ok(val) => val,
                Err(e) => {
                    evaluator.current_env = previous_env;
                    return Ok(Err(e));
                }
            };
        }

        evaluator.current_env = previous_env;
        Ok(Ok(result))
    }
    
    fn str(&self) -> String {
        format!("Block({} statements, final_expr: {})", 
                self.statements.len(), 
                self.final_expr.is_some())
    }
}

#[derive(Debug, Clone)]
pub struct EvalProgram {
    pub statements: Vec<EvalRef>,
}

impl RustValue for EvalProgram {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        for stmt in &self.statements {
            match stmt.eval(evaluator)? {
                Ok(_) => {}
                Err(e) => return Ok(Err(e)),
            }
        }
        Ok(Ok(Value::Unit))
    }
    
    fn str(&self) -> String {
        format!("Program({} statements)", self.statements.len())
    }
}

#[derive(Debug, Clone)]
pub struct EvalDeclare {
    pub name: String,
    pub init_expr: Option<EvalRef>,
}

impl RustValue for EvalDeclare {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        let value = match &self.init_expr {
            Some(expr) => {
                match expr.eval(evaluator)? {
                    Ok(val) => val,
                    Err(e) => return Ok(Err(e)),
                }
            }
            None => Value::Unit,
        };
        evaluator.current_env.define(&self.name, value);
        Ok(Ok(Value::Unit))
    }
    
    fn str(&self) -> String {
        format!("Declare({}, has_init: {})", self.name, self.init_expr.is_some())
    }
}

// Control flow structs

#[derive(Debug, Clone)]
pub struct EvalIf {
    pub condition: EvalRef,
    pub then_expr: EvalRef,
    pub else_expr: Option<EvalRef>,
}

impl RustValue for EvalIf {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        let condition_result = self.condition.eval(evaluator)?;
        let condition_val = match condition_result {
            Ok(val) => val,
            Err(e) => return Ok(Err(e)),
        };

        if condition_val.is_truthy() {
            self.then_expr.eval(evaluator)
        } else {
            match &self.else_expr {
                Some(expr) => expr.eval(evaluator),
                None => Ok(Ok(Value::Unit)),
            }
        }
    }
    
    fn str(&self) -> String {
        format!("If(has_else: {})", self.else_expr.is_some())
    }
}

#[derive(Debug, Clone)]
pub struct EvalReturn {
    pub expr: Option<EvalRef>,
}

impl RustValue for EvalReturn {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        let value = match &self.expr {
            Some(e) => {
                match e.eval(evaluator)? {
                    Ok(val) => val,
                    Err(e) => return Ok(Err(e)),
                }
            }
            None => Value::Unit,
        };
        Ok(Err(ControlFlow::Return(value)))
    }
    
    fn str(&self) -> String {
        format!("Return(has_expr: {})", self.expr.is_some())
    }
}

#[derive(Debug, Clone)]
pub struct EvalBreak {
    pub expr: Option<EvalRef>,
}

impl RustValue for EvalBreak {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        let value = match &self.expr {
            Some(e) => {
                match e.eval(evaluator)? {
                    Ok(val) => val,
                    Err(e) => return Ok(Err(e)),
                }
            }
            None => Value::Unit,
        };
        Ok(Err(ControlFlow::Break(value)))
    }
    
    fn str(&self) -> String {
        format!("Break(has_expr: {})", self.expr.is_some())
    }
}

#[derive(Debug, Clone)]
pub struct EvalContinue;

impl RustValue for EvalContinue {
    fn eval(&self, _evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        Ok(Err(ControlFlow::Continue))
    }
    
    fn str(&self) -> String {
        "Continue".to_string()
    }
}

// Additional control flow and operations

#[derive(Debug, Clone)]
pub struct EvalAssign {
    pub name: String,
    pub expr: EvalRef,
}

impl RustValue for EvalAssign {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        let value = match self.expr.eval(evaluator)? {
            Ok(val) => val,
            Err(e) => return Ok(Err(e)),
        };
        match evaluator.current_env.set(&self.name, value) {
            Ok(_) => Ok(Ok(Value::Unit)),
            Err(err) => Ok(Err(ControlFlow::Error(ErrorKind::SystemError(anyhow!(err))))),
        }
    }
    
    fn str(&self) -> String {
        format!("Assign({}, {})", self.name, self.expr.str())
    }
}

#[derive(Debug, Clone)]
pub struct EvalLoop {
    pub body: EvalRef,
}

impl RustValue for EvalLoop {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        loop {
            match self.body.eval(evaluator)? {
                Ok(_) => continue,
                Err(ControlFlow::Break(value)) => return Ok(Ok(value)),
                Err(ControlFlow::Continue) => continue,
                Err(other) => return Ok(Err(other)),
            }
        }
    }
    
    fn str(&self) -> String {
        format!("Loop({})", self.body.str())
    }
}

#[derive(Debug, Clone)]
pub struct EvalWhile {
    pub condition: EvalRef,
    pub body: EvalRef,
}

impl RustValue for EvalWhile {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        loop {
            let condition_val = match self.condition.eval(evaluator)? {
                Ok(val) => val,
                Err(e) => return Ok(Err(e)),
            };

            if !condition_val.is_truthy() {
                return Ok(Ok(Value::Unit));
            }

            match self.body.eval(evaluator)? {
                Ok(_) => continue,
                Err(ControlFlow::Break(value)) => return Ok(Ok(value)),
                Err(ControlFlow::Continue) => continue,
                Err(other) => return Ok(Err(other)),
            }
        }
    }
    
    fn str(&self) -> String {
        format!("While({}, {})", self.condition.str(), self.body.str())
    }
}

#[derive(Debug, Clone)]
pub struct EvalFor {
    pub var_name: String,
    pub iter_expr: EvalRef,
    pub body: EvalRef,
}

impl RustValue for EvalFor {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        let iter_value = match self.iter_expr.eval(evaluator)? {
            Ok(val) => val,
            Err(e) => return Ok(Err(e)),
        };

        let loop_scope = evaluator.current_env.child();
        let previous_env = std::mem::replace(&mut evaluator.current_env, loop_scope);

        let mut result = Value::Unit;

        let mut iterator = match iter_value.op_iter() {
            Ok(iter) => iter,
            Err(e) => {
                evaluator.current_env = previous_env;
                return Ok(Err(ControlFlow::Error(ErrorKind::SystemError(e))));
            }
        };

        loop {
            let next_item = match iterator.op_next() {
                Ok(item) => item,
                Err(e) => {
                    evaluator.current_env = previous_env;
                    return Ok(Err(ControlFlow::Error(ErrorKind::SystemError(e))));
                }
            };

            match next_item {
                Some(item) => {
                    evaluator.current_env.define(&self.var_name, item);

                    match self.body.eval(evaluator)? {
                        Ok(_) => {}
                        Err(ControlFlow::Break(value)) => {
                            result = value;
                            break;
                        }
                        Err(ControlFlow::Continue) => continue,
                        Err(other) => {
                            evaluator.current_env = previous_env;
                            return Ok(Err(other));
                        }
                    }
                }
                None => break,
            }
        }

        evaluator.current_env = previous_env;
        Ok(Ok(result))
    }
    
    fn str(&self) -> String {
        format!("For({}, {}, {})", self.var_name, self.iter_expr.str(), self.body.str())
    }
}

// Logical operations

#[derive(Debug, Clone)]
pub struct EvalLogicalAnd {
    pub left: EvalRef,
    pub right: EvalRef,
}

impl RustValue for EvalLogicalAnd {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        let left_val = match self.left.eval(evaluator)? {
            Ok(val) => val,
            Err(e) => return Ok(Err(e)),
        };
        if !left_val.is_truthy() {
            Ok(Ok(left_val))
        } else {
            self.right.eval(evaluator)
        }
    }
    
    fn str(&self) -> String {
        format!("LogicalAnd({}, {})", self.left.str(), self.right.str())
    }
}

#[derive(Debug, Clone)]
pub struct EvalLogicalOr {
    pub left: EvalRef,
    pub right: EvalRef,
}

impl RustValue for EvalLogicalOr {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        let left_val = match self.left.eval(evaluator)? {
            Ok(val) => val,
            Err(e) => return Ok(Err(e)),
        };
        if left_val.is_truthy() {
            Ok(Ok(left_val))
        } else {
            self.right.eval(evaluator)
        }
    }
    
    fn str(&self) -> String {
        format!("LogicalOr({}, {})", self.left.str(), self.right.str())
    }
}

#[derive(Debug, Clone)]
pub struct EvalLogicalNot {
    pub expr: EvalRef,
}

impl RustValue for EvalLogicalNot {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        let val = match self.expr.eval(evaluator)? {
            Ok(val) => val,
            Err(e) => return Ok(Err(e)),
        };
        Ok(Ok(Value::Bool(!val.is_truthy())))
    }
    
    fn str(&self) -> String {
        format!("LogicalNot({})", self.expr.str())
    }
}

#[derive(Debug, Clone)]
pub struct EvalIs {
    pub left: EvalRef,
    pub right: EvalRef,
}

impl RustValue for EvalIs {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        let left_val = match self.left.eval(evaluator)? {
            Ok(val) => val,
            Err(e) => return Ok(Err(e)),
        };
        let right_val = match self.right.eval(evaluator)? {
            Ok(val) => val,
            Err(e) => return Ok(Err(e)),
        };

        if let Value::RustValue(rust_val_ref) = &right_val {
            let rust_val = rust_val_ref.borrow();
            match rust_val.op_is(&left_val) {
                Ok(result) => return Ok(Ok(result)),
                Err(_) => {}
            }
        }

        Ok(Ok(Value::Bool(left_val == right_val)))
    }
    
    fn str(&self) -> String {
        format!("Is({}, {})", self.left.str(), self.right.str())
    }
}

// Attribute and item access

#[derive(Debug, Clone)]
pub struct EvalGetAttr {
    pub obj_expr: EvalRef,
    pub attr_name: String,
}

impl RustValue for EvalGetAttr {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        let obj_value = match self.obj_expr.eval(evaluator)? {
            Ok(val) => val,
            Err(e) => return Ok(Err(e)),
        };

        match obj_value.get_attr(&self.attr_name, evaluator) {
            Some(value) => Ok(Ok(value)),
            None => Ok(Err(ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                "No attribute '{}' on value {:?}",
                self.attr_name,
                obj_value
            ))))),
        }
    }
    
    fn str(&self) -> String {
        format!("GetAttr({}, {})", self.obj_expr.str(), self.attr_name)
    }
}

#[derive(Debug, Clone)]
pub struct EvalSetAttr {
    pub obj_expr: EvalRef,
    pub attr_name: String,
    pub value_expr: EvalRef,
}

impl RustValue for EvalSetAttr {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        let obj_value = match self.obj_expr.eval(evaluator)? {
            Ok(val) => val,
            Err(e) => return Ok(Err(e)),
        };
        let new_value = match self.value_expr.eval(evaluator)? {
            Ok(val) => val,
            Err(e) => return Ok(Err(e)),
        };

        match obj_value.set_attr(&self.attr_name, new_value) {
            Ok(_) => Ok(Ok(Value::Unit)),
            Err(err) => Ok(Err(ControlFlow::Error(ErrorKind::SystemError(anyhow!(err))))),
        }
    }
    
    fn str(&self) -> String {
        format!("SetAttr({}, {}, {})", self.obj_expr.str(), self.attr_name, self.value_expr.str())
    }
}

#[derive(Debug, Clone)]
pub struct EvalGetItem {
    pub obj_expr: EvalRef,
    pub index_expr: EvalRef,
}

impl RustValue for EvalGetItem {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        let obj_value = match self.obj_expr.eval(evaluator)? {
            Ok(val) => val,
            Err(e) => return Ok(Err(e)),
        };
        let index_value = match self.index_expr.eval(evaluator)? {
            Ok(val) => val,
            Err(e) => return Ok(Err(e)),
        };

        match obj_value.get_attr("op_get_item", evaluator) {
            Some(method) => {
                let args = crate::core::Args::positional(vec![index_value]);
                match method.call(args) {
                    Ok(result) => Ok(Ok(result)),
                    Err(e) => Ok(Err(ControlFlow::Error(ErrorKind::SystemError(e)))),
                }
            }
            None => Ok(Err(ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                "No op_get_item method on value {:?}",
                obj_value
            ))))),
        }
    }
    
    fn str(&self) -> String {
        format!("GetItem({}, {})", self.obj_expr.str(), self.index_expr.str())
    }
}

#[derive(Debug, Clone)]
pub struct EvalSetItem {
    pub obj_expr: EvalRef,
    pub index_expr: EvalRef,
    pub value_expr: EvalRef,
}

impl RustValue for EvalSetItem {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        let obj_value = match self.obj_expr.eval(evaluator)? {
            Ok(val) => val,
            Err(e) => return Ok(Err(e)),
        };
        let index_value = match self.index_expr.eval(evaluator)? {
            Ok(val) => val,
            Err(e) => return Ok(Err(e)),
        };
        let new_value = match self.value_expr.eval(evaluator)? {
            Ok(val) => val,
            Err(e) => return Ok(Err(e)),
        };

        match obj_value.get_attr("op_set_item", evaluator) {
            Some(method) => {
                let args = crate::core::Args::positional(vec![index_value, new_value]);
                match method.call(args) {
                    Ok(_) => Ok(Ok(Value::Unit)),
                    Err(e) => Ok(Err(ControlFlow::Error(ErrorKind::SystemError(e)))),
                }
            }
            None => Ok(Err(ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                "No op_set_item method on value {:?}",
                obj_value
            ))))),
        }
    }
    
    fn str(&self) -> String {
        format!("SetItem({}, {}, {})", self.obj_expr.str(), self.index_expr.str(), self.value_expr.str())
    }
}

// Collections

#[derive(Debug, Clone)]
pub struct EvalList {
    pub elements: Vec<EvalRef>,
}

impl RustValue for EvalList {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        let mut list_values = Vec::new();
        for element in &self.elements {
            match element.eval(evaluator)? {
                Ok(val) => list_values.push(val),
                Err(e) => return Ok(Err(e)),
            }
        }
        Ok(Ok(Value::new_list_with_values(list_values)))
    }
    
    fn str(&self) -> String {
        format!("List({} elements)", self.elements.len())
    }
}

#[derive(Debug, Clone)]
pub struct EvalDict {
    pub pairs: Vec<(EvalRef, EvalRef)>,
}

impl RustValue for EvalDict {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        let mut dict_map = indexmap::IndexMap::new();
        for (key_expr, value_expr) in &self.pairs {
            let key_val = match key_expr.eval(evaluator)? {
                Ok(val) => val,
                Err(e) => return Ok(Err(e)),
            };
            let value_val = match value_expr.eval(evaluator)? {
                Ok(val) => val,
                Err(e) => return Ok(Err(e)),
            };

            let key_str = match key_val {
                Value::String(s) => s,
                Value::Int(i) => i.to_string(),
                Value::Float(f) => f.to_string(),
                Value::Bool(b) => b.to_string(),
                _ => {
                    return Ok(Err(ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                        "Dictionary keys must be strings, numbers, or booleans, got {:?}",
                        key_val
                    )))))
                }
            };

            dict_map.insert(key_str, value_val);
        }
        Ok(Ok(Value::new_dict_with_map(dict_map)))
    }
    
    fn str(&self) -> String {
        format!("Dict({} pairs)", self.pairs.len())
    }
}

// Advanced evaluation structs

#[derive(Debug, Clone)]
pub struct EvalFunction {
    pub data: crate::eval::core::FunctionData,
}

impl RustValue for EvalFunction {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        use crate::eval::{RustLeafFunction, Params};
        
        let params = Params::from_vec(self.data.params.clone());
        let function = RustLeafFunction::new(
            params,
            self.data.body.as_ref().clone(),
            evaluator.current_env.clone(),
        );
        
        let function_value = Value::from_rust(function);
        evaluator.current_env.define(&self.data.name, function_value);
        Ok(Ok(Value::Unit))
    }
    
    fn str(&self) -> String {
        format!("Function({}, {} params)", self.data.name, self.data.params.len())
    }
}

#[derive(Debug, Clone)]
pub struct EvalLambda {
    pub data: crate::eval::core::LambdaData,
}

impl RustValue for EvalLambda {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        use crate::eval::{RustLeafFunction, Params};
        
        let params = Params::from_names(&self.data.params);
        let lambda_fn = RustLeafFunction::new(
            params,
            self.data.body.as_ref().clone(),
            evaluator.current_env.clone(),
        );
        
        Ok(Ok(Value::from_rust(lambda_fn)))
    }
    
    fn str(&self) -> String {
        format!("Lambda({} params)", self.data.params.len())
    }
}

#[derive(Debug, Clone)]
pub struct EvalClassDecl {
    pub data: crate::eval::core::ClassDeclData,
}

impl RustValue for EvalClassDecl {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        use crate::eval::Class;
        
        let class = Class {
            name: self.data.name.clone(),
            field_names: self.data.field_names.clone(),
            field_defaults: self.data.field_defaults.clone(),
            methods: self.data.methods.clone(),
        };
        
        let class_value = Value::new_class(class);
        evaluator.current_env.define(&self.data.name, class_value);
        Ok(Ok(Value::Unit))
    }
    
    fn str(&self) -> String {
        format!("ClassDecl({}, {} fields)", self.data.name, self.data.field_names.len())
    }
}

#[derive(Debug, Clone)]
pub struct EvalImport {
    pub data: crate::eval::core::ImportData,
}

impl RustValue for EvalImport {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        use crate::core::ImportItems;
        
        match evaluator.load_module(&self.data.module) {
            Ok(module_scope) => {
                match &self.data.items {
                    ImportItems::All => {
                        // Import all public items from module
                        let module_vars = module_scope.iter();
                        for (name, value) in module_vars {
                            // Skip built-in functions (they start with certain patterns)
                            if !name.starts_with("__") {
                                evaluator.current_env.define(&name, value);
                            }
                        }
                    }
                    ImportItems::Specific(items) => {
                        // Import only selected items
                        for import_item in items {
                            let item_name = &import_item.name;
                            if let Some(value) = module_scope.get(item_name) {
                                let local_name = import_item.alias.as_ref().unwrap_or(item_name);
                                evaluator.current_env.define(local_name, value);
                            } else {
                                return Ok(Err(ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                                    "Item '{}' not found in module '{}'",
                                    item_name,
                                    self.data.module
                                )))));
                            }
                        }
                    }
                }
                Ok(Ok(Value::Unit))
            }
            Err(e) => Ok(Err(e)),
        }
    }
    
    fn str(&self) -> String {
        format!("Import({})", self.data.module)
    }
}

#[derive(Debug, Clone)]
pub struct EvalMatch {
    pub data: crate::eval::core::MatchData,
}

impl RustValue for EvalMatch {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        // Evaluate the match expression
        let match_value = match self.data.expr.eval(evaluator)? {
            Ok(val) => val,
            Err(e) => return Ok(Err(e)),
        };
        
        // Try each case in order
        for case in &self.data.cases {
            // Check if pattern matches
            match evaluator.match_pattern_matches(&case.pattern, &match_value) {
                Ok(true) => {
                    // Check guard condition if present
                    if let Some(guard) = &case.guard {
                        let guard_result = match guard.eval(evaluator)? {
                            Ok(val) => val,
                            Err(e) => return Ok(Err(e)),
                        };
                        if !guard_result.is_truthy() {
                            continue;
                        }
                    }
                    
                    // Pattern matches (and guard passes), execute body
                    // For Variable patterns, bind the variable
                    if let crate::eval::core::EvalMatchPattern::Variable(var_name) = &case.pattern {
                        evaluator.current_env.define(var_name.clone(), match_value);
                    }
                    
                    return case.body.eval(evaluator);
                }
                Ok(false) => continue,
                Err(e) => return Ok(Err(e)),
            }
        }
        
        // No pattern matched
        Ok(Err(ControlFlow::Error(ErrorKind::SystemError(anyhow!(
            "No pattern matched in match expression"
        )))))
    }
    
    fn str(&self) -> String {
        format!("Match({} cases)", self.data.cases.len())
    }
}

#[derive(Debug, Clone)]
pub struct EvalMacro {
    pub data: crate::eval::core::MacroData,
}

impl RustValue for EvalMacro {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        // Get the macro function
        let macro_fn_result = self.data.macro_fn.eval(evaluator)?;
        let macro_fn = match macro_fn_result {
            Ok(val) => val,
            Err(e) => return Ok(Err(e)),
        };
        
        // Evaluate macro arguments
        let mut arg_values = Vec::new();
        for arg in &self.data.args {
            let arg_result = arg.eval(evaluator)?;
            match arg_result {
                Ok(val) => arg_values.push(val),
                Err(e) => return Ok(Err(e)),
            }
        }
        
        // Add the target Eval as a special argument
        let target_eval_value = Value::from_rust(crate::eval::EvalTypeConstant::new());
        arg_values.insert(0, target_eval_value);
        
        // Call the macro function
        let args_obj = crate::core::Args::positional(arg_values);
        match macro_fn.call(args_obj) {
            Ok(result) => {
                // For now, just return the result directly
                // TODO: Properly handle macro expansion when EvalTypeConstant is enhanced
                Ok(Ok(result))
            }
            Err(e) => Ok(Err(ControlFlow::Error(ErrorKind::SystemError(e)))),
        }
    }
    
    fn str(&self) -> String {
        format!("Macro({} args)", self.data.args.len())
    }
}

#[derive(Debug, Clone)]
pub struct EvalWith {
    pub data: crate::eval::core::WithData,
}

impl RustValue for EvalWith {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        let mut resources = Vec::new();
        
        // Acquire all resources
        for (resource_name, resource_expr) in &self.data.resources {
            let resource_value = match resource_expr.eval(evaluator)? {
                Ok(val) => val,
                Err(e) => {
                    // Cleanup any already acquired resources
                    evaluator.cleanup_resources(&resources);
                    return Ok(Err(e));
                }
            };
            
            // Define the resource in current scope
            evaluator.current_env.define(resource_name.clone(), resource_value.clone());
            resources.push((resource_name.clone(), resource_value));
        }
        
        // Execute the body
        let body_result = self.data.body.eval(evaluator)?;
        
        // Always cleanup resources
        evaluator.cleanup_resources(&resources);
        
        Ok(body_result)
    }
    
    fn str(&self) -> String {
        format!("With({} resources)", self.data.resources.len())
    }
}