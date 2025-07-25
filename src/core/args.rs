use anyhow::Result;
use indexmap::IndexMap;

use super::Value;

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
