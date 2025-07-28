// Module declarations for individual struct files
pub mod eval_assign;
pub mod eval_block;
pub mod eval_break;
pub mod eval_call;
pub mod eval_class_decl;
pub mod eval_continue;
pub mod eval_declare;
pub mod eval_declare_pattern;
pub mod eval_dict;
pub mod eval_for;
pub mod eval_function;
pub mod eval_get_attr;
pub mod eval_get_item;
pub mod eval_if;
pub mod eval_import;
pub mod eval_is;
pub mod eval_lambda;
pub mod eval_list;
pub mod eval_literal;
pub mod eval_logical_and;
pub mod eval_logical_not;
pub mod eval_logical_or;
pub mod eval_loop;
pub mod eval_macro;
pub mod eval_match;
pub mod eval_program;
pub mod eval_ref;
pub mod eval_return;
pub mod eval_set_attr;
pub mod eval_set_item;
pub mod eval_try;
pub mod eval_variable;
pub mod eval_while;
pub mod eval_with;

// Re-export all structs at the module level for convenience
pub use eval_assign::EvalAssign;
pub use eval_block::EvalBlock;
pub use eval_break::EvalBreak;
pub use eval_call::EvalCall;
pub use eval_class_decl::EvalClassDecl;
pub use eval_continue::EvalContinue;
pub use eval_declare::EvalDeclare;
pub use eval_declare_pattern::EvalDeclarePattern;
pub use eval_dict::EvalDict;
pub use eval_for::EvalFor;
pub use eval_function::EvalFunction;
pub use eval_get_attr::EvalGetAttr;
pub use eval_get_item::EvalGetItem;
pub use eval_if::EvalIf;
pub use eval_import::EvalImport;
pub use eval_is::EvalIs;
pub use eval_lambda::EvalLambda;
pub use eval_list::EvalList;
pub use eval_literal::EvalLiteral;
pub use eval_logical_and::EvalLogicalAnd;
pub use eval_logical_not::EvalLogicalNot;
pub use eval_logical_or::EvalLogicalOr;
pub use eval_loop::EvalLoop;
pub use eval_macro::EvalMacro;
pub use eval_match::EvalMatch;
pub use eval_program::EvalProgram;
pub use eval_return::EvalReturn;
pub use eval_set_attr::EvalSetAttr;
pub use eval_set_item::EvalSetItem;
pub use eval_try::EvalTry;
pub use eval_variable::EvalVariable;
pub use eval_while::EvalWhile;
pub use eval_with::EvalWith;

// Re-export from eval_ref (which contains multiple related structs)
pub use eval_ref::{
    ClassDeclData, ClassMethod, EvalDictPattern, EvalMatchCase, EvalMatchPattern, EvalPattern,
    FunctionData, ImportData, LambdaData, MacroData, MatchData, WithData,
};
