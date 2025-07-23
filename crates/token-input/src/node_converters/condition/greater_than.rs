use crate::{StructuredTokenInput, typed_node_converter::{TypedNodeConverter, TypedConverterRegistry, convert_child}};
use crate::type_system::{TypedAst, Type};
use action_system::*;
use action_system::nodes::condition::{GreaterThanNode, CharacterHpVsValueGreaterThanNode, ValueVsCharacterHpGreaterThanNode};

// Type alias for Node trait with action-system's EvaluationContext
type ActionSystemNode<T> = dyn for<'a> Node<T, EvaluationContext<'a>> + Send + Sync;

/// 型情報を活用するGreaterThanコンバーター
pub struct TypedGreaterThanConverter;

impl TypedGreaterThanConverter {
    /// Numeric型の実際の型を推論
    fn infer_numeric_type(ast: &TypedAst) -> Type {
        match &ast.token {
            StructuredTokenInput::NumericMax { .. } | StructuredTokenInput::NumericMin { .. } => {
                // Check the array element type
                if let Some(array_ast) = ast.children.get("array") {
                    if let Type::Vec(elem_type) = &array_ast.ty {
                        return *elem_type.clone();
                    }
                }
                Type::I32 // default
            }
            _ => Type::I32 // default for other numeric types
        }
    }
}

impl TypedNodeConverter<bool> for TypedGreaterThanConverter {
    fn can_convert(&self, token: &StructuredTokenInput, expected_type: &Type) -> bool {
        matches!(token, StructuredTokenInput::GreaterThan { .. }) && 
        matches!(expected_type, Type::Bool)
    }
    
    fn convert(&self, 
               typed_ast: &TypedAst, 
               registry: &dyn TypedConverterRegistry) -> Result<Box<ActionSystemNode<bool>>, String> {
        if let StructuredTokenInput::GreaterThan { .. } = &typed_ast.token {
            // 子要素の型情報を確認
            let left_ast = typed_ast.children.get("left")
                .ok_or_else(|| "GreaterThan requires a left argument".to_string())?;
            let right_ast = typed_ast.children.get("right")
                .ok_or_else(|| "GreaterThan requires a right argument".to_string())?;
            
            let left_type = &left_ast.ty;
            let right_type = &right_ast.ty;
            
            // Numeric型の実際の型を推論
            let actual_left_type = if matches!(left_type, Type::Numeric) {
                Self::infer_numeric_type(left_ast)
            } else {
                left_type.clone()
            };
            
            let actual_right_type = if matches!(right_type, Type::Numeric) {
                Self::infer_numeric_type(right_ast)
            } else {
                right_type.clone()
            };
            
            // 型の組み合わせに基づいて適切なNodeを選択
            match (&actual_left_type, &actual_right_type) {
                (Type::I32, Type::I32) => {
                    let left_node = convert_child::<i32>(registry, typed_ast, "left")?;
                    let right_node = convert_child::<i32>(registry, typed_ast, "right")?;
                    Ok(Box::new(GreaterThanNode::<i32>::new(left_node, right_node)))
                }
                (Type::CharacterHP, Type::CharacterHP) => {
                    let left_node = convert_child::<CharacterHP>(registry, typed_ast, "left")?;
                    let right_node = convert_child::<CharacterHP>(registry, typed_ast, "right")?;
                    Ok(Box::new(GreaterThanNode::<CharacterHP>::new(left_node, right_node)))
                }
                (Type::CharacterHP, Type::I32) => {
                    let left_node = convert_child::<CharacterHP>(registry, typed_ast, "left")?;
                    let right_node = convert_child::<i32>(registry, typed_ast, "right")?;
                    Ok(Box::new(CharacterHpVsValueGreaterThanNode::new(left_node, right_node)))
                }
                (Type::I32, Type::CharacterHP) => {
                    let left_node = convert_child::<i32>(registry, typed_ast, "left")?;
                    let right_node = convert_child::<CharacterHP>(registry, typed_ast, "right")?;
                    Ok(Box::new(ValueVsCharacterHpGreaterThanNode::new(left_node, right_node)))
                }
                _ => Err(format!("GreaterThan not supported for types {:?} and {:?}", 
                               actual_left_type, actual_right_type))
            }
        } else {
            Err("Expected GreaterThan token".to_string())
        }
    }
}