use crate::{StructuredTokenInput, typed_node_converter::{TypedNodeConverter, TypedConverterRegistry}};
use crate::type_system::{TypedAst, Type};
use action_system::*;

// Type alias for Node trait with action-system's EvaluationContext
type ActionSystemNode<T> = dyn for<'a> Node<T, EvaluationContext<'a>> + Send + Sync;

/// 型情報を活用するAllCharactersコンバーター
pub struct TypedAllCharactersConverter;

impl TypedNodeConverter<Vec<Character>> for TypedAllCharactersConverter {
    fn can_convert(&self, token: &StructuredTokenInput, expected_type: &Type) -> bool {
        matches!(token, StructuredTokenInput::AllCharacters) && 
        if let Type::Vec(elem_type) = expected_type {
            matches!(**elem_type, Type::Character)
        } else {
            false
        }
    }
    
    fn convert(&self, 
               typed_ast: &TypedAst, 
               _registry: &dyn TypedConverterRegistry) -> Result<Box<ActionSystemNode<Vec<Character>>>, String> {
        if let StructuredTokenInput::AllCharacters = &typed_ast.token {
            Ok(Box::new(AllCharactersNode))
        } else {
            Err("Expected AllCharacters token".to_string())
        }
    }
}