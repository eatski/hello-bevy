// Condition check node - evaluates condition and delegates to next node or breaks

use crate::core::{Action, NodeResult, NodeError};
use crate::nodes::evaluation_context::EvaluationContext;
use node_core::Node;

pub struct ConditionCheckNode {
    condition: Box<dyn for<'a> Node<bool, EvaluationContext<'a>>>,
    next: Box<dyn for<'a> Node<Box<dyn Action>, EvaluationContext<'a>>>,
}

impl ConditionCheckNode {
    pub fn new(condition: Box<dyn for<'a> Node<bool, EvaluationContext<'a>>>, next: Box<dyn for<'a> Node<Box<dyn Action>, EvaluationContext<'a>>>) -> Self {
        Self { condition, next }
    }
}

impl<'a> Node<Box<dyn Action>, EvaluationContext<'a>> for ConditionCheckNode {
    fn evaluate(&self, eval_context: &mut EvaluationContext<'a>) -> NodeResult<Box<dyn Action>> {
        let condition_result = self.condition.evaluate(eval_context)?;
        if condition_result {
            // Continue: delegate to next node
            self.next.evaluate(eval_context)
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
        
        let mut eval_context = EvaluationContext::new(&battle_context, &mut rng);
        let result = Node::<Box<dyn Action>>::evaluate(&check_random, &mut eval_context);
        // Should either return an Action or Break error
        match result {
            Ok(_action) => assert!(true), // Got an action
            Err(NodeError::Break) => assert!(true), // Got break
            Err(_) => panic!("Unexpected error type"),
        }
    }
}