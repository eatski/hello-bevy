use crate::{StructuredTokenInput, typed_node_converter::{TypedNodeConverter, TypedConverterRegistry, convert_child}};
use crate::type_system::{TypedAst, Type};
use action_system::*;
use action_system::nodes::array::FilterListNode;

// Type alias for Node trait with action-system's EvaluationContext
type ActionSystemNode<T> = dyn for<'a> Node<T, EvaluationContext<'a>> + Send + Sync;

/// Character専用のFilterListコンバーター
pub struct TypedFilterListCharacterConverter;

impl TypedNodeConverter<Vec<Character>> for TypedFilterListCharacterConverter {
    fn can_convert(&self, token: &StructuredTokenInput, expected_type: &Type) -> bool {
        matches!(token, StructuredTokenInput::FilterList { .. }) && 
        if let Type::Vec(elem_type) = expected_type {
            matches!(**elem_type, Type::Character)
        } else {
            false
        }
    }
    
    fn convert(&self, 
               typed_ast: &TypedAst, 
               registry: &dyn TypedConverterRegistry) -> Result<Box<ActionSystemNode<Vec<Character>>>, String> {
        if let StructuredTokenInput::FilterList { .. } = &typed_ast.token {
            let array_node = convert_child::<Vec<Character>>(registry, typed_ast, "array")?;
            let condition_node = convert_child::<bool>(registry, typed_ast, "condition")?;
            
            Ok(Box::new(FilterListNode::new(array_node, condition_node)))
        } else {
            Err("Expected FilterList token".to_string())
        }
    }
}