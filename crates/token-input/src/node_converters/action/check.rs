use crate::{StructuredTokenInput, typed_node_converter::{TypedNodeConverter, TypedConverterRegistry, convert_child}};
use crate::type_system::{TypedAst, Type};
use action_system::*;
use action_system::ConditionCheckNode as CheckActionNode;

// Type alias for Node trait with action-system's EvaluationContext
type ActionSystemNode<T> = dyn for<'a> Node<T, EvaluationContext<'a>> + Send + Sync;

/// 型情報を活用するCheckコンバーター
pub struct TypedCheckActionConverter;

impl TypedNodeConverter<Box<dyn Action>> for TypedCheckActionConverter {
    fn can_convert(&self, token: &StructuredTokenInput, expected_type: &Type) -> bool {
        matches!(token, StructuredTokenInput::Check { .. }) && 
        matches!(expected_type, Type::Action)
    }
    
    fn convert(&self, 
               typed_ast: &TypedAst, 
               registry: &dyn TypedConverterRegistry) -> Result<Box<ActionSystemNode<Box<dyn Action>>>, String> {
        if let StructuredTokenInput::Check { .. } = &typed_ast.token {
            // 子要素の型情報を確認
            let condition_ast = typed_ast.children.get("condition")
                .ok_or_else(|| "Check requires a condition".to_string())?;
            let action_ast = typed_ast.children.get("then_action")
                .ok_or_else(|| "Check requires a then_action".to_string())?;
            
            let condition_type = &condition_ast.ty;
            let action_type = &action_ast.ty;
            
            // 型チェック
            if !matches!(condition_type, Type::Bool) {
                return Err(format!("Check condition must be Bool, got {:?}", condition_type));
            }
            if !matches!(action_type, Type::Action) {
                return Err(format!("Check then_action must be Action, got {:?}", action_type));
            }
            
            // 型情報に基づいて変換
            let condition_node = convert_child::<bool>(registry, typed_ast, "condition")?;
            let action_node = convert_child::<Box<dyn Action>>(registry, typed_ast, "then_action")?;
            Ok(Box::new(CheckActionNode::new(condition_node, action_node)))
        } else {
            Err("Expected Check token".to_string())
        }
    }
}