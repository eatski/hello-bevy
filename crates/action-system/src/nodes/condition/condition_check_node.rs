// Condition check node - evaluates condition and delegates to next node or breaks

use crate::core::{ActionResolver, Action, NodeResult, NodeError};
use crate::nodes::evaluation_context::EvaluationContext;
use crate::nodes::unified_node::Node;

#[derive(Debug)]
pub struct ConditionCheckNode {
    condition: Box<dyn Node<bool>>,
    next: Box<dyn ActionResolver>,
}

impl ConditionCheckNode {
    pub fn new(condition: Box<dyn Node<bool>>, next: Box<dyn ActionResolver>) -> Self {
        Self { condition, next }
    }
}

impl ActionResolver for ConditionCheckNode {
    fn resolve(&self, eval_context: &EvaluationContext, rng: &mut dyn rand::RngCore) -> NodeResult<Box<dyn Action>> {
        let condition_result = self.condition.evaluate(eval_context, rng)?;
        if condition_result {
            // Continue: delegate to next node
            self.next.resolve(eval_context, rng)
        } else {
            Err(NodeError::Break)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Character, Team, TeamSide};
    use crate::{RandomConditionNode, StrikeActionNode};
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_condition_check_node() {
        let player = Character::new(1, "Player".to_string(), 100, 50, 25);
        let enemy = Character::new(2, "Enemy".to_string(), 80, 30, 20);
        let acting_character = Character::new(3, "Test".to_string(), 100, 50, 25);
        
        let player_team = Team::new("Player Team".to_string(), vec![player.clone(), acting_character.clone()]);
        let enemy_team = Team::new("Enemy Team".to_string(), vec![enemy.clone()]);
        let battle_context = crate::BattleContext::new(&acting_character, TeamSide::Player, &player_team, &enemy_team);
        
        let check_random = ConditionCheckNode::new(
            Box::new(RandomConditionNode),
            Box::new(StrikeActionNode::new(Box::new(crate::nodes::character::ActingCharacterNode))),
        );
        let mut rng = StdRng::from_entropy();
        
        let eval_context = EvaluationContext::new(&battle_context);
        let result = check_random.resolve(&eval_context, &mut rng);
        // Should either return an Action or Break error
        match result {
            Ok(_action) => assert!(true), // Got an action
            Err(NodeError::Break) => assert!(true), // Got break
            Err(_) => panic!("Unexpected error type"),
        }
    }
}