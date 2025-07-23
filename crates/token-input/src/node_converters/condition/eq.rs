use crate::{StructuredTokenInput, typed_node_converter::{TypedNodeConverter, TypedConverterRegistry, convert_child}};
use crate::type_system::{TypedAst, Type};
use action_system::*;
use action_system::nodes::condition::{EqConditionNode, TeamSideEqNode};

// Type alias for Node trait with action-system's EvaluationContext
type ActionSystemNode<T> = dyn for<'a> Node<T, EvaluationContext<'a>> + Send + Sync;

/// 型情報を活用するEqコンバーター
pub struct TypedEqConverter;

impl TypedNodeConverter<bool> for TypedEqConverter {
    fn can_convert(&self, token: &StructuredTokenInput, expected_type: &Type) -> bool {
        matches!(token, StructuredTokenInput::Eq { .. }) && 
        matches!(expected_type, Type::Bool)
    }
    
    fn convert(&self, 
               typed_ast: &TypedAst, 
               registry: &dyn TypedConverterRegistry) -> Result<Box<ActionSystemNode<bool>>, String> {
        if let StructuredTokenInput::Eq { .. } = &typed_ast.token {
            // 子要素の型情報を確認
            let left_ast = typed_ast.children.get("left")
                .ok_or_else(|| "Eq requires a left argument".to_string())?;
            let right_ast = typed_ast.children.get("right")
                .ok_or_else(|| "Eq requires a right argument".to_string())?;
            
            let left_type = &left_ast.ty;
            let right_type = &right_ast.ty;
            
            // 型の組み合わせに基づいて適切なNodeを選択
            match (left_type, right_type) {
                (Type::I32, Type::I32) => {
                    let left_node = convert_child::<i32>(registry, typed_ast, "left")?;
                    let right_node = convert_child::<i32>(registry, typed_ast, "right")?;
                    Ok(Box::new(EqConditionNode::new(left_node, right_node)))
                }
                (Type::TeamSide, Type::TeamSide) => {
                    let left_node = convert_child::<TeamSide>(registry, typed_ast, "left")?;
                    let right_node = convert_child::<TeamSide>(registry, typed_ast, "right")?;
                    Ok(Box::new(TeamSideEqNode::new(left_node, right_node)))
                }
                _ => Err(format!("Eq not supported for types {:?} and {:?}", left_type, right_type))
            }
        } else {
            Err("Expected Eq token".to_string())
        }
    }
}