// Converter - FlatTokenInput <-> StructuredTokenInput <-> Node 変換

use crate::{FlatTokenInput, StructuredTokenInput, RuleSet};
use action_system::{RuleNode, ConditionCheckNode, ConstantValueNode, ActingCharacterNode, CharacterHpNode, RandomConditionNode, GreaterThanConditionNode, StrikeActionNode, HealActionNode, AllCharactersNode, Character, Node, Action, FilterListNode, TeamSideEqNode, CharacterTeamNode, ElementCharacterNode, EnemyNode, HeroNode, TeamSide};

// パース結果を表すEnum
#[derive(Debug)]
pub enum ParsedResolver {
    Action(Box<dyn Node<Box<dyn Action>>>),
    Condition(Box<dyn Node<bool>>),
    Value(Box<dyn Node<i32>>),
    Character(Box<dyn Node<i32>>),
    CharacterArray(Box<dyn Node<Vec<Character>>>),
    TeamSide(Box<dyn Node<TeamSide>>),
    ActualCharacter(Box<dyn Node<Character>>),
}

impl ParsedResolver {
    pub fn require_action(self) -> Result<Box<dyn Node<Box<dyn Action>>>, String> {
        match self {
            ParsedResolver::Action(action) => Ok(action),
            _ => Err(format!("Expected Action, got {:?}", self)),
        }
    }
    
    pub fn require_condition(self) -> Result<Box<dyn Node<bool>>, String> {
        match self {
            ParsedResolver::Condition(condition) => Ok(condition),
            _ => Err(format!("Expected Condition, got {:?}", self)),
        }
    }
    
    pub fn require_value(self) -> Result<Box<dyn Node<i32>>, String> {
        match self {
            ParsedResolver::Value(value) => Ok(value),
            _ => Err(format!("Expected Value, got {:?}", self)),
        }
    }
    
    pub fn require_character(self) -> Result<Box<dyn Node<i32>>, String> {
        match self {
            ParsedResolver::Character(character_node) => Ok(character_node),
            _ => Err(format!("Expected Character, got {:?}", self)),
        }
    }
    
    pub fn require_character_array(self) -> Result<Box<dyn Node<Vec<Character>>>, String> {
        match self {
            ParsedResolver::CharacterArray(character_array_node) => Ok(character_array_node),
            _ => Err(format!("Expected CharacterArray, got {:?}", self)),
        }
    }
    
    pub fn require_team_side(self) -> Result<Box<dyn Node<TeamSide>>, String> {
        match self {
            ParsedResolver::TeamSide(team_side_node) => Ok(team_side_node),
            _ => Err(format!("Expected TeamSide, got {:?}", self)),
        }
    }
    
    pub fn require_actual_character(self) -> Result<Box<dyn Node<Character>>, String> {
        match self {
            ParsedResolver::ActualCharacter(character_node) => Ok(character_node),
            _ => Err(format!("Expected ActualCharacter, got {:?}", self)),
        }
    }
}

// FlatTokenInput → StructuredTokenInput 変換
pub fn convert_flat_to_structured(flat_tokens: &[FlatTokenInput]) -> Result<Vec<StructuredTokenInput>, String> {
    let mut result = Vec::new();
    let mut index = 0;
    
    while index < flat_tokens.len() {
        let (token, consumed) = parse_flat_token(flat_tokens, index)?;
        result.push(token);
        index += consumed;
    }
    
    Ok(result)
}

fn parse_flat_token(tokens: &[FlatTokenInput], index: usize) -> Result<(StructuredTokenInput, usize), String> {
    if index >= tokens.len() {
        return Err("No tokens to parse".to_string());
    }
    
    match &tokens[index] {
        FlatTokenInput::Strike => {
            if index + 1 >= tokens.len() {
                return Err("Strike requires a target".to_string());
            }
            let (target, consumed) = parse_flat_token(tokens, index + 1)?;
            Ok((StructuredTokenInput::Strike { target: Box::new(target) }, 1 + consumed))
        }
        FlatTokenInput::Heal => {
            if index + 1 >= tokens.len() {
                return Err("Heal requires a target".to_string());
            }
            let (target, consumed) = parse_flat_token(tokens, index + 1)?;
            Ok((StructuredTokenInput::Heal { target: Box::new(target) }, 1 + consumed))
        }
        FlatTokenInput::Check => {
            if index + 2 >= tokens.len() {
                return Err("Check requires a condition and action".to_string());
            }
            let (condition, cond_consumed) = parse_flat_token(tokens, index + 1)?;
            let (action, action_consumed) = parse_flat_token(tokens, index + 1 + cond_consumed)?;
            Ok((StructuredTokenInput::Check { 
                condition: Box::new(condition), 
                then_action: Box::new(action) 
            }, 1 + cond_consumed + action_consumed))
        }
        FlatTokenInput::GreaterThan => {
            if index + 2 >= tokens.len() {
                return Err("GreaterThan requires two operands".to_string());
            }
            let (left, left_consumed) = parse_flat_token(tokens, index + 1)?;
            let (right, right_consumed) = parse_flat_token(tokens, index + 1 + left_consumed)?;
            Ok((StructuredTokenInput::GreaterThan { 
                left: Box::new(left), 
                right: Box::new(right) 
            }, 1 + left_consumed + right_consumed))
        }
        FlatTokenInput::HP => {
            if index + 1 >= tokens.len() {
                return Err("HP requires a character".to_string());
            }
            let (character, consumed) = parse_flat_token(tokens, index + 1)?;
            Ok((StructuredTokenInput::HP { character: Box::new(character) }, 1 + consumed))
        }
        FlatTokenInput::Number(n) => Ok((StructuredTokenInput::Number { value: *n as i32 }, 1)),
        FlatTokenInput::ActingCharacter => Ok((StructuredTokenInput::ActingCharacter, 1)),
        FlatTokenInput::AllCharacters => Ok((StructuredTokenInput::AllCharacters, 1)),
        FlatTokenInput::RandomPick => {
            if index + 1 >= tokens.len() {
                return Err("RandomPick requires an array argument".to_string());
            }
            let (array_token, array_consumed) = parse_flat_token(tokens, index + 1)?;
            Ok((StructuredTokenInput::RandomPick { array: Box::new(array_token) }, 1 + array_consumed))
        }
        FlatTokenInput::TrueOrFalse => Ok((StructuredTokenInput::TrueOrFalseRandom, 1)),
        FlatTokenInput::FilterList => {
            if index + 2 >= tokens.len() {
                return Err("FilterList requires an array and a condition".to_string());
            }
            let (array, array_consumed) = parse_flat_token(tokens, index + 1)?;
            let (condition, condition_consumed) = parse_flat_token(tokens, index + 1 + array_consumed)?;
            Ok((StructuredTokenInput::FilterList {
                array: Box::new(array),
                condition: Box::new(condition),
            }, 1 + array_consumed + condition_consumed))
        }
        FlatTokenInput::Eq => {
            if index + 2 >= tokens.len() {
                return Err("Eq requires two operands".to_string());
            }
            let (left, left_consumed) = parse_flat_token(tokens, index + 1)?;
            let (right, right_consumed) = parse_flat_token(tokens, index + 1 + left_consumed)?;
            Ok((StructuredTokenInput::Eq {
                left: Box::new(left),
                right: Box::new(right),
            }, 1 + left_consumed + right_consumed))
        }
        FlatTokenInput::CharacterTeam => {
            if index + 1 >= tokens.len() {
                return Err("CharacterTeam requires a character".to_string());
            }
            let (character, consumed) = parse_flat_token(tokens, index + 1)?;
            Ok((StructuredTokenInput::CharacterTeam {
                character: Box::new(character),
            }, 1 + consumed))
        }
        FlatTokenInput::Element => Ok((StructuredTokenInput::Element, 1)),
        FlatTokenInput::Enemy => Ok((StructuredTokenInput::Enemy, 1)),
        FlatTokenInput::Hero => Ok((StructuredTokenInput::Hero, 1)),
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
            let char_node = character_node.require_character()?;
            Ok(ParsedResolver::Value(Box::new(CharacterHpNode::new(char_node))))
        }
        StructuredTokenInput::Number { value } => {
            Ok(ParsedResolver::Value(Box::new(ConstantValueNode::new(*value))))
        }
        StructuredTokenInput::ActingCharacter => {
            Ok(ParsedResolver::Character(Box::new(ActingCharacterNode)))
        }
        StructuredTokenInput::AllCharacters => {
            Ok(ParsedResolver::CharacterArray(Box::new(AllCharactersNode::new())))
        }
        StructuredTokenInput::RandomPick { array } => {
            let array_node = convert_structured_to_node(array)?;
            let character_array_node = array_node.require_character_array()?;
            // For backward compatibility, return Character ID using CharacterRandomPickNode
            Ok(ParsedResolver::Character(Box::new(action_system::CharacterRandomPickNode::from_character_array(character_array_node))))
        }
        StructuredTokenInput::TrueOrFalseRandom => {
            Ok(ParsedResolver::Condition(Box::new(RandomConditionNode)))
        }
        StructuredTokenInput::CharacterHP => {
            // Legacy support - assume acting character
            Ok(ParsedResolver::Value(Box::new(CharacterHpNode::new(Box::new(ActingCharacterNode)))))
        }
        StructuredTokenInput::FilterList { array, condition } => {
            let array_node = convert_structured_to_node(array)?;
            let condition_node = convert_structured_to_node(condition)?;
            let character_array_node = array_node.require_character_array()?;
            let condition_bool_node = condition_node.require_condition()?;
            Ok(ParsedResolver::CharacterArray(Box::new(FilterListNode::new(character_array_node, condition_bool_node))))
        }
        StructuredTokenInput::Eq { left, right } => {
            let left_node = convert_structured_to_node(left)?;
            let right_node = convert_structured_to_node(right)?;
            let left_team = left_node.require_team_side()?;
            let right_team = right_node.require_team_side()?;
            Ok(ParsedResolver::Condition(Box::new(TeamSideEqNode::new(left_team, right_team))))
        }
        StructuredTokenInput::CharacterTeam { character } => {
            let character_node = convert_structured_to_node(character)?;
            let actual_character_node = character_node.require_actual_character()?;
            Ok(ParsedResolver::TeamSide(Box::new(CharacterTeamNode::new(actual_character_node))))
        }
        StructuredTokenInput::Element => {
            Ok(ParsedResolver::ActualCharacter(Box::new(ElementCharacterNode::new())))
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

// Vec<Vec<FlatTokenInput>> → Vec<RuleNode> 変換（UI入力経路）
pub fn convert_flat_rules_to_nodes(flat_rules: &[Vec<FlatTokenInput>]) -> Vec<RuleNode> {
    flat_rules
        .iter()
        .filter(|rule_row| !rule_row.is_empty())
        .filter_map(|rule_row| {
            // FlatTokenInput → StructuredTokenInput → Node
            match convert_flat_to_structured(rule_row) {
                Ok(structured_tokens) => {
                    if structured_tokens.is_empty() {
                        return None;
                    }
                    
                    match convert_structured_to_node(&structured_tokens[0]) {
                        Ok(parsed) => {
                            match parsed.require_action() {
                                Ok(action) => Some(action),
                                Err(_) => None,
                            }
                        }
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
    fn test_flat_to_structured_simple() {
        let flat = vec![FlatTokenInput::Strike, FlatTokenInput::ActingCharacter];
        let structured = convert_flat_to_structured(&flat).unwrap();
        
        assert_eq!(structured.len(), 1);
        match &structured[0] {
            StructuredTokenInput::Strike { target } => {
                match target.as_ref() {
                    StructuredTokenInput::ActingCharacter => (),
                    _ => panic!("Expected ActingCharacter target"),
                }
            }
            _ => panic!("Expected Strike"),
        }
    }

    #[test]
    fn test_structured_to_node() {
        let structured = StructuredTokenInput::Strike { 
            target: Box::new(StructuredTokenInput::ActingCharacter) 
        };
        
        let result = convert_structured_to_node(&structured).unwrap();
        assert!(result.require_action().is_ok());
    }

    #[test]
    fn test_flat_rules_to_nodes() {
        let flat_rules = vec![
            vec![FlatTokenInput::Strike, FlatTokenInput::ActingCharacter],
            vec![FlatTokenInput::Heal, FlatTokenInput::RandomPick, FlatTokenInput::AllCharacters],
        ];
        
        let nodes = convert_flat_rules_to_nodes(&flat_rules);
        assert_eq!(nodes.len(), 2);
    }

    #[test]
    fn test_all_characters_token() {
        // Test FlatTokenInput::AllCharacters conversion
        let flat = vec![FlatTokenInput::AllCharacters];
        let structured = convert_flat_to_structured(&flat).unwrap();
        
        assert_eq!(structured.len(), 1);
        match &structured[0] {
            StructuredTokenInput::AllCharacters => (),
            _ => panic!("Expected AllCharacters"),
        }
        
        // Test structured to node conversion
        let result = convert_structured_to_node(&structured[0]).unwrap();
        assert!(result.require_character_array().is_ok());
    }

    #[test]
    fn test_random_pick_token() {
        // Test FlatTokenInput::RandomPick with AllCharacters
        let flat = vec![FlatTokenInput::RandomPick, FlatTokenInput::AllCharacters];
        let structured = convert_flat_to_structured(&flat).unwrap();
        
        assert_eq!(structured.len(), 1);
        match &structured[0] {
            StructuredTokenInput::RandomPick { array } => {
                match array.as_ref() {
                    StructuredTokenInput::AllCharacters => (),
                    _ => panic!("Expected AllCharacters array"),
                }
            }
            _ => panic!("Expected RandomPick"),
        }
        
        // Test structured to node conversion
        let result = convert_structured_to_node(&structured[0]).unwrap();
        assert!(result.require_character().is_ok());
    }

    #[test]
    fn test_new_tokens_flat_to_structured() {
        // Test new tokens: FilterList, Eq, CharacterTeam, Element, Enemy, Hero
        let flat = vec![
            FlatTokenInput::FilterList, FlatTokenInput::AllCharacters, FlatTokenInput::Eq, 
            FlatTokenInput::CharacterTeam, FlatTokenInput::Element, FlatTokenInput::Enemy
        ];
        let structured = convert_flat_to_structured(&flat).unwrap();
        
        assert_eq!(structured.len(), 1);
        match &structured[0] {
            StructuredTokenInput::FilterList { array, condition } => {
                match array.as_ref() {
                    StructuredTokenInput::AllCharacters => (),
                    _ => panic!("Expected AllCharacters array"),
                }
                match condition.as_ref() {
                    StructuredTokenInput::Eq { left, right } => {
                        match left.as_ref() {
                            StructuredTokenInput::CharacterTeam { character } => {
                                match character.as_ref() {
                                    StructuredTokenInput::Element => (),
                                    _ => panic!("Expected Element character"),
                                }
                            }
                            _ => panic!("Expected CharacterTeam left"),
                        }
                        match right.as_ref() {
                            StructuredTokenInput::Enemy => (),
                            _ => panic!("Expected Enemy right"),
                        }
                    }
                    _ => panic!("Expected Eq condition"),
                }
            }
            _ => panic!("Expected FilterList"),
        }
    }

    #[test]
    fn test_enemy_hero_tokens() {
        // Test Enemy token
        let flat_enemy = vec![FlatTokenInput::Enemy];
        let structured_enemy = convert_flat_to_structured(&flat_enemy).unwrap();
        assert_eq!(structured_enemy.len(), 1);
        match &structured_enemy[0] {
            StructuredTokenInput::Enemy => (),
            _ => panic!("Expected Enemy"),
        }
        
        let result_enemy = convert_structured_to_node(&structured_enemy[0]).unwrap();
        assert!(result_enemy.require_team_side().is_ok());
        
        // Test Hero token
        let flat_hero = vec![FlatTokenInput::Hero];
        let structured_hero = convert_flat_to_structured(&flat_hero).unwrap();
        assert_eq!(structured_hero.len(), 1);
        match &structured_hero[0] {
            StructuredTokenInput::Hero => (),
            _ => panic!("Expected Hero"),
        }
        
        let result_hero = convert_structured_to_node(&structured_hero[0]).unwrap();
        assert!(result_hero.require_team_side().is_ok());
    }
}