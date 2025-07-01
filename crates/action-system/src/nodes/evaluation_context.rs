// Evaluation context - manages the context for node evaluation including current element being processed
use crate::Character;
use crate::nodes::character::BattleContext;

/// Context for evaluating nodes, includes both battle context and current element being processed
#[derive(Debug)]
pub struct EvaluationContext<'a> {
    /// The battle context containing teams and acting character
    pub battle_context: &'a BattleContext<'a>,
    /// The current element being processed (used by Element node in array operations)
    pub current_element: Option<&'a Character>,
}

impl<'a> EvaluationContext<'a> {
    /// Creates a new EvaluationContext with battle context only
    pub fn new(battle_context: &'a BattleContext<'a>) -> Self {
        Self {
            battle_context,
            current_element: None,
        }
    }
    
    /// Creates a new EvaluationContext with both battle context and current element
    pub fn with_element(battle_context: &'a BattleContext<'a>, current_element: &'a Character) -> Self {
        Self {
            battle_context,
            current_element: Some(current_element),
        }
    }
    
    /// Gets the current element being processed, or falls back to acting character
    pub fn get_current_character(&self) -> &'a Character {
        self.current_element.unwrap_or(self.battle_context.get_acting_character())
    }
    
    /// Gets the battle context
    pub fn get_battle_context(&self) -> &'a BattleContext<'a> {
        self.battle_context
    }
    
    /// Creates a new EvaluationContext with a different current element
    pub fn with_new_element(&self, element: &'a Character) -> EvaluationContext<'a> {
        EvaluationContext {
            battle_context: self.battle_context,
            current_element: Some(element),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Team, TeamSide};

    #[test]
    fn test_evaluation_context_new() {
        let character = Character::new(1, "Test".to_string(), 100, 100, 20);
        let team = Team::new("Test Team".to_string(), vec![character.clone()]);
        let battle_context = BattleContext::new(&character, TeamSide::Player, &team, &team);
        
        let eval_context = EvaluationContext::new(&battle_context);
        
        assert!(eval_context.current_element.is_none());
        assert_eq!(eval_context.get_current_character().id, 1);
    }
    
    #[test]
    fn test_evaluation_context_with_element() {
        let acting_character = Character::new(1, "Acting".to_string(), 100, 100, 20);
        let element_character = Character::new(2, "Element".to_string(), 80, 100, 15);
        let team = Team::new("Test Team".to_string(), vec![acting_character.clone(), element_character.clone()]);
        let battle_context = BattleContext::new(&acting_character, TeamSide::Player, &team, &team);
        
        let eval_context = EvaluationContext::with_element(&battle_context, &element_character);
        
        assert!(eval_context.current_element.is_some());
        assert_eq!(eval_context.get_current_character().id, 2); // Should return element character
        assert_eq!(eval_context.get_battle_context().get_acting_character().id, 1); // Battle context should be unchanged
    }
    
    #[test]
    fn test_evaluation_context_with_new_element() {
        let acting_character = Character::new(1, "Acting".to_string(), 100, 100, 20);
        let element1 = Character::new(2, "Element1".to_string(), 80, 100, 15);
        let element2 = Character::new(3, "Element2".to_string(), 60, 100, 12);
        let team = Team::new("Test Team".to_string(), vec![acting_character.clone(), element1.clone(), element2.clone()]);
        let battle_context = BattleContext::new(&acting_character, TeamSide::Player, &team, &team);
        
        let eval_context1 = EvaluationContext::with_element(&battle_context, &element1);
        let eval_context2 = eval_context1.with_new_element(&element2);
        
        assert_eq!(eval_context1.get_current_character().id, 2);
        assert_eq!(eval_context2.get_current_character().id, 3);
    }
}