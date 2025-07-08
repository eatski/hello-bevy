// StructuredTokenInput → Node 変換

use crate::{StructuredTokenInput, RuleSet};
use action_system::{RuleNode, ConditionCheckNode, ConstantValueNode, ActingCharacterNode, CharacterHpNode, RandomConditionNode, GreaterThanConditionNode, StrikeActionNode, HealActionNode, AllCharactersNode, Character, Node, Action, FilterListNode, CharacterTeamNode, ElementNode, EnemyNode, HeroNode, TeamSide, CharacterToCharacterMappingNode, CharacterToValueMappingNode, ValueToValueMappingNode, ValueToCharacterMappingNode};
use action_system::nodes::condition::EqConditionNode;
use std::any::Any;

// パース結果を表すAnyベースのResolver
pub struct ParsedResolver {
    pub(crate) node: Box<dyn Any>,
    pub(crate) type_name: String,
}

impl ParsedResolver {
    pub fn new<T: Any + 'static>(node: T, type_name: String) -> Self {
        Self {
            node: Box::new(node),
            type_name,
        }
    }
    
    pub fn require_action(self) -> Result<Box<dyn Node<Box<dyn Action>>>, String> {
        match self.node.downcast::<Box<dyn Node<Box<dyn Action>>>>() {
            Ok(action) => Ok(*action),
            Err(_) => Err(format!("Expected Action, got {}", self.type_name)),
        }
    }
    
    pub fn require_condition(self) -> Result<Box<dyn Node<bool>>, String> {
        match self.node.downcast::<Box<dyn Node<bool>>>() {
            Ok(condition) => Ok(*condition),
            Err(_) => Err(format!("Expected Condition, got {}", self.type_name)),
        }
    }
    
    pub fn require_value(self) -> Result<Box<dyn Node<i32>>, String> {
        match self.node.downcast::<Box<dyn Node<i32>>>() {
            Ok(value) => Ok(*value),
            Err(_) => Err(format!("Expected Value, got {}", self.type_name)),
        }
    }
    
    pub fn require_character(self) -> Result<Box<dyn Node<Character>>, String> {
        match self.node.downcast::<Box<dyn Node<Character>>>() {
            Ok(character_node) => Ok(*character_node),
            Err(_) => Err(format!("Expected Character, got {}", self.type_name)),
        }
    }
    
    pub fn require_character_array(self) -> Result<Box<dyn Node<Vec<Character>>>, String> {
        match self.node.downcast::<Box<dyn Node<Vec<Character>>>>() {
            Ok(character_array_node) => Ok(*character_array_node),
            Err(_) => Err(format!("Expected CharacterArray, got {}", self.type_name)),
        }
    }
    
    pub fn require_value_array(self) -> Result<Box<dyn Node<Vec<i32>>>, String> {
        match self.node.downcast::<Box<dyn Node<Vec<i32>>>>() {
            Ok(value_array_node) => Ok(*value_array_node),
            Err(_) => Err(format!("Expected ValueArray, got {}", self.type_name)),
        }
    }
    
    pub fn require_team_side(self) -> Result<Box<dyn Node<TeamSide>>, String> {
        match self.node.downcast::<Box<dyn Node<TeamSide>>>() {
            Ok(team_side_node) => Ok(*team_side_node),
            Err(_) => Err(format!("Expected TeamSide, got {}", self.type_name)),
        }
    }
}

// Simple macro that auto-generates type combination tests
// Adding new types only requires updating this single list
macro_rules! try_all_mapping_combinations {
    (
        $array:expr, $transform:expr;
        $(($array_method:ident, $transform_method:ident, $mapping_node:ident, $result_type:ty, $type_name:expr)),*
    ) => {
        $(
            if let (Ok(array_node), Ok(transform_node)) = (
                convert_structured_to_node($array)?.$array_method(),
                convert_structured_to_node($transform)?.$transform_method()
            ) {
                return Ok(ParsedResolver::new(
                    Box::new($mapping_node::new(array_node, transform_node)) as Box<dyn Node<$result_type>>,
                    $type_name.to_string()
                ));
            }
        )*
    };
}

fn convert_map_token(array: &StructuredTokenInput, transform: &StructuredTokenInput) -> Result<ParsedResolver, String> {
    // All mapping combinations are automatically tried
    // To add new types: just add a new line below following the same pattern
    try_all_mapping_combinations!(
        array, transform;
        (require_character_array, require_character, CharacterToCharacterMappingNode, Vec<Character>, "CharacterArray"),
        (require_character_array, require_value, CharacterToValueMappingNode, Vec<i32>, "ValueArray"),
        (require_value_array, require_value, ValueToValueMappingNode, Vec<i32>, "ValueArray"),
        (require_value_array, require_character, ValueToCharacterMappingNode, Vec<Character>, "CharacterArray")
        // NEW TYPES GO HERE - no other changes needed anywhere else!
        // Example: (require_team_side_array, require_team_side, TeamSideToTeamSideMappingNode, Vec<TeamSide>, "TeamSideArray")
    );
    
    Err(format!("Cannot determine mapping type for Map - no compatible array→transform combination found"))
}

// StructuredTokenInput → Node 変換
pub fn convert_structured_to_node(token: &StructuredTokenInput) -> Result<ParsedResolver, String> {
    match token {
        StructuredTokenInput::Strike { target } => {
            let target_node = convert_structured_to_node(target)?;
            let character_node = target_node.require_character()?;
            Ok(ParsedResolver::new(
                Box::new(StrikeActionNode::new(character_node)) as Box<dyn Node<Box<dyn Action>>>,
                "Action".to_string()
            ))
        }
        StructuredTokenInput::Heal { target } => {
            let target_node = convert_structured_to_node(target)?;
            let character_node = target_node.require_character()?;
            Ok(ParsedResolver::new(
                Box::new(HealActionNode::new(character_node)) as Box<dyn Node<Box<dyn Action>>>,
                "Action".to_string()
            ))
        }
        StructuredTokenInput::Check { condition, then_action } => {
            let condition_node = convert_structured_to_node(condition)?;
            let action_node = convert_structured_to_node(then_action)?;
            let cond = condition_node.require_condition()?;
            let action = action_node.require_action()?;
            Ok(ParsedResolver::new(
                Box::new(ConditionCheckNode::new(cond, action)) as Box<dyn Node<Box<dyn Action>>>,
                "Action".to_string()
            ))
        }
        StructuredTokenInput::GreaterThan { left, right } => {
            let left_node = convert_structured_to_node(left)?;
            let right_node = convert_structured_to_node(right)?;
            let left_val = left_node.require_value()?;
            let right_val = right_node.require_value()?;
            Ok(ParsedResolver::new(
                Box::new(GreaterThanConditionNode::new(left_val, right_val)) as Box<dyn Node<bool>>,
                "Condition".to_string()
            ))
        }
        StructuredTokenInput::HP { character } => {
            let character_node = convert_structured_to_node(character)?;
            let character_target_node = character_node.require_character()?;
            Ok(ParsedResolver::new(
                Box::new(CharacterHpNode::new(character_target_node)) as Box<dyn Node<i32>>,
                "Value".to_string()
            ))
        }
        StructuredTokenInput::Number { value } => {
            Ok(ParsedResolver::new(
                Box::new(ConstantValueNode::new(*value)) as Box<dyn Node<i32>>,
                "Value".to_string()
            ))
        }
        StructuredTokenInput::ActingCharacter => {
            Ok(ParsedResolver::new(
                Box::new(ActingCharacterNode) as Box<dyn Node<Character>>,
                "Character".to_string()
            ))
        }
        StructuredTokenInput::AllCharacters => {
            Ok(ParsedResolver::new(
                Box::new(AllCharactersNode::new()) as Box<dyn Node<Vec<Character>>>,
                "CharacterArray".to_string()
            ))
        }
        StructuredTokenInput::RandomPick { array } => {
            let array_node = convert_structured_to_node(array)?;
            let character_array_node = array_node.require_character_array()?;
            Ok(ParsedResolver::new(
                Box::new(action_system::CharacterRandomPickNode::new(character_array_node)) as Box<dyn Node<Character>>,
                "Character".to_string()
            ))
        }
        StructuredTokenInput::TrueOrFalseRandom => {
            Ok(ParsedResolver::new(
                Box::new(RandomConditionNode) as Box<dyn Node<bool>>,
                "Condition".to_string()
            ))
        }
        StructuredTokenInput::CharacterHP { character } => {
            let character_node = convert_structured_to_node(character)?;
            let character_target_node = character_node.require_character()?;
            Ok(ParsedResolver::new(
                Box::new(CharacterHpNode::new(character_target_node)) as Box<dyn Node<i32>>,
                "Value".to_string()
            ))
        }
        StructuredTokenInput::Eq { left, right } => {
            // Try TeamSide comparison
            if let (Ok(left_team), Ok(right_team)) = (
                convert_structured_to_node(left)?.require_team_side(),
                convert_structured_to_node(right)?.require_team_side()
            ) {
                Ok(ParsedResolver::new(
                    Box::new(EqConditionNode::new(left_team, right_team)) as Box<dyn Node<bool>>,
                    "Condition".to_string()
                ))
            }
            // Try Value comparison
            else if let (Ok(left_value), Ok(right_value)) = (
                convert_structured_to_node(left)?.require_value(),
                convert_structured_to_node(right)?.require_value()
            ) {
                Ok(ParsedResolver::new(
                    Box::new(EqConditionNode::new(left_value, right_value)) as Box<dyn Node<bool>>,
                    "Condition".to_string()
                ))
            }
            // Try Character comparison
            else if let (Ok(left_character), Ok(right_character)) = (
                convert_structured_to_node(left)?.require_character(),
                convert_structured_to_node(right)?.require_character()
            ) {
                Ok(ParsedResolver::new(
                    Box::new(EqConditionNode::new(left_character, right_character)) as Box<dyn Node<bool>>,
                    "Condition".to_string()
                ))
            }
            else {
                Err(format!("Cannot compare different types in Eq"))
            }
        }
        StructuredTokenInput::CharacterTeam { character } => {
            let character_node = convert_structured_to_node(character)?;
            let character_target_node = character_node.require_character()?;
            Ok(ParsedResolver::new(
                Box::new(CharacterTeamNode::new(character_target_node)) as Box<dyn Node<TeamSide>>,
                "TeamSide".to_string()
            ))
        }
        StructuredTokenInput::FilterList { array, condition } => {
            let array_node = convert_structured_to_node(array)?;
            let condition_node = convert_structured_to_node(condition)?;
            let character_array_node = array_node.require_character_array()?;
            let condition_bool_node = condition_node.require_condition()?;
            Ok(ParsedResolver::new(
                Box::new(FilterListNode::new(character_array_node, condition_bool_node)) as Box<dyn Node<Vec<Character>>>,
                "CharacterArray".to_string()
            ))
        }
        StructuredTokenInput::Map { array, transform } => {
            convert_map_token(array, transform)
        }
        StructuredTokenInput::Element => {
            Ok(ParsedResolver::new(
                Box::new(ElementNode::new()) as Box<dyn Node<Character>>,
                "Character".to_string()
            ))
        }
        StructuredTokenInput::Enemy => {
            Ok(ParsedResolver::new(
                Box::new(EnemyNode::new()) as Box<dyn Node<TeamSide>>,
                "TeamSide".to_string()
            ))
        }
        StructuredTokenInput::Hero => {
            Ok(ParsedResolver::new(
                Box::new(HeroNode::new()) as Box<dyn Node<TeamSide>>,
                "TeamSide".to_string()
            ))
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