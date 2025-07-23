use crate::{StructuredTokenInput, typed_node_converter::{TypedNodeConverter, TypedConverterRegistry}};
use crate::type_system::{TypedAst, Type};
use action_system::*;

// Type alias for Node trait with action-system's EvaluationContext
type ActionSystemNode<T> = dyn for<'a> Node<T, EvaluationContext<'a>> + Send + Sync;

/// 型情報を活用するActingCharacterコンバーター
pub struct TypedActingCharacterConverter;

impl TypedNodeConverter<Character> for TypedActingCharacterConverter {
    fn can_convert(&self, token: &StructuredTokenInput, expected_type: &Type) -> bool {
        matches!(token, StructuredTokenInput::ActingCharacter) && 
        matches!(expected_type, Type::Character)
    }
    
    fn convert(&self, 
               typed_ast: &TypedAst, 
               _registry: &dyn TypedConverterRegistry) -> Result<Box<ActionSystemNode<Character>>, String> {
        if let StructuredTokenInput::ActingCharacter = &typed_ast.token {
            Ok(Box::new(ActingCharacterNode))
        } else {
            Err("Expected ActingCharacter token".to_string())
        }
    }
}