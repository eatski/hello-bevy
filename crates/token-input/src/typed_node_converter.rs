// 型情報を伝播させる新しいコンバーターシステム

use crate::StructuredTokenInput;
use crate::type_system::{TypedAst, Type};
use action_system::*;
use std::any::{Any, TypeId};

// Type alias for Node trait with action-system's EvaluationContext
type ActionSystemNode<T> = dyn for<'a> Node<T, EvaluationContext<'a>> + Send + Sync;

/// 型情報を伝播させるコンバータートレイト
pub trait TypedNodeConverter<T>: Send + Sync {
    /// このコンバーターが指定されたトークンと型を変換できるかチェック
    fn can_convert(&self, token: &StructuredTokenInput, expected_type: &Type) -> bool;
    
    /// 型情報を使ってトークンをノードに変換
    fn convert(&self, 
               typed_ast: &TypedAst, 
               registry: &dyn TypedConverterRegistry) -> Result<Box<ActionSystemNode<T>>, String>;
}

/// 型消去されたノード
pub type ErasedNode = Box<dyn Any + Send + Sync>;

/// 型情報を伝播させるコンバーターレジストリのトレイト
pub trait TypedConverterRegistry: Send + Sync {
    /// 型情報を使って変換（型消去版）
    fn convert_typed_erased(&self, typed_ast: &TypedAst, target_type_id: TypeId) -> Result<ErasedNode, String>;
    
    /// 子要素の型情報を取得して変換（型消去版）
    fn convert_child_erased(&self, 
                           typed_ast: &TypedAst, 
                           child_name: &str,
                           target_type_id: TypeId) -> Result<ErasedNode, String>;
}

/// 型安全なヘルパー関数
pub fn convert_typed<T: Any + 'static>(
    registry: &dyn TypedConverterRegistry,
    typed_ast: &TypedAst
) -> Result<Box<ActionSystemNode<T>>, String> {
    let erased = registry.convert_typed_erased(typed_ast, TypeId::of::<T>())?;
    erased.downcast::<Box<ActionSystemNode<T>>>()
        .map(|boxed| *boxed)
        .map_err(|_| format!("Type mismatch: expected {:?}", std::any::type_name::<T>()))
}

/// 型安全なヘルパー関数（子要素用）
pub fn convert_child<T: Any + 'static>(
    registry: &dyn TypedConverterRegistry,
    typed_ast: &TypedAst,
    child_name: &str
) -> Result<Box<ActionSystemNode<T>>, String> {
    let erased = registry.convert_child_erased(typed_ast, child_name, TypeId::of::<T>())?;
    erased.downcast::<Box<ActionSystemNode<T>>>()
        .map(|boxed| *boxed)
        .map_err(|_| format!("Type mismatch: expected {:?} for child {}", std::any::type_name::<T>(), child_name))
}