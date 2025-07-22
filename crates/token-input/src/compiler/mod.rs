//! コンパイラモジュール
//! 
//! StructuredTokenInput → 型検査 → 中間表現 → Node への変換パイプライン

pub mod pipeline;
pub mod code_generator;
pub mod error_reporter;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod type_checker_tests;
#[cfg(test)]
mod advanced_type_tests;

pub use pipeline::{Compiler, CompilerOptions};
pub use error_reporter::ErrorReporter;