use crate::core::{Args, RustValue, Value};
use crate::eval::{ControlFlow, ErrorKind, EvalResult, Evaluator};
use anyhow::anyhow;

use super::literals::EvalRef;

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
                let args = Args::positional(vec![index_value]);
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
        format!("{}[{}]", self.obj_expr.str(), self.index_expr.str())
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
                let args = Args::positional(vec![index_value, new_value]);
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
        format!(
            "{}[{}] = {}",
            self.obj_expr.str(),
            self.index_expr.str(),
            self.value_expr.str()
        )
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
        let elements_str = self
            .elements
            .iter()
            .map(|elem| elem.str())
            .collect::<Vec<_>>()
            .join(", ");
        format!("[{}]", elements_str)
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
        let pairs_str = self
            .pairs
            .iter()
            .map(|(key, value)| format!("{}: {}", key.str(), value.str()))
            .collect::<Vec<_>>()
            .join(", ");
        format!("{{{}}}", pairs_str)
    }
}
