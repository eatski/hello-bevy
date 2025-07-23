use crate::{StructuredTokenInput, typed_node_converter::{TypedNodeConverter, TypedConverterRegistry, convert_child}};
use crate::type_system::{TypedAst, Type};
use action_system::*;

// Type alias for Node trait with action-system's EvaluationContext
type ActionSystemNode<T> = dyn for<'a> Node<T, EvaluationContext<'a>> + Send + Sync;

/// Character専用のMinコンバーター（Min -> CharacterのマッピングがAPIで存在するため）
pub struct TypedMinCharacterConverter;

impl TypedNodeConverter<Character> for TypedMinCharacterConverter {
    fn can_convert(&self, token: &StructuredTokenInput, expected_type: &Type) -> bool {
        matches!(token, StructuredTokenInput::Min { .. }) && 
        matches!(expected_type, Type::Character)
    }
    
    fn convert(&self, 
               typed_ast: &TypedAst, 
               registry: &dyn TypedConverterRegistry) -> Result<Box<ActionSystemNode<Character>>, String> {
        if let StructuredTokenInput::Min { .. } = &typed_ast.token {
            let array_node = convert_child::<Vec<Character>>(registry, typed_ast, "array")?;
            Ok(Box::new(MinNodeCharacter::new(array_node)))
        } else {
            Err("Expected Min token".to_string())
        }
    }
}