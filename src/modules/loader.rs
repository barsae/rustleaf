use std::path::{Path, PathBuf};
use std::fs;
use crate::parser::ast::{ModulePath, ModulePathRoot};

/// Handles module file resolution and loading
#[derive(Debug, Clone)]
pub struct ModuleLoader {
    /// Base directory for module resolution (current file's directory)
    base_dir: PathBuf,
}

impl ModuleLoader {
    /// Create a new module loader with a base directory
    pub fn new(base_dir: impl AsRef<Path>) -> Self {
        Self {
            base_dir: base_dir.as_ref().to_path_buf(),
        }
    }

    /// Resolve a module path to a filesystem path
    pub fn resolve_path(&self, module_path: &ModulePath) -> Result<PathBuf, ModuleResolutionError> {
        let mut resolved_path = match module_path.root_type {
            ModulePathRoot::Relative => self.base_dir.clone(),
            ModulePathRoot::Super => {
                self.base_dir.parent()
                    .ok_or_else(|| ModuleResolutionError::InvalidSuperPath {
                        base_dir: self.base_dir.clone(),
                    })?
                    .to_path_buf()
            }
        };

        // Add each segment as a directory component
        for segment in &module_path.segments {
            resolved_path.push(segment);
        }

        // Add .rustleaf extension
        resolved_path.set_extension("rustleaf");

        Ok(resolved_path)
    }

    /// Load module source code from filesystem
    pub fn load_module(&self, module_path: &ModulePath) -> Result<String, ModuleLoadError> {
        let file_path = self.resolve_path(module_path)
            .map_err(ModuleLoadError::Resolution)?;

        if !file_path.exists() {
            return Err(ModuleLoadError::FileNotFound {
                path: file_path,
                original_module_path: format!("{}", ModulePathDisplay(module_path)),
            });
        }

        fs::read_to_string(&file_path)
            .map_err(|io_error| ModuleLoadError::IoError {
                path: file_path,
                error: io_error.to_string(),
            })
    }

    /// Get the base directory
    pub fn base_dir(&self) -> &Path {
        &self.base_dir
    }

    /// Create a new loader for a subdirectory (for nested imports)
    pub fn for_subdirectory(&self, subdir: impl AsRef<Path>) -> Self {
        let mut new_base = self.base_dir.clone();
        new_base.push(subdir);
        Self::new(new_base)
    }
}

#[derive(Debug, Clone)]
pub enum ModuleResolutionError {
    InvalidSuperPath {
        base_dir: PathBuf,
    },
}

impl std::fmt::Display for ModuleResolutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModuleResolutionError::InvalidSuperPath { base_dir } => {
                write!(f, "Cannot resolve super path from base directory: {}", base_dir.display())
            }
        }
    }
}

impl std::error::Error for ModuleResolutionError {}

#[derive(Debug)]
pub enum ModuleLoadError {
    Resolution(ModuleResolutionError),
    FileNotFound {
        path: PathBuf,
        original_module_path: String,
    },
    IoError {
        path: PathBuf,
        error: String, // Store error message instead of std::io::Error
    },
}

impl std::fmt::Display for ModuleLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModuleLoadError::Resolution(err) => write!(f, "Module resolution error: {}", err),
            ModuleLoadError::FileNotFound { path, original_module_path } => {
                write!(f, "Module not found: '{}' (resolved to {})", original_module_path, path.display())
            }
            ModuleLoadError::IoError { path, error } => {
                write!(f, "IO error loading module from {}: {}", path.display(), error)
            }
        }
    }
}

impl std::error::Error for ModuleLoadError {}

/// Helper struct for displaying module paths
struct ModulePathDisplay<'a>(&'a ModulePath);

impl<'a> std::fmt::Display for ModulePathDisplay<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0.root_type {
            ModulePathRoot::Super => write!(f, "super::")?,
            ModulePathRoot::Relative => {}
        }
        write!(f, "{}", self.0.segments.join("::"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_resolve_relative_path() {
        let temp_dir = TempDir::new().unwrap();
        let loader = ModuleLoader::new(temp_dir.path());

        let module_path = ModulePath {
            root_type: ModulePathRoot::Relative,
            segments: vec!["math".to_string(), "geometry".to_string()],
        };

        let resolved = loader.resolve_path(&module_path).unwrap();
        let expected = temp_dir.path().join("math").join("geometry.rustleaf");
        assert_eq!(resolved, expected);
    }

    #[test]
    fn test_resolve_super_path() {
        let temp_dir = TempDir::new().unwrap();
        let subdir = temp_dir.path().join("subdir");
        fs::create_dir(&subdir).unwrap();
        
        let loader = ModuleLoader::new(&subdir);

        let module_path = ModulePath {
            root_type: ModulePathRoot::Super,
            segments: vec!["sibling".to_string()],
        };

        let resolved = loader.resolve_path(&module_path).unwrap();
        let expected = temp_dir.path().join("sibling.rustleaf");
        assert_eq!(resolved, expected);
    }

    #[test]
    fn test_load_existing_module() {
        let temp_dir = TempDir::new().unwrap();
        let module_file = temp_dir.path().join("test_module.rustleaf");
        let content = "pub fn hello() { \"Hello from module\" }";
        fs::write(&module_file, content).unwrap();

        let loader = ModuleLoader::new(temp_dir.path());
        let module_path = ModulePath {
            root_type: ModulePathRoot::Relative,
            segments: vec!["test_module".to_string()],
        };

        let loaded_content = loader.load_module(&module_path).unwrap();
        assert_eq!(loaded_content, content);
    }

    #[test]
    fn test_load_nonexistent_module() {
        let temp_dir = TempDir::new().unwrap();
        let loader = ModuleLoader::new(temp_dir.path());

        let module_path = ModulePath {
            root_type: ModulePathRoot::Relative,
            segments: vec!["nonexistent".to_string()],
        };

        let result = loader.load_module(&module_path);
        assert!(matches!(result, Err(ModuleLoadError::FileNotFound { .. })));
    }

    #[test]
    fn test_super_path_resolution_at_root() {
        let temp_dir = TempDir::new().unwrap();
        let loader = ModuleLoader::new(temp_dir.path());

        let module_path = ModulePath {
            root_type: ModulePathRoot::Super,
            segments: vec!["parent".to_string()],
        };

        // This should work if temp_dir has a parent
        let result = loader.resolve_path(&module_path);
        // Result depends on filesystem structure, but shouldn't panic
        match result {
            Ok(path) => assert!(path.ends_with("parent.rustleaf")),
            Err(ModuleResolutionError::InvalidSuperPath { .. }) => {
                // This is acceptable if we're at filesystem root
            }
        }
    }
}