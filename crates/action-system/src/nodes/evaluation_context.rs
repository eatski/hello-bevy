// Evaluation context - manages the context for node evaluation including current element being processed
use crate::nodes::character::BattleContext;
use crate::nodes::unknown_value::UnknownValue;

/// Context for evaluating nodes, includes battle context, current element, and RNG
pub struct EvaluationContext<'a> {
    /// The battle context containing teams and acting character
    pub battle_context: &'a BattleContext<'a>,
    /// The current element being processed (used by Element node in array operations)
    pub current_element: Option<UnknownValue>,
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
    
    
    /// Gets the battle context
    pub fn get_battle_context(&self) -> &'a BattleContext<'a> {
        self.battle_context
    }
    
    /// Creates a new EvaluationContext with a different current element
    /// This method takes ownership of the RNG to avoid borrowing issues
    pub fn with_current_element_from_context(&mut self, element: UnknownValue) -> EvaluationContext<'_> {
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
    use crate::{Character, Team, TeamSide};

    
    #[test]
    fn test_evaluation_context_with_new_element() {
        use rand::SeedableRng;
        let mut rng1 = rand::rngs::StdRng::seed_from_u64(12345);
        
        let acting_character = Character::new(1, "Acting".to_string(), 100, 100, 20);
        let element1 = Character::new(2, "Element1".to_string(), 80, 100, 15);
        let element2 = Character::new(3, "Element2".to_string(), 60, 100, 12);
        let team = Team::new("Test Team".to_string(), vec![acting_character.clone(), element1.clone(), element2.clone()]);
        let battle_context = BattleContext::new(&acting_character, TeamSide::Player, &team, &team);
        
        let mut eval_context1 = EvaluationContext::new(&battle_context, &mut rng1);
        eval_context1.current_element = Some(UnknownValue::Character(element1.clone()));
        
        // Check the original context first
        if let Some(UnknownValue::Character(character)) = &eval_context1.current_element {
            assert_eq!(character.id, 2);
        } else {
            panic!("Expected Character element");
        }
        
        // Now create the new context with a different element
        let eval_context2 = eval_context1.with_current_element_from_context(UnknownValue::Character(element2.clone()));
        
        if let Some(UnknownValue::Character(character)) = &eval_context2.current_element {
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
        let mut rng4 = rand::rngs::StdRng::seed_from_u64(12345);
        
        let acting_character = Character::new(1, "Acting".to_string(), 100, 100, 20);
        let team = Team::new("Test Team".to_string(), vec![acting_character.clone()]);
        let battle_context = BattleContext::new(&acting_character, TeamSide::Player, &team, &team);
        
        // Test with character element
        let character_element = Character::new(2, "Element".to_string(), 80, 100, 15);
        let mut eval_context_char = EvaluationContext::new(&battle_context, &mut rng1);
        eval_context_char.current_element = Some(UnknownValue::Character(character_element));
        if let Some(UnknownValue::Character(character)) = &eval_context_char.current_element {
            assert_eq!(character.id, 2);
        } else {
            panic!("Expected Character element");
        }
        
        // Test with value element
        let mut eval_context_value = EvaluationContext::new(&battle_context, &mut rng2);
        eval_context_value.current_element = Some(UnknownValue::Value(42));
        if let Some(UnknownValue::Value(value)) = &eval_context_value.current_element {
            assert_eq!(*value, 42);
        } else {
            panic!("Expected Value element");
        }
        
        // Test with team side element
        let mut eval_context_team = EvaluationContext::new(&battle_context, &mut rng3);
        eval_context_team.current_element = Some(UnknownValue::TeamSide(TeamSide::Enemy));
        if let Some(UnknownValue::TeamSide(side)) = &eval_context_team.current_element {
            assert_eq!(*side, TeamSide::Enemy);
        } else {
            panic!("Expected TeamSide element");
        }
        
        // Test with character HP element
        let character_for_hp = Character::new(3, "HP Element".to_string(), 70, 100, 18);
        let character_hp = crate::core::character_hp::CharacterHP::new(character_for_hp);
        let mut eval_context_hp = EvaluationContext::new(&battle_context, &mut rng4);
        eval_context_hp.current_element = Some(UnknownValue::CharacterHP(character_hp.clone()));
        if let Some(UnknownValue::CharacterHP(hp)) = &eval_context_hp.current_element {
            assert_eq!(hp.hp_value, 70);
        } else {
            panic!("Expected CharacterHP element");
        }
    }
}