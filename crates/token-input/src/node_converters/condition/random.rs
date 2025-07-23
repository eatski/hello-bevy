use crate::{StructuredTokenInput, typed_node_converter::{TypedNodeConverter, TypedConverterRegistry}};
use crate::type_system::{TypedAst, Type};
use action_system::*;
use action_system::nodes::condition::RandomConditionNode;

// Type alias for Node trait with action-system's EvaluationContext
type ActionSystemNode<T> = dyn for<'a> Node<T, EvaluationContext<'a>> + Send + Sync;

/// 型情報を活用するTrueOrFalseRandomコンバーター
pub struct TypedTrueOrFalseRandomConverter;

impl TypedNodeConverter<bool> for TypedTrueOrFalseRandomConverter {
    fn can_convert(&self, token: &StructuredTokenInput, expected_type: &Type) -> bool {
        matches!(token, StructuredTokenInput::TrueOrFalseRandom) && 
        matches!(expected_type, Type::Bool)
    }
    
    fn convert(&self, 
               typed_ast: &TypedAst, 
               _registry: &dyn TypedConverterRegistry) -> Result<Box<ActionSystemNode<bool>>, String> {
        if let StructuredTokenInput::TrueOrFalseRandom = &typed_ast.token {
            Ok(Box::new(RandomConditionNode))
        } else {
            Err("Expected TrueOrFalseRandom token".to_string())
        }
    }
}