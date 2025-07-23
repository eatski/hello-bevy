use crate::{StructuredTokenInput, typed_node_converter::{TypedNodeConverter, TypedConverterRegistry, convert_child}};
use crate::type_system::{TypedAst, Type};
use action_system::*;
use action_system::nodes::array::{GenericFilterListNode, AsCurrentElement};
use std::marker::PhantomData;

// Type alias for Node trait with action-system's EvaluationContext
type ActionSystemNode<T> = dyn for<'a> Node<T, EvaluationContext<'a>> + Send + Sync;

/// ジェネリック型のFilterListコンバーター（Character以外）
pub struct TypedGenericFilterListConverter<T> 
where
    T: Clone + Send + Sync + 'static,
{
    _phantom: PhantomData<T>,
}

impl<T> TypedGenericFilterListConverter<T>
where
    T: Clone + Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<T> TypedNodeConverter<Vec<T>> for TypedGenericFilterListConverter<T>
where
    T: Clone + Send + Sync + AsCurrentElement + 'static,
{
    fn can_convert(&self, token: &StructuredTokenInput, expected_type: &Type) -> bool {
        if let StructuredTokenInput::FilterList { .. } = token {
            if let Type::Vec(elem_type) = expected_type {
                // Character以外の型に対応
                !matches!(**elem_type, Type::Character)
            } else {
                false
            }
        } else {
            false
        }
    }
    
    fn convert(&self, 
               typed_ast: &TypedAst, 
               registry: &dyn TypedConverterRegistry) -> Result<Box<ActionSystemNode<Vec<T>>>, String> {
        if let StructuredTokenInput::FilterList { .. } = &typed_ast.token {
            let array_node = convert_child::<Vec<T>>(registry, typed_ast, "array")?;
            let condition_node = convert_child::<bool>(registry, typed_ast, "condition")?;
            
            Ok(Box::new(GenericFilterListNode::new(array_node, condition_node)))
        } else {
            Err("Expected FilterList token".to_string())
        }
    }
}