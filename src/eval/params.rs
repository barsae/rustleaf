use crate::core::{ParameterKind, Value};

#[derive(Debug, Clone)]
pub struct Params {
    inner: Vec<(String, Option<Value>, ParameterKind)>,
}

impl Params {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn from_vec(params: Vec<(String, Option<Value>, ParameterKind)>) -> Self {
        Self { inner: params }
    }

    pub fn add_regular(mut self, name: impl Into<String>) -> Self {
        self.inner.push((name.into(), None, ParameterKind::Regular));
        self
    }

    pub fn add_with_default(mut self, name: impl Into<String>, default: Value) -> Self {
        self.inner
            .push((name.into(), Some(default), ParameterKind::Regular));
        self
    }

    pub fn add_rest(mut self, name: impl Into<String>) -> Self {
        self.inner.push((name.into(), None, ParameterKind::Rest));
        self
    }

    pub fn add_keyword(mut self, name: impl Into<String>) -> Self {
        self.inner.push((name.into(), None, ParameterKind::Keyword));
        self
    }

    pub fn from_names(names: &[String]) -> Self {
        let inner = names
            .iter()
            .map(|name| (name.clone(), None, ParameterKind::Regular))
            .collect();
        Self { inner }
    }

    pub fn iter(&self) -> impl Iterator<Item = &(String, Option<Value>, ParameterKind)> {
        self.inner.iter()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn names(&self) -> Vec<&str> {
        self.inner
            .iter()
            .map(|(name, _, _)| name.as_str())
            .collect()
    }

    pub fn get(&self, index: usize) -> Option<&(String, Option<Value>, ParameterKind)> {
        self.inner.get(index)
    }

    pub fn into_inner(self) -> Vec<(String, Option<Value>, ParameterKind)> {
        self.inner
    }

    pub fn as_slice(&self) -> &[(String, Option<Value>, ParameterKind)] {
        &self.inner
    }
}

impl Default for Params {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Vec<(String, Option<Value>, ParameterKind)>> for Params {
    fn from(params: Vec<(String, Option<Value>, ParameterKind)>) -> Self {
        Self::from_vec(params)
    }
}

impl From<Params> for Vec<(String, Option<Value>, ParameterKind)> {
    fn from(params: Params) -> Self {
        params.into_inner()
    }
}
