// Condition check node - evaluates condition and delegates to next node or breaks

use crate::core::{ActionResolver, ActionResolverResult};
use super::condition_nodes::ConditionNode;

#[derive(Debug)]
pub struct ConditionCheckNode {
    condition: Box<dyn ConditionNode>,
    next: Box<dyn ActionResolver>,
}

impl ConditionCheckNode {
    pub fn new(condition: Box<dyn ConditionNode>, next: Box<dyn ActionResolver>) -> Self {
        Self { condition, next }
    }
}

impl ActionResolver for ConditionCheckNode {
    fn resolve(&self, battle_context: &crate::BattleContext, rng: &mut dyn rand::RngCore) -> ActionResolverResult {
        if self.condition.evaluate(battle_context, rng) {
            // Continue: delegate to next node
            self.next.resolve(battle_context, rng)
        } else {
            ActionResolverResult::Break
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Character;
    use crate::{RandomConditionNode, StrikeActionNode};
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_condition_check_node() {
        let player = Character::new("Player".to_string(), 100, 50, 25);
        let enemy = Character::new("Enemy".to_string(), 80, 30, 20);
        let acting_character = Character::new("Test".to_string(), 100, 50, 25);
        let battle_context = crate::BattleContext::new(&acting_character, &player, &enemy);
        
        let check_random = ConditionCheckNode::new(
            Box::new(RandomConditionNode),
            Box::new(StrikeActionNode),
        );
        let mut rng = StdRng::from_entropy();
        
        let result = check_random.resolve(&battle_context, &mut rng);
        assert!(matches!(result, ActionResolverResult::Action(_) | ActionResolverResult::Break));
    }
}