// StructuredTokenInputからaction_system::Nodeへの変換

use crate::StructuredTokenInput;
use action_system::{
    RuleNode, Node as CoreNode, EvaluationContext, Numeric,
    StrikeActionNode, HealActionNode, ConditionCheckNode,
    RandomConditionNode, GreaterThanNode, LessThanNode,
    nodes::condition::EqConditionNode,
    ActingCharacterNode,
    AllCharactersNode, TeamMembersNode, AllTeamSidesNode,
    RandomPickNode, FilterListNode,
    CharacterToHpNode, CharacterHpToCharacterNode, CharacterTeamNode,
    EnemyNode, HeroNode, ElementNode,
    MaxNode, MinNode,
    Character, CharacterHP, TeamSide, Action,
    ConstantValueNode, NumericNode,
};


/// StructuredTokenInputをRuleNodeに変換
pub fn convert_to_rule_node(token: &StructuredTokenInput) -> Option<RuleNode> {
    convert_to_action_node(token)
}

/// アクションノードへの変換
fn convert_to_action_node(token: &StructuredTokenInput) -> Option<Box<dyn for<'a> CoreNode<Box<dyn Action>, EvaluationContext<'a>> + Send + Sync>> {
    match token {
        StructuredTokenInput::Strike { target } => {
            let target_node = convert_to_character_node(target)?;
            Some(Box::new(StrikeActionNode::new(target_node)))
        }
        StructuredTokenInput::Heal { target } => {
            let target_node = convert_to_character_node(target)?;
            Some(Box::new(HealActionNode::new(target_node)))
        }
        StructuredTokenInput::Check { condition, then_action } => {
            let condition_node = convert_to_bool_node(condition)?;
            let action_node = convert_to_action_node(then_action)?;
            Some(Box::new(ConditionCheckNode::new(condition_node, action_node)))
        }
        _ => None,
    }
}

/// 条件ノードへの変換
fn convert_to_bool_node(token: &StructuredTokenInput) -> Option<Box<dyn for<'a> CoreNode<bool, EvaluationContext<'a>> + Send + Sync>> {
    match token {
        StructuredTokenInput::TrueOrFalseRandom => {
            Some(Box::new(RandomConditionNode))
        }
        StructuredTokenInput::GreaterThan { left, right } => {
            convert_greater_than(left, right)
        }
        StructuredTokenInput::LessThan { left, right } => {
            convert_less_than(left, right)
        }
        StructuredTokenInput::Eq { left, right } => {
            // 型を推論して適切なEqNodeを作成
            if let (Some(left_i32), Some(right_i32)) = (convert_to_i32_node(left), convert_to_i32_node(right)) {
                Some(Box::new(EqConditionNode::new(left_i32, right_i32)))
            } else if let (Some(left_char), Some(right_char)) = (convert_to_character_node(left), convert_to_character_node(right)) {
                Some(Box::new(EqConditionNode::new(left_char, right_char)))
            } else if let (Some(left_hp), Some(right_hp)) = (convert_to_character_hp_node(left), convert_to_character_hp_node(right)) {
                Some(Box::new(EqConditionNode::new(left_hp, right_hp)))
            } else if let (Some(left_team), Some(right_team)) = (convert_to_team_side_node(left), convert_to_team_side_node(right)) {
                Some(Box::new(EqConditionNode::new(left_team, right_team)))
            } else {
                None
            }
        }
        _ => None,
    }
}

/// GreaterThanの変換（型を推論して適切なノードを作成）
fn convert_greater_than(
    left: &StructuredTokenInput,
    right: &StructuredTokenInput,
) -> Option<Box<dyn for<'a> CoreNode<bool, EvaluationContext<'a>> + Send + Sync>> {
    // 左右をBox<dyn Numeric>ノードに変換
    let left_numeric = convert_to_numeric_node(left)?;
    let right_numeric = convert_to_numeric_node(right)?;
    Some(Box::new(GreaterThanNode::new(left_numeric, right_numeric)))
}

/// LessThanの変換（型を推論して適切なノードを作成）
fn convert_less_than(
    left: &StructuredTokenInput,
    right: &StructuredTokenInput,
) -> Option<Box<dyn for<'a> CoreNode<bool, EvaluationContext<'a>> + Send + Sync>> {
    // 左右をBox<dyn Numeric>ノードに変換
    let left_numeric = convert_to_numeric_node(left)?;
    let right_numeric = convert_to_numeric_node(right)?;
    Some(Box::new(LessThanNode::new(left_numeric, right_numeric)))
}

/// キャラクターノードへの変換
fn convert_to_character_node(token: &StructuredTokenInput) -> Option<Box<dyn for<'a> CoreNode<Character, EvaluationContext<'a>> + Send + Sync>> {
    match token {
        StructuredTokenInput::ActingCharacter => {
            Some(Box::new(ActingCharacterNode))
        }
        StructuredTokenInput::RandomPick { array } => {
            let array_node = convert_to_character_array_node(array)?;
            Some(Box::new(RandomPickNode::new(array_node)))
        }
        StructuredTokenInput::CharacterHpToCharacter { character_hp } => {
            let hp_node = convert_to_character_hp_node(character_hp)?;
            Some(Box::new(CharacterHpToCharacterNode::new(hp_node)))
        }
        StructuredTokenInput::Max { array } => {
            let array_node = convert_to_character_array_node(array)?;
            Some(Box::new(MaxNode::new(array_node)))
        }
        StructuredTokenInput::Min { array } => {
            let array_node = convert_to_character_array_node(array)?;
            Some(Box::new(MinNode::new(array_node)))
        }
        StructuredTokenInput::Element => {
            // ElementはFilterListのcontext内で使用される
            Some(Box::new(ElementNode::new()))
        }
        _ => None,
    }
}

/// キャラクター配列ノードへの変換
fn convert_to_character_array_node(token: &StructuredTokenInput) -> Option<Box<dyn for<'a> CoreNode<Vec<Character>, EvaluationContext<'a>> + Send + Sync>> {
    match token {
        StructuredTokenInput::AllCharacters => {
            Some(Box::new(AllCharactersNode))
        }
        StructuredTokenInput::TeamMembers { team_side } => {
            // TeamMembersNodeはTeamSideを直接受け取る
            match team_side.as_ref() {
                StructuredTokenInput::Enemy => Some(Box::new(TeamMembersNode::new(TeamSide::Enemy))),
                StructuredTokenInput::Hero => Some(Box::new(TeamMembersNode::new(TeamSide::Player))),
                _ => None,
            }
        }
        StructuredTokenInput::FilterList { array, condition } => {
            let array_node = convert_to_character_array_node(array)?;
            let condition_node = convert_to_bool_node(condition)?;
            Some(Box::new(FilterListNode::new(array_node, condition_node)))
        }
        StructuredTokenInput::Map { array: _, transform: _ } => {
            // キャラクター配列からキャラクター配列へのMapは現在サポートされていない
            // （変換先が同じ型である必要があるため）
            None
        }
        _ => None,
    }
}

/// CharacterHPノードへの変換
fn convert_to_character_hp_node(token: &StructuredTokenInput) -> Option<Box<dyn for<'a> CoreNode<CharacterHP, EvaluationContext<'a>> + Send + Sync>> {
    match token {
        StructuredTokenInput::CharacterToHp { character } => {
            let char_node = convert_to_character_node(character)?;
            Some(Box::new(CharacterToHpNode::new(char_node)))
        }
        _ => None,
    }
}

/// i32ノードへの変換
fn convert_to_i32_node(token: &StructuredTokenInput) -> Option<Box<dyn for<'a> CoreNode<i32, EvaluationContext<'a>> + Send + Sync>> {
    match token {
        StructuredTokenInput::Number { value } => {
            Some(Box::new(ConstantValueNode::new(*value)))
        }
        _ => None,
    }
}

/// TeamSideノードへの変換
fn convert_to_team_side_node(token: &StructuredTokenInput) -> Option<Box<dyn for<'a> CoreNode<TeamSide, EvaluationContext<'a>> + Send + Sync>> {
    match token {
        StructuredTokenInput::Enemy => {
            Some(Box::new(EnemyNode))
        }
        StructuredTokenInput::Hero => {
            Some(Box::new(HeroNode))
        }
        StructuredTokenInput::CharacterTeam { character } => {
            let char_node = convert_to_character_node(character)?;
            Some(Box::new(CharacterTeamNode::new(char_node)))
        }
        StructuredTokenInput::RandomPick { array } => {
            // TeamSide配列のランダム選択は現在サポートされていない
            let _ = convert_to_team_side_array_node(array)?;
            None
        }
        _ => None,
    }
}

/// TeamSide配列ノードへの変換
fn convert_to_team_side_array_node(token: &StructuredTokenInput) -> Option<Box<dyn for<'a> CoreNode<Vec<TeamSide>, EvaluationContext<'a>> + Send + Sync>> {
    match token {
        StructuredTokenInput::AllTeamSides => {
            Some(Box::new(AllTeamSidesNode))
        }
        _ => None,
    }
}

/// Box<dyn Numeric>ノードへの変換
fn convert_to_numeric_node(token: &StructuredTokenInput) -> Option<Box<dyn for<'a> CoreNode<Box<dyn Numeric>, EvaluationContext<'a>> + Send + Sync>> {
    // i32ノードの場合はNumericNodeでラップ
    if let Some(i32_node) = convert_to_i32_node(token) {
        return Some(Box::new(NumericNode::new(i32_node)));
    }
    
    // CharacterHPノードの場合はNumericNodeでラップ
    if let Some(hp_node) = convert_to_character_hp_node(token) {
        return Some(Box::new(NumericNode::new(hp_node)));
    }
    
    // Characterノードの場合はNumericNodeでラップ
    if let Some(char_node) = convert_to_character_node(token) {
        return Some(Box::new(NumericNode::new(char_node)));
    }
    
    None
}