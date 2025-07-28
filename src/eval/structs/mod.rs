// Re-export all evaluation structs from their respective modules

pub mod attributes;
pub mod blocks;
pub mod collections;
pub mod control_flow;
pub mod functions;
pub mod literals;
pub mod misc;
pub mod operators;
pub mod patterns;

// Re-export all structs at the module level for convenience
pub use attributes::{EvalGetAttr, EvalSetAttr};
pub use blocks::{EvalBlock, EvalProgram, EvalWith};
pub use collections::{EvalDict, EvalGetItem, EvalList, EvalSetItem};
pub use control_flow::{EvalBreak, EvalContinue, EvalFor, EvalIf, EvalLoop, EvalReturn, EvalWhile};
pub use functions::{EvalCall, EvalClassDecl, EvalFunction, EvalLambda};
pub use literals::{EvalLiteral, EvalRef, EvalVariable};
pub use misc::{EvalAssign, EvalDeclare, EvalImport, EvalMacro};
pub use operators::{EvalIs, EvalLogicalAnd, EvalLogicalNot, EvalLogicalOr};
pub use patterns::{EvalDeclarePattern, EvalMatch, EvalTry};
