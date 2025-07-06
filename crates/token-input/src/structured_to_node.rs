// StructuredTokenInput → Node 変換

use crate::{StructuredTokenInput, RuleSet};
use action_system::{RuleNode, ConditionCheckNode, ConstantValueNode, ActingCharacterNode, CharacterHpNode, RandomConditionNode, GreaterThanConditionNode, StrikeActionNode, HealActionNode, AllCharactersNode, Character, Node, Action, FilterListNode, CharacterTeamNode, ElementNode, EnemyNode, HeroNode, TeamSide};
use action_system::nodes::condition::EqConditionNode;

// パース結果を表すEnum
pub enum ParsedResolver {
    Action(Box<dyn Node<Box<dyn Action>>>),
    Condition(Box<dyn Node<bool>>),
    Value(Box<dyn Node<i32>>),
    Character(Box<dyn Node<Character>>),
    CharacterArray(Box<dyn Node<Vec<Character>>>),
    TeamSide(Box<dyn Node<TeamSide>>),
}

impl ParsedResolver {
    pub fn require_action(self) -> Result<Box<dyn Node<Box<dyn Action>>>, String> {
        match self {
            ParsedResolver::Action(action) => Ok(action),
            _ => Err(format!("Expected Action, got different type")),
        }
    }
    
    pub fn require_condition(self) -> Result<Box<dyn Node<bool>>, String> {
        match self {
            ParsedResolver::Condition(condition) => Ok(condition),
            _ => Err(format!("Expected Condition, got different type")),
        }
    }
    
    pub fn require_value(self) -> Result<Box<dyn Node<i32>>, String> {
        match self {
            ParsedResolver::Value(value) => Ok(value),
            _ => Err(format!("Expected Value, got different type")),
        }
    }
    
    pub fn require_character(self) -> Result<Box<dyn Node<Character>>, String> {
        match self {
            ParsedResolver::Character(character_node) => Ok(character_node),
            _ => Err(format!("Expected Character, got different type")),
        }
    }
    
    pub fn require_character_array(self) -> Result<Box<dyn Node<Vec<Character>>>, String> {
        match self {
            ParsedResolver::CharacterArray(character_array_node) => Ok(character_array_node),
            _ => Err(format!("Expected CharacterArray, got different type")),
        }
    }
    
    pub fn require_team_side(self) -> Result<Box<dyn Node<TeamSide>>, String> {
        match self {
            ParsedResolver::TeamSide(team_side_node) => Ok(team_side_node),
            _ => Err(format!("Expected TeamSide, got different type")),
        }
    }
}

// StructuredTokenInput → Node 変換
pub fn convert_structured_to_node(token: &StructuredTokenInput) -> Result<ParsedResolver, String> {
    match token {
        StructuredTokenInput::Strike { target } => {
            let target_node = convert_structured_to_node(target)?;
            let character_node = target_node.require_character()?;
            Ok(ParsedResolver::Action(Box::new(StrikeActionNode::new(character_node))))
        }
        StructuredTokenInput::Heal { target } => {
            let target_node = convert_structured_to_node(target)?;
            let character_node = target_node.require_character()?;
            Ok(ParsedResolver::Action(Box::new(HealActionNode::new(character_node))))
        }
        StructuredTokenInput::Check { condition, then_action } => {
            let condition_node = convert_structured_to_node(condition)?;
            let action_node = convert_structured_to_node(then_action)?;
            let cond = condition_node.require_condition()?;
            let action = action_node.require_action()?;
            Ok(ParsedResolver::Action(Box::new(ConditionCheckNode::new(cond, action))))
        }
        StructuredTokenInput::GreaterThan { left, right } => {
            let left_node = convert_structured_to_node(left)?;
            let right_node = convert_structured_to_node(right)?;
            let left_val = left_node.require_value()?;
            let right_val = right_node.require_value()?;
            Ok(ParsedResolver::Condition(Box::new(GreaterThanConditionNode::new(left_val, right_val))))
        }
        StructuredTokenInput::HP { character } => {
            let character_node = convert_structured_to_node(character)?;
            let character_target_node = character_node.require_character()?;
            Ok(ParsedResolver::Value(Box::new(CharacterHpNode::new(character_target_node))))
        }
        StructuredTokenInput::Number { value } => {
            Ok(ParsedResolver::Value(Box::new(ConstantValueNode::new(*value))))
        }
        StructuredTokenInput::ActingCharacter => {
            // ActingCharacterNode returns Character
            Ok(ParsedResolver::Character(Box::new(ActingCharacterNode)))
        }
        StructuredTokenInput::AllCharacters => {
            Ok(ParsedResolver::CharacterArray(Box::new(AllCharactersNode::new())))
        }
        StructuredTokenInput::RandomPick { array } => {
            let array_node = convert_structured_to_node(array)?;
            let character_array_node = array_node.require_character_array()?;
            // CharacterRandomPickNode now returns Character directly
            Ok(ParsedResolver::Character(Box::new(action_system::CharacterRandomPickNode::new(character_array_node))))
        }
        StructuredTokenInput::TrueOrFalseRandom => {
            Ok(ParsedResolver::Condition(Box::new(RandomConditionNode)))
        }
        StructuredTokenInput::CharacterHP { character } => {
            let character_node = convert_structured_to_node(character)?;
            let character_target_node = character_node.require_character()?;
            Ok(ParsedResolver::Value(Box::new(CharacterHpNode::new(character_target_node))))
        }
        StructuredTokenInput::Eq { left, right } => {
            let left_node = convert_structured_to_node(left)?;
            let right_node = convert_structured_to_node(right)?;
            
            // Try to match types and create appropriate EqNode
            match (left_node, right_node) {
                // TeamSide comparison
                (ParsedResolver::TeamSide(left_team), ParsedResolver::TeamSide(right_team)) => {
                    Ok(ParsedResolver::Condition(Box::new(EqConditionNode::new(left_team, right_team))))
                },
                // Value comparison
                (ParsedResolver::Value(left_value), ParsedResolver::Value(right_value)) => {
                    Ok(ParsedResolver::Condition(Box::new(EqConditionNode::new(left_value, right_value))))
                },
                // Character comparison
                (ParsedResolver::Character(left_character), ParsedResolver::Character(right_character)) => {
                    Ok(ParsedResolver::Condition(Box::new(EqConditionNode::new(left_character, right_character))))
                },
                // Type mismatch
                _ => Err(format!("Cannot compare different types in Eq")),
            }
        }
        StructuredTokenInput::CharacterTeam { character } => {
            let character_node = convert_structured_to_node(character)?;
            let character_target_node = character_node.require_character()?;
            Ok(ParsedResolver::TeamSide(Box::new(CharacterTeamNode::new(character_target_node))))
        }
        StructuredTokenInput::FilterList { array, condition } => {
            let array_node = convert_structured_to_node(array)?;
            let condition_node = convert_structured_to_node(condition)?;
            let character_array_node = array_node.require_character_array()?;
            let condition_bool_node = condition_node.require_condition()?;
            Ok(ParsedResolver::CharacterArray(Box::new(FilterListNode::new(character_array_node, condition_bool_node))))
        }
        StructuredTokenInput::Element => {
            Ok(ParsedResolver::Character(Box::new(ElementNode::new())))
        }
        StructuredTokenInput::Enemy => {
            Ok(ParsedResolver::TeamSide(Box::new(EnemyNode::new())))
        }
        StructuredTokenInput::Hero => {
            Ok(ParsedResolver::TeamSide(Box::new(HeroNode::new())))
        }
    }
}

// RuleSet → Vec<RuleNode> 変換（JSON入力経路）
pub fn convert_ruleset_to_nodes(ruleset: &RuleSet) -> Vec<RuleNode> {
    ruleset.rules
        .iter()
        .filter_map(|rule| {
            // 各ルールをActionResolverとして変換
            match convert_structured_to_node(rule) {
                Ok(parsed) => {
                    match parsed.require_action() {
                        Ok(action) => Some(action),
                        Err(_) => None,
                    }
                }
                Err(_) => None,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structured_to_node() {
        let structured = StructuredTokenInput::Strike { 
            target: Box::new(StructuredTokenInput::ActingCharacter) 
        };
        
        let result = convert_structured_to_node(&structured).unwrap();
        assert!(result.require_action().is_ok());
    }

    #[test]
    fn test_all_characters_node_conversion() {
        let structured = StructuredTokenInput::AllCharacters;
        let result = convert_structured_to_node(&structured).unwrap();
        assert!(result.require_character_array().is_ok());
    }

    #[test]
    fn test_random_pick_node_conversion() {
        let structured = StructuredTokenInput::RandomPick { 
            array: Box::new(StructuredTokenInput::AllCharacters) 
        };
        let result = convert_structured_to_node(&structured).unwrap();
        assert!(result.require_character().is_ok());
    }

    #[test]
    fn test_enemy_hero_node_conversion() {
        let structured_enemy = StructuredTokenInput::Enemy;
        let result_enemy = convert_structured_to_node(&structured_enemy).unwrap();
        assert!(result_enemy.require_team_side().is_ok());
        
        let structured_hero = StructuredTokenInput::Hero;
        let result_hero = convert_structured_to_node(&structured_hero).unwrap();
        assert!(result_hero.require_team_side().is_ok());
    }

    #[test]
    fn test_eq_value_node_conversion() {
        let structured = StructuredTokenInput::Eq {
            left: Box::new(StructuredTokenInput::Number { value: 10 }),
            right: Box::new(StructuredTokenInput::Number { value: 10 }),
        };
        let result = convert_structured_to_node(&structured).unwrap();
        assert!(result.require_condition().is_ok());
    }

    #[test]
    fn test_eq_character_node_conversion() {
        let structured = StructuredTokenInput::Eq {
            left: Box::new(StructuredTokenInput::ActingCharacter),
            right: Box::new(StructuredTokenInput::Element),
        };
        let result = convert_structured_to_node(&structured).unwrap();
        assert!(result.require_condition().is_ok());
    }

    #[test]
    fn test_eq_team_node_conversion() {
        let structured = StructuredTokenInput::Eq {
            left: Box::new(StructuredTokenInput::Enemy),
            right: Box::new(StructuredTokenInput::Hero),
        };
        let result = convert_structured_to_node(&structured).unwrap();
        assert!(result.require_condition().is_ok());
    }
}