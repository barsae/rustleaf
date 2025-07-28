/// RustValue wrapper for Eval nodes, used by the macro system
use crate::core::RustValue;
use crate::eval::{Eval, EvalResult, Evaluator};

#[derive(Debug, Clone)]
pub struct EvalNode {
    pub node: Eval,
}

impl EvalNode {
    pub fn new(node: Eval) -> Self {
        Self { node }
    }

    pub fn pretty_print(&self, _indent: usize) -> String {
        // TODO: This needs to be redesigned for the new trait-based Eval system
        // For now, use the trait object's str() method as a fallback
        self.node.0.str()
    }
}

impl RustValue for EvalNode {
    fn get_attr(&self, name: &str) -> Option<crate::core::Value> {
        use crate::core::Value;

        // Special attribute to report the node type for type checking
        if name == "node_type" {
            // TODO: Extract type information from trait object
            // For now, return a generic type
            return Some(Value::String("Eval".to_string()));
        }

        // TODO: This needs to be redesigned for the new trait-based Eval system
        // For now, we can't inspect the internal structure of trait objects
        // This functionality may need to be moved to individual eval structs
        None
    }

    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        Ok(evaluator.eval(&self.node))
    }

    fn str(&self) -> String {
        self.pretty_print(0)
    }
}
