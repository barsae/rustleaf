use anyhow::Result;
use indexmap::IndexMap;

use super::{RustValue, Value};

/// Arguments for function calls with fluent API for easy consumption
#[derive(Debug, Clone)]
pub struct Args {
    positional: Vec<Value>,
    keywords: IndexMap<String, Value>,
    function_name: Option<String>,
    positional_index: usize,
    consumed_keywords: Vec<String>,
}

impl Args {
    pub fn new(positional: Vec<Value>, keywords: IndexMap<String, Value>) -> Self {
        Self {
            positional,
            keywords,
            function_name: None,
            positional_index: 0,
            consumed_keywords: Vec::new(),
        }
    }

    pub fn positional(positional: Vec<Value>) -> Self {
        Self::new(positional, IndexMap::new())
    }

    /// Set the function name for error messages
    pub fn set_function_name(&mut self, name: &str) {
        self.function_name = Some(name.to_string());
    }

    /// Get the next positional argument with a descriptive name
    pub fn expect(&mut self, name: &str) -> Result<Value> {
        let function_name = self.function_name.as_deref().unwrap_or("function");

        if self.positional_index >= self.positional.len() {
            return Err(anyhow::anyhow!(
                "{}() missing required argument '{}'",
                function_name,
                name
            ));
        }

        let value = self.positional[self.positional_index].clone();
        self.positional_index += 1;
        Ok(value)
    }

    /// Get an optional positional argument with a default value
    pub fn optional(&mut self, _name: &str, default: Value) -> Value {
        if self.positional_index >= self.positional.len() {
            default
        } else {
            let value = self.positional[self.positional_index].clone();
            self.positional_index += 1;
            value
        }
    }

    /// Get a keyword argument by name
    pub fn keyword(&mut self, name: &str) -> Result<Value> {
        let function_name = self.function_name.as_deref().unwrap_or("function");

        if let Some(value) = self.keywords.get(name) {
            self.consumed_keywords.push(name.to_string());
            Ok(value.clone())
        } else {
            Err(anyhow::anyhow!(
                "{}() missing required keyword argument '{}'",
                function_name,
                name
            ))
        }
    }

    /// Get an optional keyword argument with a default value
    pub fn keyword_optional(&mut self, name: &str, default: Value) -> Value {
        if let Some(value) = self.keywords.get(name) {
            self.consumed_keywords.push(name.to_string());
            value.clone()
        } else {
            default
        }
    }

    /// Consume all remaining positional arguments
    pub fn rest(&mut self) -> Vec<Value> {
        let remaining = self.positional[self.positional_index..].to_vec();
        self.positional_index = self.positional.len();
        remaining
    }

    /// Check that all arguments have been consumed
    pub fn complete(&self) -> Result<()> {
        let function_name = self.function_name.as_deref().unwrap_or("function");

        // Check for unconsumed positional arguments
        if self.positional_index < self.positional.len() {
            let remaining = self.positional.len() - self.positional_index;
            return Err(anyhow::anyhow!(
                "{}() got {} unexpected positional argument{}",
                function_name,
                remaining,
                if remaining == 1 { "" } else { "s" }
            ));
        }

        // Check for unconsumed keyword arguments
        let unconsumed_keywords: Vec<&String> = self
            .keywords
            .keys()
            .filter(|k| !self.consumed_keywords.contains(k))
            .collect();

        if !unconsumed_keywords.is_empty() {
            return Err(anyhow::anyhow!(
                "{}() got unexpected keyword argument{}: {}",
                function_name,
                if unconsumed_keywords.len() == 1 {
                    ""
                } else {
                    "s"
                },
                unconsumed_keywords
                    .into_iter()
                    .map(|s| format!("'{}'", s))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }

        Ok(())
    }

    /// Get the total number of arguments provided
    pub fn len(&self) -> usize {
        self.positional.len() + self.keywords.len()
    }

    /// Check if no arguments were provided
    pub fn is_empty(&self) -> bool {
        self.positional.is_empty() && self.keywords.is_empty()
    }
}

impl From<Vec<Value>> for Args {
    fn from(positional: Vec<Value>) -> Self {
        Self::positional(positional)
    }
}

/// Helper functions for common Args patterns in RustValue implementations
impl Args {
    /// Handle no-argument methods - sets function name and validates no args
    pub fn no_args(&mut self, function_name: &str) -> Result<()> {
        self.set_function_name(function_name);
        self.complete()
    }

    /// Handle single f64 argument methods
    pub fn single_f64(&mut self, function_name: &str, arg_name: &str) -> Result<f64> {
        self.set_function_name(function_name);
        let value = self.expect(arg_name)?;
        self.complete()?;
        value.expect_f64(function_name, arg_name)
    }

    /// Handle single i64 argument methods
    pub fn single_i64(&mut self, function_name: &str, arg_name: &str) -> Result<i64> {
        self.set_function_name(function_name);
        let value = self.expect(arg_name)?;
        self.complete()?;
        value.expect_i64(function_name, arg_name)
    }

    /// Handle single bool argument methods
    pub fn single_bool(&mut self, function_name: &str, arg_name: &str) -> Result<bool> {
        self.set_function_name(function_name);
        let value = self.expect(arg_name)?;
        self.complete()?;
        value.expect_bool(function_name, arg_name)
    }

    /// Handle single string argument methods  
    pub fn single_string(&mut self, function_name: &str, arg_name: &str) -> Result<String> {
        self.set_function_name(function_name);
        let value = self.expect(arg_name)?;
        self.complete()?;
        Ok(value.expect_string(function_name, arg_name)?.to_string())
    }

    /// Handle single RustValue argument methods with type checking
    pub fn single_rust_value<T: RustValue + 'static>(
        &mut self,
        function_name: &str,
        arg_name: &str,
        expected_type: &str,
    ) -> Result<Value> {
        self.set_function_name(function_name);
        let value = self.expect(arg_name)?;
        self.complete()?;

        // Validate that it's the expected RustValue type
        value.expect_rust_value::<T>(function_name, expected_type)?;
        Ok(value)
    }

    /// Handle two f64 argument methods
    pub fn two_f64(
        &mut self,
        function_name: &str,
        arg1_name: &str,
        arg2_name: &str,
    ) -> Result<(f64, f64)> {
        self.set_function_name(function_name);
        let value1 = self.expect(arg1_name)?;
        let value2 = self.expect(arg2_name)?;
        self.complete()?;
        let arg1 = value1.expect_f64(function_name, arg1_name)?;
        let arg2 = value2.expect_f64(function_name, arg2_name)?;
        Ok((arg1, arg2))
    }
}
