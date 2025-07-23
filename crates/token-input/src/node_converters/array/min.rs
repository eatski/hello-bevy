use crate::{StructuredTokenInput, typed_node_converter::{TypedNodeConverter, TypedConverterRegistry, convert_child}};
use crate::type_system::{TypedAst, Type};
use action_system::*;
use action_system::nodes::array::MinNode;
use std::marker::PhantomData;

// Type alias for Node trait with action-system's EvaluationContext
type ActionSystemNode<T> = dyn for<'a> Node<T, EvaluationContext<'a>> + Send + Sync;

/// 型情報を活用するMinコンバーター（Numeric型）
pub struct TypedMinConverter<T>
where
    T: Numeric + Clone + Send + Sync + 'static,
{
    _phantom: PhantomData<T>,
}

impl<T> TypedMinConverter<T>
where
    T: Numeric + Clone + Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<T> TypedNodeConverter<T> for TypedMinConverter<T>
where
    T: Numeric + Clone + Send + Sync + 'static,
{
    fn can_convert(&self, token: &StructuredTokenInput, _expected_type: &Type) -> bool {
        matches!(token, StructuredTokenInput::NumericMin { .. })
    }
    
    fn convert(&self, 
               typed_ast: &TypedAst, 
               registry: &dyn TypedConverterRegistry) -> Result<Box<ActionSystemNode<T>>, String> {
        if let StructuredTokenInput::NumericMin { .. } = &typed_ast.token {
            let array_node = convert_child::<Vec<T>>(registry, typed_ast, "array")?;
            Ok(Box::new(MinNode::new(array_node)))
        } else {
            Err("Expected NumericMin token".to_string())
        }
    }
}