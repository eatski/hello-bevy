use crate::{StructuredTokenInput, typed_node_converter::{TypedNodeConverter, TypedConverterRegistry}};
use crate::type_system::{TypedAst, Type};
use action_system::*;

// Type alias for Node trait with action-system's EvaluationContext
type ActionSystemNode<T> = dyn for<'a> Node<T, EvaluationContext<'a>> + Send + Sync;

/// 型情報を活用するElementコンバーター（Character型）
pub struct TypedElementCharacterConverter;

impl TypedNodeConverter<Character> for TypedElementCharacterConverter {
    fn can_convert(&self, token: &StructuredTokenInput, expected_type: &Type) -> bool {
        matches!(token, StructuredTokenInput::Element) && 
        matches!(expected_type, Type::Character)
    }
    
    fn convert(&self, 
               typed_ast: &TypedAst, 
               _registry: &dyn TypedConverterRegistry) -> Result<Box<ActionSystemNode<Character>>, String> {
        if let StructuredTokenInput::Element = &typed_ast.token {
            Ok(Box::new(ElementNode::new()))
        } else {
            Err("Expected Element token".to_string())
        }
    }
}

/// 型情報を活用するElementコンバーター（i32型）
pub struct TypedElementI32Converter;

impl TypedNodeConverter<i32> for TypedElementI32Converter {
    fn can_convert(&self, token: &StructuredTokenInput, expected_type: &Type) -> bool {
        matches!(token, StructuredTokenInput::Element) && 
        matches!(expected_type, Type::I32)
    }
    
    fn convert(&self, 
               typed_ast: &TypedAst, 
               _registry: &dyn TypedConverterRegistry) -> Result<Box<ActionSystemNode<i32>>, String> {
        if let StructuredTokenInput::Element = &typed_ast.token {
            Ok(Box::new(ElementNode::new()))
        } else {
            Err("Expected Element token".to_string())
        }
    }
}

/// 型情報を活用するElementコンバーター（TeamSide型）
pub struct TypedElementTeamSideConverter;

impl TypedNodeConverter<TeamSide> for TypedElementTeamSideConverter {
    fn can_convert(&self, token: &StructuredTokenInput, expected_type: &Type) -> bool {
        matches!(token, StructuredTokenInput::Element) && 
        matches!(expected_type, Type::TeamSide)
    }
    
    fn convert(&self, 
               typed_ast: &TypedAst, 
               _registry: &dyn TypedConverterRegistry) -> Result<Box<ActionSystemNode<TeamSide>>, String> {
        if let StructuredTokenInput::Element = &typed_ast.token {
            Ok(Box::new(ElementNode::new()))
        } else {
            Err("Expected Element token".to_string())
        }
    }
}