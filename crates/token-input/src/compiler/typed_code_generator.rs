//! 型情報を伝播させるコードジェネレータ
//! 
//! TypedASTから実行可能なNodeへの変換（型情報を保持したまま）

use std::sync::Arc;
use action_system::{RuleNode, Action};
use crate::typed_converter_registry::TypedConverterRegistryImpl;
use crate::typed_node_converter::convert_typed;
use crate::type_system::{TypedAst, Type, CompileError, CompileResult, TypeError};


/// 型情報を伝播させるコードジェネレータ
pub struct TypedCodeGenerator {
    registry: Arc<TypedConverterRegistryImpl>,
}

impl TypedCodeGenerator {
    pub fn new() -> Self {
        Self {
            registry: Arc::new(TypedConverterRegistryImpl::new()),
        }
    }
    
    /// TypedASTをNodeに変換（型情報を完全に活用）
    pub fn generate(&self, ast: &TypedAst) -> CompileResult<RuleNode> {
        // TypedAstから直接Action型のノードを生成
        match &ast.ty {
            Type::Action => {
                let action_node = convert_typed::<Box<dyn Action>>(self.registry.as_ref(), ast)
                    .map_err(|e| CompileError::new(TypeError::UnresolvedType {
                        context: format!("Failed to generate node with typed converter: {}", e),
                    }))?;
                Ok(action_node)
            }
            _ => {
                Err(CompileError::new(TypeError::UnresolvedType {
                    context: format!("Cannot convert {} to RuleNode. RuleNode requires Action type.", ast.ty),
                }))
            }
        }
    }
}

impl Default for TypedCodeGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// コンパイラのコードジェネレータを型情報伝播版に切り替えるための設定
pub struct TypedCompilerConfig {
    pub use_typed_code_generator: bool,
}

impl Default for TypedCompilerConfig {
    fn default() -> Self {
        Self {
            use_typed_code_generator: true, // デフォルトで新しいシステムを使用
        }
    }
}