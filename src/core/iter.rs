use super::{DictRef, ListRef, Range, RustValue, Value};
use anyhow::Result;

#[derive(Debug)]
pub struct ListIter {
    list: ListRef,
    index: usize,
}

impl ListIter {
    pub fn new(list: ListRef) -> Self {
        Self { list, index: 0 }
    }
}

impl RustValue for ListIter {
    crate::impl_rust_value_any!(Self);
    fn op_next(&mut self) -> Result<Option<Value>> {
        let list = self.list.borrow();
        if self.index < list.len() {
            let value = list[self.index].clone();
            self.index += 1;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug)]
pub struct DictIter {
    dict: DictRef,
    keys: Vec<String>,
    index: usize,
}

impl DictIter {
    pub fn new(dict: DictRef) -> Self {
        let keys: Vec<String> = dict.borrow().keys().cloned().collect();
        Self {
            dict,
            keys,
            index: 0,
        }
    }
}

impl RustValue for DictIter {
    crate::impl_rust_value_any!(Self);
    fn op_next(&mut self) -> Result<Option<Value>> {
        while self.index < self.keys.len() {
            let key = &self.keys[self.index];
            let value = self.dict.borrow().get(key).cloned();
            self.index += 1;

            if value.is_some() {
                return Ok(Some(Value::String(key.clone())));
            }
            // Continue to next key if this one has no value
        }
        Ok(None)
    }
}

#[derive(Debug)]
pub struct RangeIter {
    current: i64,
    end: i64,
    inclusive: bool,
}

impl RangeIter {
    pub fn new(range: Range) -> Self {
        Self {
            current: range.start,
            end: range.end,
            inclusive: range.inclusive,
        }
    }
}

impl RustValue for RangeIter {
    crate::impl_rust_value_any!(Self);
    fn op_next(&mut self) -> Result<Option<Value>> {
        let should_continue = if self.inclusive {
            self.current <= self.end
        } else {
            self.current < self.end
        };

        if should_continue {
            let value = Value::Int(self.current);
            self.current += 1;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }
}
