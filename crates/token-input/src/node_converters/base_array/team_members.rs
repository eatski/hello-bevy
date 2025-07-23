use crate::{StructuredTokenInput, typed_node_converter::{TypedNodeConverter, TypedConverterRegistry, convert_child}};
use crate::type_system::{TypedAst, Type};
use action_system::*;

// Type alias for Node trait with action-system's EvaluationContext
type ActionSystemNode<T> = dyn for<'a> Node<T, EvaluationContext<'a>> + Send + Sync;

/// 型情報を活用するTeamMembersコンバーター
pub struct TypedTeamMembersConverter;

impl TypedNodeConverter<Vec<Character>> for TypedTeamMembersConverter {
    fn can_convert(&self, token: &StructuredTokenInput, expected_type: &Type) -> bool {
        matches!(token, StructuredTokenInput::TeamMembers { .. }) && 
        if let Type::Vec(elem_type) = expected_type {
            matches!(**elem_type, Type::Character)
        } else {
            false
        }
    }
    
    fn convert(&self, 
               typed_ast: &TypedAst, 
               registry: &dyn TypedConverterRegistry) -> Result<Box<ActionSystemNode<Vec<Character>>>, String> {
        if let StructuredTokenInput::TeamMembers { .. } = &typed_ast.token {
            // 子要素の型情報を確認
            let team_ast = typed_ast.children.get("team_side")
                .ok_or_else(|| "TeamMembers requires a team_side".to_string())?;
            let team_type = &team_ast.ty;
            
            // 型チェック
            if !matches!(team_type, Type::TeamSide) {
                return Err(format!("TeamMembers team must be TeamSide, got {:?}", team_type));
            }
            
            // 型情報に基づいて変換
            let team_node = convert_child::<TeamSide>(registry, typed_ast, "team_side")?;
            // TeamMembersNodeWithNodeを使用
            Ok(Box::new(TeamMembersNode::new_with_node(team_node)))
        } else {
            Err("Expected TeamMembers token".to_string())
        }
    }
}