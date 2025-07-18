#![allow(clippy::result_large_err)]
#![allow(clippy::uninlined_format_args)]

pub mod core;
pub mod environment;
pub mod expressions;
pub mod operators;
pub mod statements;

pub use core::Evaluator;
pub use environment::Environment;
