use crate::{StructuredTokenInput, typed_node_converter::{TypedNodeConverter, TypedConverterRegistry, convert_child}};
use crate::type_system::{TypedAst, Type};
use action_system::*;

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