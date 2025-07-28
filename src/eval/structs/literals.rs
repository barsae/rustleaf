use crate::core::{RustValue, Value};
use crate::eval::{ControlFlow, ErrorKind, EvalResult, Evaluator};
use anyhow::anyhow;
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
        self.value.str()
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
        self.name.clone()
    }
}
