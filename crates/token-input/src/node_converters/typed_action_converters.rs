// 型情報を伝播させるアクションコンバーター

use crate::{StructuredTokenInput, typed_node_converter::{TypedNodeConverter, TypedConverterRegistry, convert_child}};
use crate::type_system::{TypedAst, Type};
use action_system::*;
use action_system::ConditionCheckNode as CheckActionNode;

// Type alias for Node trait with action-system's EvaluationContext
type ActionSystemNode<T> = dyn for<'a> Node<T, EvaluationContext<'a>> + Send + Sync;

/// 型情報を活用するStrikeコンバーター
pub struct TypedStrikeActionConverter;

impl TypedNodeConverter<Box<dyn Action>> for TypedStrikeActionConverter {
    fn can_convert(&self, token: &StructuredTokenInput, expected_type: &Type) -> bool {
        matches!(token, StructuredTokenInput::Strike { .. }) && 
        matches!(expected_type, Type::Action)
    }
    
    fn convert(&self, 
               typed_ast: &TypedAst, 
               registry: &dyn TypedConverterRegistry) -> Result<Box<ActionSystemNode<Box<dyn Action>>>, String> {
        if let StructuredTokenInput::Strike { .. } = &typed_ast.token {
            // 子要素の型情報を確認
            let target_ast = typed_ast.children.get("target")
                .ok_or_else(|| "Strike requires a target".to_string())?;
            let target_type = &target_ast.ty;
            
            // 型情報に基づいて適切にtargetを変換
            match target_type {
                Type::Character => {
                    let target_node = convert_child::<Character>(registry, typed_ast, "target")?;
                    Ok(Box::new(StrikeActionNode::new(target_node)))
                }
                _ => Err(format!("Strike target must be Character, got {:?}", target_type))
            }
        } else {
            Err("Expected Strike token".to_string())
        }
    }
}

/// 型情報を活用するHealコンバーター
pub struct TypedHealActionConverter;

impl TypedNodeConverter<Box<dyn Action>> for TypedHealActionConverter {
    fn can_convert(&self, token: &StructuredTokenInput, expected_type: &Type) -> bool {
        matches!(token, StructuredTokenInput::Heal { .. }) && 
        matches!(expected_type, Type::Action)
    }
    
    fn convert(&self, 
               typed_ast: &TypedAst, 
               registry: &dyn TypedConverterRegistry) -> Result<Box<ActionSystemNode<Box<dyn Action>>>, String> {
        if let StructuredTokenInput::Heal { .. } = &typed_ast.token {
            // 子要素の型情報を確認
            let target_ast = typed_ast.children.get("target")
                .ok_or_else(|| "Heal requires a target".to_string())?;
            let target_type = &target_ast.ty;
            
            // Heal の場合、amount は任意（デフォルト値を使用）
            let amount_type = typed_ast.children.get("amount")
                .map(|ast| &ast.ty)
                .unwrap_or(&Type::I32);
            
            // 型チェック
            if !matches!(target_type, Type::Character) {
                return Err(format!("Heal target must be Character, got {:?}", target_type));
            }
            if !matches!(amount_type, Type::I32) {
                return Err(format!("Heal amount must be i32, got {:?}", amount_type));
            }
            
            // 型情報に基づいて変換
            let target_node = convert_child::<Character>(registry, typed_ast, "target")?;
            Ok(Box::new(HealActionNode::new(target_node)))
        } else {
            Err("Expected Heal token".to_string())
        }
    }
}

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