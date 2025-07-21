// Evaluation context - manages the context for node evaluation including current element being processed
use crate::Character;
use crate::core::character_hp::CharacterHP;
use crate::nodes::character::BattleContext;

/// Represents different types of values that can be used as current elements
#[derive(Debug, Clone)]
pub enum CurrentElement {
    /// A character value
    Character(Character),
    /// A numeric value
    Value(i32),
    /// A team side
    TeamSide(crate::TeamSide),
    /// A character HP value
    CharacterHP(CharacterHP),
}

/// Context for evaluating nodes, includes battle context, current element, and RNG
pub struct EvaluationContext<'a> {
    /// The battle context containing teams and acting character
    pub battle_context: &'a BattleContext<'a>,
    /// The current element being processed (used by Element node in array operations)
    pub current_element: Option<CurrentElement>,
    /// Random number generator for node evaluation
    pub rng: &'a mut dyn rand::RngCore,
}

impl<'a> EvaluationContext<'a> {
    /// Creates a new EvaluationContext with battle context only
    pub fn new(battle_context: &'a BattleContext<'a>, rng: &'a mut dyn rand::RngCore) -> Self {
        Self {
            battle_context,
            current_element: None,
            rng,
        }
    }
    
    /// Creates a new EvaluationContext with both battle context and current element
    pub fn with_element(battle_context: &'a BattleContext<'a>, current_element: &'a Character, rng: &'a mut dyn rand::RngCore) -> Self {
        Self {
            battle_context,
            current_element: Some(CurrentElement::Character(current_element.clone())),
            rng,
        }
    }
    
    /// Creates a new EvaluationContext with a character element
    pub fn with_character_element(battle_context: &'a BattleContext<'a>, character: Character, rng: &'a mut dyn rand::RngCore) -> Self {
        Self {
            battle_context,
            current_element: Some(CurrentElement::Character(character)),
            rng,
        }
    }
    
    /// Creates a new EvaluationContext with a value element
    pub fn with_value_element(battle_context: &'a BattleContext<'a>, value: i32, rng: &'a mut dyn rand::RngCore) -> Self {
        Self {
            battle_context,
            current_element: Some(CurrentElement::Value(value)),
            rng,
        }
    }
    
    /// Creates a new EvaluationContext with a team side element
    pub fn with_team_side_element(battle_context: &'a BattleContext<'a>, team_side: crate::TeamSide, rng: &'a mut dyn rand::RngCore) -> Self {
        Self {
            battle_context,
            current_element: Some(CurrentElement::TeamSide(team_side)),
            rng,
        }
    }
    
    
    /// Gets the battle context
    pub fn get_battle_context(&self) -> &'a BattleContext<'a> {
        self.battle_context
    }
    
    /// Creates a new EvaluationContext with a different current element
    /// This method takes ownership of the RNG to avoid borrowing issues
    pub fn with_element_from_context(&mut self, element: &Character) -> EvaluationContext<'_> {
        EvaluationContext {
            battle_context: self.battle_context,
            current_element: Some(CurrentElement::Character(element.clone())),
            rng: &mut *self.rng,
        }
    }
    
    /// Creates a new EvaluationContext with a different current element (any type)
    /// This method takes ownership of the RNG to avoid borrowing issues
    pub fn with_current_element_from_context(&mut self, element: CurrentElement) -> EvaluationContext<'_> {
        EvaluationContext {
            battle_context: self.battle_context,
            current_element: Some(element),
            rng: &mut *self.rng,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Team, TeamSide};

    
    #[test]
    fn test_evaluation_context_with_new_element() {
        use rand::SeedableRng;
        let mut rng1 = rand::rngs::StdRng::seed_from_u64(12345);
        let mut rng2 = rand::rngs::StdRng::seed_from_u64(12345);
        
        let acting_character = Character::new(1, "Acting".to_string(), 100, 100, 20);
        let element1 = Character::new(2, "Element1".to_string(), 80, 100, 15);
        let element2 = Character::new(3, "Element2".to_string(), 60, 100, 12);
        let team = Team::new("Test Team".to_string(), vec![acting_character.clone(), element1.clone(), element2.clone()]);
        let battle_context = BattleContext::new(&acting_character, TeamSide::Player, &team, &team);
        
        let mut eval_context1 = EvaluationContext::with_element(&battle_context, &element1, &mut rng1);
        
        // Check the original context first
        if let Some(CurrentElement::Character(character)) = &eval_context1.current_element {
            assert_eq!(character.id, 2);
        } else {
            panic!("Expected Character element");
        }
        
        // Now create the new context with a different element
        let eval_context2 = eval_context1.with_element_from_context(&element2);
        
        if let Some(CurrentElement::Character(character)) = &eval_context2.current_element {
            assert_eq!(character.id, 3);
        } else {
            panic!("Expected Character element");
        }
    }
    
    #[test]
    fn test_evaluation_context_with_different_element_types() {
        use rand::SeedableRng;
        let mut rng1 = rand::rngs::StdRng::seed_from_u64(12345);
        let mut rng2 = rand::rngs::StdRng::seed_from_u64(12345);
        let mut rng3 = rand::rngs::StdRng::seed_from_u64(12345);
        
        let acting_character = Character::new(1, "Acting".to_string(), 100, 100, 20);
        let team = Team::new("Test Team".to_string(), vec![acting_character.clone()]);
        let battle_context = BattleContext::new(&acting_character, TeamSide::Player, &team, &team);
        
        // Test with character element
        let character_element = Character::new(2, "Element".to_string(), 80, 100, 15);
        let eval_context_char = EvaluationContext::with_character_element(&battle_context, character_element, &mut rng1);
        if let Some(CurrentElement::Character(character)) = &eval_context_char.current_element {
            assert_eq!(character.id, 2);
        } else {
            panic!("Expected Character element");
        }
        
        // Test with value element
        let eval_context_value = EvaluationContext::with_value_element(&battle_context, 42, &mut rng2);
        if let Some(CurrentElement::Value(value)) = &eval_context_value.current_element {
            assert_eq!(*value, 42);
        } else {
            panic!("Expected Value element");
        }
        
        // Test with team side element
        let eval_context_team = EvaluationContext::with_team_side_element(&battle_context, TeamSide::Enemy, &mut rng3);
        if let Some(CurrentElement::TeamSide(side)) = &eval_context_team.current_element {
            assert_eq!(*side, TeamSide::Enemy);
        } else {
            panic!("Expected TeamSide element");
        }
    }
}