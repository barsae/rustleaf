use crate::parser::ast::{ImportClause, ImportItem, Visibility};
use crate::value::Value;
use std::collections::HashMap;

/// Represents an exported item from a module
#[derive(Debug, Clone)]
pub struct ExportedItem {
    pub name: String,
    pub value: Value,
    pub visibility: Visibility,
}

/// Module environment that tracks module-level variables and exports
#[derive(Debug, Clone)]
pub struct ModuleEnvironment {
    /// All module-level bindings (both public and private)
    bindings: HashMap<String, Value>,
    /// Visibility of each binding
    visibility: HashMap<String, Visibility>,
    /// Imported items from other modules
    imports: HashMap<String, Value>,
}

impl Default for ModuleEnvironment {
    fn default() -> Self {
        Self::new()
    }
}

impl ModuleEnvironment {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
            visibility: HashMap::new(),
            imports: HashMap::new(),
        }
    }

    /// Define a binding in this module
    pub fn define(&mut self, name: String, value: Value, visibility: Visibility) {
        self.bindings.insert(name.clone(), value);
        self.visibility.insert(name, visibility);
    }

    /// Get a binding from this module (respects visibility for external access)
    pub fn get(&self, name: &str, external_access: bool) -> Option<&Value> {
        if let Some(value) = self.bindings.get(name) {
            if external_access {
                // External access - check visibility
                match self.visibility.get(name) {
                    Some(Visibility::Public) => Some(value),
                    _ => None, // Private or not found
                }
            } else {
                // Internal access - always allowed
                Some(value)
            }
        } else {
            // Check imports
            self.imports.get(name)
        }
    }

    /// Get all public bindings for export
    pub fn get_public_bindings(&self) -> HashMap<String, Value> {
        self.bindings
            .iter()
            .filter_map(|(name, value)| {
                if matches!(self.visibility.get(name), Some(Visibility::Public)) {
                    Some((name.clone(), value.clone()))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Import items from another module based on import clause
    pub fn import_from(
        &mut self,
        source_env: &ModuleEnvironment,
        clause: &Option<ImportClause>,
        module_name: &str,
    ) -> Result<(), ImportError> {
        match clause {
            None => {
                // Import the module itself as a namespace
                let module_obj = self.create_module_object(source_env);
                self.imports.insert(module_name.to_string(), module_obj);
            }
            Some(ImportClause::All) => {
                // Import all public items
                for (name, value) in source_env.get_public_bindings() {
                    if self.imports.contains_key(&name) || self.bindings.contains_key(&name) {
                        return Err(ImportError::NameConflict {
                            name,
                            reason: "Item already exists in current scope".to_string(),
                        });
                    }
                    self.imports.insert(name, value);
                }
            }
            Some(ImportClause::Single(item_name)) => {
                // Import a single item
                if let Some(value) = source_env.get(item_name, true) {
                    if self.imports.contains_key(item_name) || self.bindings.contains_key(item_name)
                    {
                        return Err(ImportError::NameConflict {
                            name: item_name.clone(),
                            reason: "Item already exists in current scope".to_string(),
                        });
                    }
                    self.imports.insert(item_name.clone(), value.clone());
                } else {
                    return Err(ImportError::ItemNotFound {
                        item: item_name.clone(),
                        module: module_name.to_string(),
                    });
                }
            }
            Some(ImportClause::Named(items)) => {
                // Import specific named items
                for ImportItem { name, alias } in items {
                    if let Some(value) = source_env.get(name, true) {
                        let import_name = alias.as_ref().unwrap_or(name);
                        if self.imports.contains_key(import_name)
                            || self.bindings.contains_key(import_name)
                        {
                            return Err(ImportError::NameConflict {
                                name: import_name.clone(),
                                reason: "Item already exists in current scope".to_string(),
                            });
                        }
                        self.imports.insert(import_name.clone(), value.clone());
                    } else {
                        return Err(ImportError::ItemNotFound {
                            item: name.clone(),
                            module: module_name.to_string(),
                        });
                    }
                }
            }
        }
        Ok(())
    }

    /// Create a module object that provides access to all public items
    fn create_module_object(&self, source_env: &ModuleEnvironment) -> Value {
        let public_bindings = source_env.get_public_bindings();
        Value::new_dict(public_bindings)
    }

    /// Check if a name exists in this module (including imports)
    pub fn contains(&self, name: &str) -> bool {
        self.bindings.contains_key(name) || self.imports.contains_key(name)
    }

    /// Get all available names (bindings + imports)
    pub fn available_names(&self) -> impl Iterator<Item = &String> {
        self.bindings.keys().chain(self.imports.keys())
    }

    /// Update an existing binding
    pub fn update(&mut self, name: &str, value: Value) -> Result<(), UpdateError> {
        if self.bindings.contains_key(name) {
            self.bindings.insert(name.to_string(), value);
            Ok(())
        } else if self.imports.contains_key(name) {
            Err(UpdateError::ImportedVariable {
                name: name.to_string(),
            })
        } else {
            Err(UpdateError::UndefinedVariable {
                name: name.to_string(),
            })
        }
    }

    /// Get the visibility of a binding
    pub fn get_visibility(&self, name: &str) -> Option<&Visibility> {
        self.visibility.get(name)
    }
}

#[derive(Debug, Clone)]
pub enum ImportError {
    ItemNotFound { item: String, module: String },
    NameConflict { name: String, reason: String },
}

impl std::fmt::Display for ImportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImportError::ItemNotFound { item, module } => {
                write!(f, "Item '{}' not found in module '{}'", item, module)
            }
            ImportError::NameConflict { name, reason } => {
                write!(f, "Name conflict for '{}': {}", name, reason)
            }
        }
    }
}

impl std::error::Error for ImportError {}

#[derive(Debug, Clone)]
pub enum UpdateError {
    UndefinedVariable { name: String },
    ImportedVariable { name: String },
}

impl std::fmt::Display for UpdateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UpdateError::UndefinedVariable { name } => {
                write!(f, "Undefined variable: '{}'", name)
            }
            UpdateError::ImportedVariable { name } => {
                write!(f, "Cannot modify imported variable: '{}'", name)
            }
        }
    }
}

impl std::error::Error for UpdateError {}
