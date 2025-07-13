// StructuredTokenInput → Node 変換

use crate::{StructuredTokenInput, RuleSet};
use action_system::{RuleNode, ConditionCheckNode, ConstantValueNode, ActingCharacterNode, RandomConditionNode, GreaterThanConditionNode, StrikeActionNode, HealActionNode, AllCharactersNode, Character, Node, Action, FilterListNode, CharacterTeamNode, ElementNode, EnemyNode, HeroNode, TeamSide, AllTeamSidesNode, GreaterThanNode, CharacterHpVsValueGreaterThanNode, ValueVsCharacterHpGreaterThanNode};
use action_system::nodes::array::MappingNode;
use action_system::nodes::condition::EqConditionNode;
use std::any::Any;

// No more type aliases needed - MappingNode is now generic and auto-implemented

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
    
    pub fn require_team_side_array(self) -> Result<Box<dyn Node<Vec<TeamSide>>>, String> {
        match self.node.downcast::<Box<dyn Node<Vec<TeamSide>>>>() {
            Ok(team_side_array_node) => Ok(*team_side_array_node),
            Err(_) => Err(format!("Expected TeamSideArray, got {}", self.type_name)),
        }
    }
    
    pub fn require_character_hp(self) -> Result<Box<dyn Node<action_system::CharacterHP>>, String> {
        match self.node.downcast::<Box<dyn Node<action_system::CharacterHP>>>() {
            Ok(character_hp_node) => Ok(*character_hp_node),
            Err(_) => Err(format!("Expected CharacterHP, got {}", self.type_name)),
        }
    }
    
    pub fn require_character_hp_array(self) -> Result<Box<dyn Node<Vec<action_system::CharacterHP>>>, String> {
        match self.node.downcast::<Box<dyn Node<Vec<action_system::CharacterHP>>>>() {
            Ok(character_hp_array_node) => Ok(*character_hp_array_node),
            Err(_) => Err(format!("Expected CharacterHPArray, got {}", self.type_name)),
        }
    }
}

// Generic type conversion macro - automatically handles all possible type combinations
macro_rules! try_all_type_combinations {
    (
        $token:expr;
        $(($variant:ident { $($field:ident),* }, $parser:expr, $result_type:ty, $type_name:expr)),*
        ; $(($simple_variant:ident, $simple_parser:expr, $simple_result_type:ty, $simple_type_name:expr)),*
    ) => {
        match $token {
            $(
                StructuredTokenInput::$variant { $($field),* } => {
                    Some(Ok(ParsedResolver::new(
                        Box::new($parser) as Box<dyn Node<$result_type>>,
                        $type_name.to_string()
                    )))
                }
            )*
            $(
                StructuredTokenInput::$simple_variant => {
                    Some(Ok(ParsedResolver::new(
                        Box::new($simple_parser) as Box<dyn Node<$simple_result_type>>,
                        $simple_type_name.to_string()
                    )))
                }
            )*
            _ => None
        }
    };
}

// Mapping-specific macro for all array → transform combinations
macro_rules! try_all_mapping_combinations {
    (
        $array:expr, $transform:expr;
        $(($array_method:ident, $transform_method:ident, $mapping_node:ident, $result_type:ty, $type_name:expr)),*
    ) => {
        $({
            let array_result = convert_structured_to_node($array)?;
            let transform_result = convert_structured_to_node($transform)?;
            
            if let (Ok(array_node), Ok(transform_node)) = (
                array_result.$array_method(),
                transform_result.$transform_method()
            ) {
                return Ok(ParsedResolver::new(
                    Box::new($mapping_node::new(array_node, transform_node)) as Box<dyn Node<$result_type>>,
                    $type_name.to_string()
                ));
            }
        })*
    };
}

// Generic array operation macro - tries all possible array types for operations like Max, Min
macro_rules! try_all_array_operations {
    (
        $array:expr, $operation:ident;
        $(($array_method:ident, $result_type:ty, $type_name:expr)),*
    ) => {
        {
            $(
                if let Ok(array_node) = convert_structured_to_node($array)?.$array_method() {
                    return Ok(ParsedResolver::new(
                        Box::new(action_system::$operation::<$result_type>::new(array_node)) as Box<dyn Node<$result_type>>,
                        $type_name.to_string()
                    ));
                }
            )*
            Err(format!("Cannot convert {}: unsupported array type", stringify!($operation)))
        }
    };
}

// Comparison operation macro - tries all possible left/right type combinations  
macro_rules! try_all_comparison_combinations {
    (
        $left:expr, $right:expr;
        $(($left_method:ident, $right_method:ident, $node_type:ident $(< $($type_param:ty),* >)?)),*
    ) => {
        {
            $(
                if let (Ok(left_node), Ok(right_node)) = (
                    convert_structured_to_node($left)?.$left_method(),
                    convert_structured_to_node($right)?.$right_method()
                ) {
                    return Ok(ParsedResolver::new(
                        Box::new($node_type$(::< $($type_param),* >)?::new(left_node, right_node)) as Box<dyn Node<bool>>,
                        "Condition".to_string()
                    ));
                }
            )*
            Err("Cannot convert comparison: unsupported operand types".to_string())
        }
    };
}

// Helper functions for complex token conversions
fn convert_greater_than_token(left: &StructuredTokenInput, right: &StructuredTokenInput) -> Result<ParsedResolver, String> {
    try_all_comparison_combinations!(
        left, right;
        (require_value, require_value, GreaterThanConditionNode),
        (require_character_hp, require_value, CharacterHpVsValueGreaterThanNode),
        (require_value, require_character_hp, ValueVsCharacterHpGreaterThanNode),
        (require_character_hp, require_character_hp, GreaterThanNode<action_system::CharacterHP>)
    )
}

fn convert_character_to_hp_token(character: &StructuredTokenInput) -> Result<ParsedResolver, String> {
    let character_node = convert_structured_to_node(character)?;
    let character_target_node = character_node.require_character()?;
    Ok(ParsedResolver::new(
        Box::new(action_system::CharacterToHpNode::new(character_target_node)) as Box<dyn Node<action_system::CharacterHP>>,
        "CharacterHP".to_string()
    ))
}

fn convert_character_hp_to_character_token(character_hp: &StructuredTokenInput) -> Result<ParsedResolver, String> {
    let character_hp_node = convert_structured_to_node(character_hp)?;
    let character_hp_target_node = character_hp_node.require_character_hp()?;
    Ok(ParsedResolver::new(
        Box::new(action_system::CharacterHpToCharacterNode::new(character_hp_target_node)) as Box<dyn Node<Character>>,
        "Character".to_string()
    ))
}

fn convert_team_members_token(team_side: &StructuredTokenInput) -> Result<ParsedResolver, String> {
    let team_side_node = convert_structured_to_node(team_side)?;
    let team_side_target_node = team_side_node.require_team_side()?;
    Ok(ParsedResolver::new(
        Box::new(action_system::TeamMembersNode::new_with_node(team_side_target_node)) as Box<dyn Node<Vec<Character>>>,
        "CharacterArray".to_string()
    ))
}

fn convert_random_pick_token(array: &StructuredTokenInput) -> Result<ParsedResolver, String> {
    let array_node = convert_structured_to_node(array)?;
    let character_array_node = array_node.require_character_array()?;
    Ok(ParsedResolver::new(
        Box::new(action_system::CharacterRandomPickNode::new(character_array_node)) as Box<dyn Node<Character>>,
        "Character".to_string()
    ))
}

fn convert_eq_token(left: &StructuredTokenInput, right: &StructuredTokenInput) -> Result<ParsedResolver, String> {
    try_all_comparison_combinations!(
        left, right;
        (require_team_side, require_team_side, EqConditionNode),
        (require_value, require_value, EqConditionNode),
        (require_character, require_character, EqConditionNode)
    )
}

fn convert_character_team_token(character: &StructuredTokenInput) -> Result<ParsedResolver, String> {
    let character_node = convert_structured_to_node(character)?;
    let character_target_node = character_node.require_character()?;
    Ok(ParsedResolver::new(
        Box::new(CharacterTeamNode::new(character_target_node)) as Box<dyn Node<TeamSide>>,
        "TeamSide".to_string()
    ))
}

fn convert_filter_list_token(array: &StructuredTokenInput, condition: &StructuredTokenInput) -> Result<ParsedResolver, String> {
    let array_node = convert_structured_to_node(array)?;
    let condition_node = convert_structured_to_node(condition)?;
    let character_array_node = array_node.require_character_array()?;
    let condition_bool_node = condition_node.require_condition()?;
    Ok(ParsedResolver::new(
        Box::new(FilterListNode::new(character_array_node, condition_bool_node)) as Box<dyn Node<Vec<Character>>>,
        "CharacterArray".to_string()
    ))
}

fn convert_max_token(array: &StructuredTokenInput) -> Result<ParsedResolver, String> {
    try_all_array_operations!(
        array, MaxNode;
        (require_value_array, i32, "Value"),
        (require_character_hp_array, action_system::CharacterHP, "CharacterHP")
    )
}

fn convert_numeric_max_token(array: &StructuredTokenInput) -> Result<ParsedResolver, String> {
    try_all_array_operations!(
        array, MaxNode;
        (require_value_array, i32, "Value"),
        (require_character_hp_array, action_system::CharacterHP, "CharacterHP")
    )
}

fn convert_min_token(array: &StructuredTokenInput) -> Result<ParsedResolver, String> {
    try_all_array_operations!(
        array, MinNode;
        (require_value_array, i32, "Value"),
        (require_character_hp_array, action_system::CharacterHP, "CharacterHP")
    )
}

fn convert_numeric_min_token(array: &StructuredTokenInput) -> Result<ParsedResolver, String> {
    try_all_array_operations!(
        array, MinNode;
        (require_value_array, i32, "Value"),
        (require_character_hp_array, action_system::CharacterHP, "CharacterHP")
    )
}

fn convert_map_token(array: &StructuredTokenInput, transform: &StructuredTokenInput) -> Result<ParsedResolver, String> {
    // All mapping combinations are automatically tried using the generic MappingNode
    // Thanks to the macro implementation in action-system, all type combinations are supported
    try_all_mapping_combinations!(
        array, transform;
        (require_character_array, require_character, MappingNode, Vec<Character>, "CharacterArray"),
        (require_character_array, require_value, MappingNode, Vec<i32>, "ValueArray"),
        (require_character_array, require_character_hp, MappingNode, Vec<action_system::CharacterHP>, "CharacterHPArray"),
        (require_character_hp_array, require_character, MappingNode, Vec<Character>, "CharacterArray"),
        (require_value_array, require_value, MappingNode, Vec<i32>, "ValueArray"),
        (require_value_array, require_character, MappingNode, Vec<Character>, "CharacterArray")
        // NEW TYPES: Just add new combinations here. The impl will be auto-generated in action-system!
        // Example: (require_team_side_array, require_team_side, MappingNode, Vec<TeamSide>, "TeamSideArray")
    );
    
    Err(format!("Cannot determine mapping type for Map - no compatible array→transform combination found"))
}

// StructuredTokenInput → Node 変換
pub fn convert_structured_to_node(token: &StructuredTokenInput) -> Result<ParsedResolver, String> {
    // Try structured patterns first
    if let Some(result) = try_all_type_combinations!(
        token;
        (Strike { target }, {
            let target_node = convert_structured_to_node(target)?;
            let character_node = target_node.require_character()?;
            StrikeActionNode::new(character_node)
        }, Box<dyn Action>, "Action"),
        (Heal { target }, {
            let target_node = convert_structured_to_node(target)?;
            let character_node = target_node.require_character()?;
            HealActionNode::new(character_node)
        }, Box<dyn Action>, "Action"),
        (Check { condition, then_action }, {
            let condition_node = convert_structured_to_node(condition)?;
            let action_node = convert_structured_to_node(then_action)?;
            let cond = condition_node.require_condition()?;
            let action = action_node.require_action()?;
            ConditionCheckNode::new(cond, action)
        }, Box<dyn Action>, "Action")
        ;
        (ActingCharacter, ActingCharacterNode, Character, "Character"),
        (AllCharacters, AllCharactersNode::new(), Vec<Character>, "CharacterArray"),
        (TrueOrFalseRandom, RandomConditionNode, bool, "Condition"),
        (Enemy, EnemyNode::new(), TeamSide, "TeamSide"),
        (Hero, HeroNode::new(), TeamSide, "TeamSide"),
        (Element, ElementNode::new(), Character, "Character"),
        (AllTeamSides, AllTeamSidesNode::new(), Vec<TeamSide>, "TeamSideArray")
    ) {
        return result;
    }
    
    // Handle complex patterns that don't fit the macro
    match token {
        StructuredTokenInput::GreaterThan { left, right } => {
            convert_greater_than_token(left, right)
        }
        StructuredTokenInput::CharacterToHp { character } => {
            convert_character_to_hp_token(character)
        }
        StructuredTokenInput::CharacterHpToCharacter { character_hp } => {
            convert_character_hp_to_character_token(character_hp)
        }
        StructuredTokenInput::Number { value } => {
            Ok(ParsedResolver::new(
                Box::new(ConstantValueNode::new(*value)) as Box<dyn Node<i32>>,
                "Value".to_string()
            ))
        }
        StructuredTokenInput::TeamMembers { team_side } => {
            convert_team_members_token(team_side)
        }
        StructuredTokenInput::RandomPick { array } => {
            convert_random_pick_token(array)
        }
        StructuredTokenInput::Eq { left, right } => {
            convert_eq_token(left, right)
        }
        StructuredTokenInput::CharacterTeam { character } => {
            convert_character_team_token(character)
        }
        StructuredTokenInput::FilterList { array, condition } => {
            convert_filter_list_token(array, condition)
        }
        StructuredTokenInput::Map { array, transform } => {
            convert_map_token(array, transform)
        }
        StructuredTokenInput::Max { array } => {
            convert_max_token(array)
        }
        StructuredTokenInput::NumericMax { array } => {
            convert_numeric_max_token(array)
        }
        StructuredTokenInput::Min { array } => {
            convert_min_token(array)
        }
        StructuredTokenInput::NumericMin { array } => {
            convert_numeric_min_token(array)
        }
        _ => Err(format!("Unknown token type: {:?}", token))
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
    fn test_character_hp_value_node_conversion() {
        let structured = StructuredTokenInput::CharacterToHp { 
            character: Box::new(StructuredTokenInput::ActingCharacter) 
        };
        let result = convert_structured_to_node(&structured).unwrap();
        assert!(result.require_character_hp().is_ok());
    }

    #[test]
    fn test_hp_character_node_conversion() {
        let structured = StructuredTokenInput::CharacterHpToCharacter { 
            character_hp: Box::new(StructuredTokenInput::CharacterToHp { 
                character: Box::new(StructuredTokenInput::ActingCharacter) 
            }) 
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