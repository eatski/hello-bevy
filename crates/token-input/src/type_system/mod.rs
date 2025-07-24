//! 型システムモジュール
//! 
//! ADRに基づいた3層アーキテクチャのコア層実装
//! - 型定義と型解決のルール管理
//! - 静的型検査
//! - 型推論（基本的な部分のみ）

pub mod types;
pub mod advanced_type_checker;
pub mod errors;
pub mod type_inference;
pub mod hindley_milner;
pub mod traits;
pub mod generics;

pub use types::*;
pub use errors::*;
pub use type_inference::*;
// TypeCheckerとしてAdvancedTypeCheckerをエクスポート
pub use advanced_type_checker::AdvancedTypeChecker;
pub use advanced_type_checker::AdvancedTypeChecker as TypeChecker;