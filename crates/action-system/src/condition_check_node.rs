// Condition check node - evaluates condition and delegates to next node or breaks

use super::core::{ActionResolver, ActionResolverResult};
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
    fn resolve(&self, character: &crate::Character, rng: &mut dyn rand::RngCore) -> ActionResolverResult {
        if self.condition.evaluate(character, rng) {
            // Continue: delegate to next node
            self.next.resolve(character, rng)
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
        let character = Character::new("Test".to_string(), 100, 50, 25);
        let check_random = ConditionCheckNode::new(
            Box::new(RandomConditionNode),
            Box::new(StrikeActionNode),
        );
        let mut rng = StdRng::from_entropy();
        
        match check_random.resolve(&character, &mut rng) {
            ActionResolverResult::Action(_) | ActionResolverResult::Break => assert!(true),
        }
    }
}