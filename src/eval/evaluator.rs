/// Evaluator implementation for RustLeaf
use std::cell::RefCell;
use std::rc::Rc;

use super::scope::Scope;
use crate::core::*;
use anyhow::Result;

/// Evaluate a program and return the final value
pub fn evaluate(program: Program) -> Result<Value> {
    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program)
}

pub struct Evaluator {
    globals: Rc<RefCell<Scope>>,
    current_env: Rc<RefCell<Scope>>,
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}

impl Evaluator {
    pub fn new() -> Self {
        let globals = Rc::new(RefCell::new(Scope::new()));

        // TODO: Register built-in functions in globals
        // print, type, len, etc.

        Evaluator {
            globals: globals.clone(),
            current_env: globals,
        }
    }

    pub fn eval_program(&mut self, program: Program) -> Result<Value> {
        todo!()
    }
}
