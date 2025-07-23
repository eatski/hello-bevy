use crate::{StructuredTokenInput, typed_node_converter::{TypedNodeConverter, TypedConverterRegistry}};
use crate::type_system::{TypedAst, Type};
use action_system::*;

// Type alias for Node trait with action-system's EvaluationContext
type ActionSystemNode<T> = dyn for<'a> Node<T, EvaluationContext<'a>> + Send + Sync;

/// 型情報を活用するAllTeamSidesコンバーター
pub struct TypedAllTeamSidesConverter;

impl TypedNodeConverter<Vec<TeamSide>> for TypedAllTeamSidesConverter {
    fn can_convert(&self, token: &StructuredTokenInput, expected_type: &Type) -> bool {
        matches!(token, StructuredTokenInput::AllTeamSides) && 
        if let Type::Vec(elem_type) = expected_type {
            matches!(**elem_type, Type::TeamSide)
        } else {
            false
        }
    }
    
    fn convert(&self, 
               typed_ast: &TypedAst, 
               _registry: &dyn TypedConverterRegistry) -> Result<Box<ActionSystemNode<Vec<TeamSide>>>, String> {
        if let StructuredTokenInput::AllTeamSides = &typed_ast.token {
            Ok(Box::new(AllTeamSidesNode))
        } else {
            Err("Expected AllTeamSides token".to_string())
        }
    }
}