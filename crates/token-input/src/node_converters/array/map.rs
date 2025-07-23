use crate::{StructuredTokenInput, typed_node_converter::{TypedNodeConverter, TypedConverterRegistry, convert_child}};
use crate::type_system::{TypedAst, Type};
use action_system::*;
use action_system::nodes::array::{MappingNode, AsCurrentElement};
use std::marker::PhantomData;

// Type alias for Node trait with action-system's EvaluationContext
type ActionSystemNode<T> = dyn for<'a> Node<T, EvaluationContext<'a>> + Send + Sync;

/// 型情報を活用するMapコンバーター（ジェネリック）
pub struct TypedMapConverter<TInput, TOutput> 
where
    TInput: Clone + Send + Sync + 'static,
    TOutput: Clone + Send + Sync + 'static,
{
    _phantom: PhantomData<(TInput, TOutput)>,
}

impl<TInput, TOutput> TypedMapConverter<TInput, TOutput>
where
    TInput: Clone + Send + Sync + 'static,
    TOutput: Clone + Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<TInput, TOutput> TypedNodeConverter<Vec<TOutput>> for TypedMapConverter<TInput, TOutput>
where
    TInput: Clone + Send + Sync + AsCurrentElement + 'static,
    TOutput: Clone + Send + Sync + 'static,
{
    fn can_convert(&self, token: &StructuredTokenInput, expected_type: &Type) -> bool {
        matches!(token, StructuredTokenInput::Map { .. }) && 
        matches!(expected_type, Type::Vec(_))
    }
    
    fn convert(&self, 
               typed_ast: &TypedAst, 
               registry: &dyn TypedConverterRegistry) -> Result<Box<ActionSystemNode<Vec<TOutput>>>, String> {
        if let StructuredTokenInput::Map { .. } = &typed_ast.token {
            // 子要素の型情報を確認
            if !typed_ast.children.contains_key("array") || !typed_ast.children.contains_key("transform") {
                return Err("Map requires array and transform arguments".to_string());
            }
            
            let array_node = convert_child::<Vec<TInput>>(registry, typed_ast, "array")?;
            let transform_node = convert_child::<TOutput>(registry, typed_ast, "transform")?;
            
            Ok(Box::new(MappingNode::new(array_node, transform_node)))
        } else {
            Err("Expected Map token".to_string())
        }
    }
}