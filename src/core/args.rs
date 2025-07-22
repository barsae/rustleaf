use indexmap::IndexMap;
use anyhow::Result;
use std::cell::RefCell;

use super::Value;

/// Arguments for function calls with fluent API for easy consumption
#[derive(Debug, Clone)]
pub struct Args {
    positional: Vec<Value>,
    keywords: IndexMap<String, Value>,
    // Mutable state for consumption tracking
    state: RefCell<ArgsState>,
}

#[derive(Debug, Clone)]
struct ArgsState {
    function_name: Option<String>,
    positional_index: usize,
    consumed_keywords: Vec<String>,
}

impl Args {
    pub fn new(positional: Vec<Value>, keywords: IndexMap<String, Value>) -> Self {
        Self { 
            positional, 
            keywords,
            state: RefCell::new(ArgsState {
                function_name: None,
                positional_index: 0,
                consumed_keywords: Vec::new(),
            }),
        }
    }

    pub fn positional(positional: Vec<Value>) -> Self {
        Self::new(positional, IndexMap::new())
    }

    /// Set the function name for error messages
    pub fn set_function_name(&self, name: &str) {
        self.state.borrow_mut().function_name = Some(name.to_string());
    }

    /// Get the next positional argument with a descriptive name
    pub fn expect(&self, name: &str) -> Result<Value> {
        let mut state = self.state.borrow_mut();
        let function_name = state.function_name.as_deref().unwrap_or("function");
        
        if state.positional_index >= self.positional.len() {
            return Err(anyhow::anyhow!(
                "{}() missing required argument '{}'",
                function_name, name
            ));
        }

        let value = self.positional[state.positional_index].clone();
        state.positional_index += 1;
        Ok(value)
    }

    /// Get an optional positional argument with a default value
    pub fn optional(&self, name: &str, default: Value) -> Value {
        let mut state = self.state.borrow_mut();
        
        if state.positional_index >= self.positional.len() {
            default
        } else {
            let value = self.positional[state.positional_index].clone();
            state.positional_index += 1;
            value
        }
    }

    /// Get a keyword argument by name
    pub fn keyword(&self, name: &str) -> Result<Value> {
        let mut state = self.state.borrow_mut();
        let function_name = state.function_name.as_deref().unwrap_or("function");
        
        if let Some(value) = self.keywords.get(name) {
            state.consumed_keywords.push(name.to_string());
            Ok(value.clone())
        } else {
            Err(anyhow::anyhow!(
                "{}() missing required keyword argument '{}'",
                function_name, name
            ))
        }
    }

    /// Get an optional keyword argument with a default value
    pub fn keyword_optional(&self, name: &str, default: Value) -> Value {
        let mut state = self.state.borrow_mut();
        
        if let Some(value) = self.keywords.get(name) {
            state.consumed_keywords.push(name.to_string());
            value.clone()
        } else {
            default
        }
    }

    /// Consume all remaining positional arguments
    pub fn rest(&self) -> Vec<Value> {
        let mut state = self.state.borrow_mut();
        let remaining = self.positional[state.positional_index..].to_vec();
        state.positional_index = self.positional.len();
        remaining
    }

    /// Check that all arguments have been consumed
    pub fn complete(&self) -> Result<()> {
        let state = self.state.borrow();
        let function_name = state.function_name.as_deref().unwrap_or("function");
        
        // Check for unconsumed positional arguments
        if state.positional_index < self.positional.len() {
            let remaining = self.positional.len() - state.positional_index;
            return Err(anyhow::anyhow!(
                "{}() got {} unexpected positional argument{}",
                function_name,
                remaining,
                if remaining == 1 { "" } else { "s" }
            ));
        }

        // Check for unconsumed keyword arguments
        let unconsumed_keywords: Vec<&String> = self.keywords.keys()
            .filter(|k| !state.consumed_keywords.contains(k))
            .collect();
        
        if !unconsumed_keywords.is_empty() {
            return Err(anyhow::anyhow!(
                "{}() got unexpected keyword argument{}: {}",
                function_name,
                if unconsumed_keywords.len() == 1 { "" } else { "s" },
                unconsumed_keywords.into_iter().map(|s| format!("'{}'", s)).collect::<Vec<_>>().join(", ")
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fluent_api_basic() {
        let args = Args::positional(vec![Value::Int(42), Value::String("hello".to_string())]);
        args.set_function_name("test");
        
        let first = args.expect("number").unwrap();
        assert_eq!(first, Value::Int(42));
        
        let second = args.expect("text").unwrap();
        assert_eq!(second, Value::String("hello".to_string()));
        
        // Should complete successfully
        args.complete().unwrap();
    }

    #[test]
    fn test_fluent_api_missing_arg() {
        let args = Args::positional(vec![Value::Int(42)]);
        args.set_function_name("test");
        
        let first = args.expect("number").unwrap();
        assert_eq!(first, Value::Int(42));
        
        // Should fail when expecting a missing argument
        let result = args.expect("missing");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("test() missing required argument 'missing'"));
    }

    #[test]
    fn test_fluent_api_too_many_args() {
        let args = Args::positional(vec![Value::Int(42), Value::String("extra".to_string())]);
        args.set_function_name("test");
        
        let first = args.expect("number").unwrap();
        assert_eq!(first, Value::Int(42));
        
        // Should fail when there are leftover arguments
        let result = args.complete();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("test() got 1 unexpected positional argument"));
    }

    #[test]
    fn test_fluent_api_optional() {
        let args = Args::positional(vec![Value::Int(42)]);
        args.set_function_name("test");
        
        let required = args.expect("number").unwrap();
        assert_eq!(required, Value::Int(42));
        
        let optional = args.optional("default", Value::String("default".to_string()));
        assert_eq!(optional, Value::String("default".to_string()));
        
        args.complete().unwrap();
    }

    #[test]
    fn test_fluent_api_keywords() {
        let mut keywords = IndexMap::new();
        keywords.insert("name".to_string(), Value::String("Alice".to_string()));
        keywords.insert("age".to_string(), Value::Int(30));
        
        let args = Args::new(vec![Value::Bool(true)], keywords);
        args.set_function_name("test");
        
        let positional = args.expect("flag").unwrap();
        assert_eq!(positional, Value::Bool(true));
        
        let name = args.keyword("name").unwrap();
        assert_eq!(name, Value::String("Alice".to_string()));
        
        let age = args.keyword("age").unwrap();
        assert_eq!(age, Value::Int(30));
        
        args.complete().unwrap();
    }

    #[test]
    fn test_fluent_api_unused_keywords() {
        let mut keywords = IndexMap::new();
        keywords.insert("unused".to_string(), Value::String("value".to_string()));
        
        let args = Args::new(vec![], keywords);
        args.set_function_name("test");
        
        // Should fail when keyword arguments are not consumed
        let result = args.complete();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("test() got unexpected keyword argument: 'unused'"));
    }

    #[test]
    fn test_fluent_api_rest() {
        let args = Args::positional(vec![
            Value::Int(1), 
            Value::Int(2), 
            Value::Int(3), 
            Value::Int(4)
        ]);
        args.set_function_name("test");
        
        let first = args.expect("first").unwrap();
        assert_eq!(first, Value::Int(1));
        
        let rest = args.rest();
        assert_eq!(rest, vec![Value::Int(2), Value::Int(3), Value::Int(4)]);
        
        args.complete().unwrap();
    }
}