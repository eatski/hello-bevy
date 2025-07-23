use crate::{StructuredTokenInput, typed_node_converter::{TypedNodeConverter, TypedConverterRegistry, convert_child}};
use crate::type_system::{TypedAst, Type};
use action_system::*;
use action_system::nodes::array::MaxNode;
use std::marker::PhantomData;

// Type alias for Node trait with action-system's EvaluationContext
type ActionSystemNode<T> = dyn for<'a> Node<T, EvaluationContext<'a>> + Send + Sync;

/// 型情報を活用するMaxコンバーター（Numeric型）
pub struct TypedMaxConverter<T>
where
    T: Numeric + Clone + Send + Sync + 'static,
{
    _phantom: PhantomData<T>,
}

impl<T> TypedMaxConverter<T>
where
    T: Numeric + Clone + Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<T> TypedNodeConverter<T> for TypedMaxConverter<T>
where
    T: Numeric + Clone + Send + Sync + 'static,
{
    fn can_convert(&self, token: &StructuredTokenInput, _expected_type: &Type) -> bool {
        matches!(token, StructuredTokenInput::NumericMax { .. })
    }
    
    fn convert(&self, 
               typed_ast: &TypedAst, 
               registry: &dyn TypedConverterRegistry) -> Result<Box<ActionSystemNode<T>>, String> {
        if let StructuredTokenInput::NumericMax { .. } = &typed_ast.token {
            let array_node = convert_child::<Vec<T>>(registry, typed_ast, "array")?;
            Ok(Box::new(MaxNode::new(array_node)))
        } else {
            Err("Expected NumericMax token".to_string())
        }
    }
}

/// Character専用のMaxコンバーター（Max -> CharacterのマッピングがAPIで存在するため）
pub struct TypedMaxCharacterConverter;

impl TypedNodeConverter<Character> for TypedMaxCharacterConverter {
    fn can_convert(&self, token: &StructuredTokenInput, expected_type: &Type) -> bool {
        matches!(token, StructuredTokenInput::Max { .. }) && 
        matches!(expected_type, Type::Character)
    }
    
    fn convert(&self, 
               typed_ast: &TypedAst, 
               registry: &dyn TypedConverterRegistry) -> Result<Box<ActionSystemNode<Character>>, String> {
        if let StructuredTokenInput::Max { .. } = &typed_ast.token {
            let array_node = convert_child::<Vec<Character>>(registry, typed_ast, "array")?;
            Ok(Box::new(MaxNodeCharacter::new(array_node)))
        } else {
            Err("Expected Max token".to_string())
        }
    }
}