// 型情報を伝播させる基本配列コンバーター

use crate::{StructuredTokenInput, typed_node_converter::{TypedNodeConverter, TypedConverterRegistry, convert_child}};
use crate::type_system::{TypedAst, Type};
use action_system::*;

// Type alias for Node trait with action-system's EvaluationContext
type ActionSystemNode<T> = dyn for<'a> Node<T, EvaluationContext<'a>> + Send + Sync;

/// 型情報を活用するAllCharactersコンバーター
pub struct TypedAllCharactersConverter;

impl TypedNodeConverter<Vec<Character>> for TypedAllCharactersConverter {
    fn can_convert(&self, token: &StructuredTokenInput, expected_type: &Type) -> bool {
        matches!(token, StructuredTokenInput::AllCharacters) && 
        matches!(expected_type, Type::Vec(elem) if **elem == Type::Character)
    }
    
    fn convert(&self, 
               _typed_ast: &TypedAst, 
               _registry: &dyn TypedConverterRegistry) -> Result<Box<ActionSystemNode<Vec<Character>>>, String> {
        Ok(Box::new(AllCharactersNode::new()))
    }
}

/// 型情報を活用するTeamMembersコンバーター
pub struct TypedTeamMembersConverter;

impl TypedNodeConverter<Vec<Character>> for TypedTeamMembersConverter {
    fn can_convert(&self, token: &StructuredTokenInput, expected_type: &Type) -> bool {
        matches!(token, StructuredTokenInput::TeamMembers { .. }) && 
        matches!(expected_type, Type::Vec(elem) if **elem == Type::Character)
    }
    
    fn convert(&self, 
               typed_ast: &TypedAst, 
               registry: &dyn TypedConverterRegistry) -> Result<Box<ActionSystemNode<Vec<Character>>>, String> {
        if let StructuredTokenInput::TeamMembers { team_side } = &typed_ast.token {
            // 静的なTeamSideの場合とNodeの場合を処理
            match team_side.as_ref() {
                StructuredTokenInput::Enemy => Ok(Box::new(TeamMembersNode::new(TeamSide::Enemy))),
                StructuredTokenInput::Hero => Ok(Box::new(TeamMembersNode::new(TeamSide::Player))),
                _ => {
                    // 動的なTeamSide評価 - 子要素があることを確認
                    let team_side_ast = typed_ast.children.get("team_side")
                        .ok_or_else(|| "TeamMembers with dynamic team_side requires child AST".to_string())?;
                    let team_side_type = &team_side_ast.ty;
                    
                    // 型チェック
                    if !matches!(team_side_type, Type::TeamSide) {
                        return Err(format!("TeamMembers requires TeamSide, got {:?}", team_side_type));
                    }
                    
                    let team_side_node = convert_child::<TeamSide>(registry, typed_ast, "team_side")?;
                    Ok(Box::new(TeamMembersNode::new_with_node(team_side_node)))
                }
            }
        } else {
            Err("Expected TeamMembers token".to_string())
        }
    }
}

/// 型情報を活用するAllTeamSidesコンバーター
pub struct TypedAllTeamSidesConverter;

impl TypedNodeConverter<Vec<TeamSide>> for TypedAllTeamSidesConverter {
    fn can_convert(&self, token: &StructuredTokenInput, expected_type: &Type) -> bool {
        matches!(token, StructuredTokenInput::AllTeamSides) && 
        matches!(expected_type, Type::Vec(elem) if **elem == Type::TeamSide)
    }
    
    fn convert(&self, 
               _typed_ast: &TypedAst, 
               _registry: &dyn TypedConverterRegistry) -> Result<Box<ActionSystemNode<Vec<TeamSide>>>, String> {
        Ok(Box::new(AllTeamSidesNode::new()))
    }
}