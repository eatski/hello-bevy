//! 型システムのエラー定義
//! 
//! 型チェックやコンパイル時のエラーを詳細に報告

use std::fmt;
use crate::structured_token::StructuredTokenInput;
use super::Type;

/// 型エラーの種類
#[derive(Debug, Clone)]
pub enum TypeError {
    /// 型の不一致
    TypeMismatch {
        expected: Type,
        actual: Type,
        context: String,
    },
    
    /// 未定義のトークン
    UndefinedToken {
        token_type: String,
    },
    
    /// 引数の数が合わない
    ArgumentCountMismatch {
        token_type: String,
        expected: usize,
        actual: usize,
    },
    
    /// 必須フィールドが不足
    MissingField {
        token_type: String,
        field_name: String,
    },
    
    /// 型が解決できない
    UnresolvedType {
        context: String,
    },
    
    /// 循環参照
    CyclicReference {
        token_type: String,
    },
    
    /// 型推論エラー (Phase 2)
    InferenceError {
        /// エラーの種類
        kind: InferenceErrorKind,
        /// 関連する型情報
        types: Vec<InferredType>,
        /// エラーが発生した場所
        location: Vec<String>,
    },
    
    /// Trait境界エラー (Phase 2)
    TraitBoundError {
        ty: Type,
        trait_name: String,
        available_traits: Vec<String>,
    },
}

/// 型推論エラーの種類
#[derive(Debug, Clone, PartialEq)]
pub enum InferenceErrorKind {
    /// 型変数の統一化失敗
    UnificationFailure,
    /// 無限型の検出
    InfiniteType,
    /// 多相型のインスタンス化失敗
    InstantiationFailure,
    /// 制約の解決失敗
    ConstraintViolation,
}

/// 推論された型情報
#[derive(Debug, Clone, PartialEq)]
pub struct InferredType {
    /// 型変数名（あれば）
    pub var_name: Option<String>,
    /// 推論された型
    pub ty: Type,
    /// 推論の起源
    pub origin: String,
}

/// コンパイルエラー
#[derive(Debug)]
pub struct CompileError {
    /// エラーの種類
    pub error: TypeError,
    /// エラーが発生したトークンのパス（ネストしたトークンの場合）
    pub path: Vec<String>,
    /// 元のトークン（デバッグ用）
    pub token: Option<StructuredTokenInput>,
}

impl CompileError {
    pub fn new(error: TypeError) -> Self {
        Self {
            error,
            path: vec![],
            token: None,
        }
    }
    
    pub fn with_path(mut self, path: Vec<String>) -> Self {
        self.path = path;
        self
    }
    
    pub fn with_token(mut self, token: StructuredTokenInput) -> Self {
        self.token = Some(token);
        self
    }
    
    /// コンテキストを追加（パスに要素を追加）
    pub fn add_context(mut self, context: String) -> Self {
        self.path.insert(0, context);
        self
    }
}

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeError::TypeMismatch { expected, actual, context } => {
                write!(f, "Type mismatch in {}: expected {}, but got {}", context, expected, actual)
            }
            TypeError::UndefinedToken { token_type } => {
                write!(f, "Undefined token type: {}", token_type)
            }
            TypeError::ArgumentCountMismatch { token_type, expected, actual } => {
                write!(f, "Token '{}' expects {} arguments, but got {}", token_type, expected, actual)
            }
            TypeError::MissingField { token_type, field_name } => {
                write!(f, "Token '{}' is missing required field '{}'", token_type, field_name)
            }
            TypeError::UnresolvedType { context } => {
                write!(f, "Cannot resolve type in context: {}", context)
            }
            TypeError::CyclicReference { token_type } => {
                write!(f, "Cyclic reference detected in token '{}'", token_type)
            }
            TypeError::InferenceError { kind, types, location } => {
                write!(f, "Type inference error ({:?}) at {:?} with types: {:?}", kind, location, types)
            }
            TypeError::TraitBoundError { ty, trait_name, available_traits: _ } => {
                write!(f, "Type {} does not implement trait {}", ty, trait_name)
            }
        }
    }
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.path.is_empty() {
            write!(f, "{}", self.error)
        } else {
            write!(f, "{} at {}", self.error, self.path.join(" -> "))
        }
    }
}

impl std::error::Error for CompileError {}

pub type CompileResult<T> = Result<T, CompileError>;