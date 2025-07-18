use crate::value::types::{RustValue, Value};

// Example implementation of RustValue for demonstration
#[derive(Debug, Clone)]
pub struct ExampleRustValue {
    pub data: String,
}

impl RustValue for ExampleRustValue {
    fn type_name(&self) -> &'static str {
        "ExampleRustValue"
    }

    fn to_string(&self) -> String {
        format!("ExampleRustValue({})", self.data)
    }

    fn clone_box(&self) -> Box<dyn RustValue> {
        Box::new(self.clone())
    }
}

// This would be the main trait that Rust types implement to be usable in RustLeaf
pub trait IntoRustLeafValue {
    fn into_rustleaf_value(self) -> Value;
}

// Example implementations
impl IntoRustLeafValue for String {
    fn into_rustleaf_value(self) -> Value {
        Value::String(self)
    }
}

impl IntoRustLeafValue for i32 {
    fn into_rustleaf_value(self) -> Value {
        Value::Int(self as i64)
    }
}

impl IntoRustLeafValue for i64 {
    fn into_rustleaf_value(self) -> Value {
        Value::Int(self)
    }
}

impl IntoRustLeafValue for f64 {
    fn into_rustleaf_value(self) -> Value {
        Value::Float(self)
    }
}

impl IntoRustLeafValue for bool {
    fn into_rustleaf_value(self) -> Value {
        Value::Bool(self)
    }
}

// Lazy Range Iterator implementation
#[derive(Debug, Clone)]
pub struct RangeIterator {
    pub start: i64,
    pub end: i64,
    pub step: i64,
    pub current: i64,
    pub finished: bool,
}

impl RangeIterator {
    pub fn new(start: i64, end: i64, step: i64) -> Self {
        let finished = if step > 0 { start >= end } else { start <= end };

        RangeIterator {
            start,
            end,
            step,
            current: start,
            finished,
        }
    }

    pub fn iter_next_value(&mut self) -> Option<i64> {
        if self.finished {
            return None;
        }

        let value = self.current;
        self.current += self.step;

        // Check if we should finish on the next call
        self.finished = if self.step > 0 {
            self.current >= self.end
        } else {
            self.current <= self.end
        };

        Some(value)
    }

    pub fn len(&self) -> i64 {
        if self.finished && self.current == self.start {
            // Empty range
            return 0;
        }

        if self.step > 0 {
            if self.start >= self.end {
                0
            } else {
                (self.end - self.start + self.step - 1) / self.step
            }
        } else if self.start <= self.end {
            0
        } else {
            (self.start - self.end - self.step - 1) / (-self.step)
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl RustValue for RangeIterator {
    fn type_name(&self) -> &'static str {
        "range"
    }

    fn to_string(&self) -> String {
        format!("range({}, {}, {})", self.start, self.end, self.step)
    }

    fn clone_box(&self) -> Box<dyn RustValue> {
        Box::new(self.clone())
    }

    fn iter_next(&mut self) -> Option<Value> {
        self.iter_next_value().map(Value::Int)
    }

    fn iter_reset(&mut self) {
        self.current = self.start;
        self.finished = if self.step > 0 {
            self.start >= self.end
        } else {
            self.start <= self.end
        };
    }

    fn is_iterable(&self) -> bool {
        true
    }

    fn len(&self) -> Option<i64> {
        Some(RangeIterator::len(self))
    }
}
