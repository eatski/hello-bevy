// 型情報を伝播させる値コンバーター

use crate::{StructuredTokenInput, typed_node_converter::{TypedNodeConverter, TypedConverterRegistry}};
use crate::type_system::{TypedAst, Type};
use action_system::*;

// Type alias for Node trait with action-system's EvaluationContext
type ActionSystemNode<T> = dyn for<'a> Node<T, EvaluationContext<'a>> + Send + Sync;

/// 型情報を活用する数値コンバーター
pub struct TypedNumberConverter;

impl TypedNodeConverter<i32> for TypedNumberConverter {
    fn can_convert(&self, token: &StructuredTokenInput, expected_type: &Type) -> bool {
        matches!(token, StructuredTokenInput::Number { .. }) && 
        matches!(expected_type, Type::I32)
    }
    
    fn convert(&self, 
               typed_ast: &TypedAst, 
               _registry: &dyn TypedConverterRegistry) -> Result<Box<ActionSystemNode<i32>>, String> {
        if let StructuredTokenInput::Number { value } = &typed_ast.token {
            Ok(Box::new(ConstantValueNode::new(*value)))
        } else {
            Err("Expected Number token".to_string())
        }
    }
}