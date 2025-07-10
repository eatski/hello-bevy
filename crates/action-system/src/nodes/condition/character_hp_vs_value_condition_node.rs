// CharacterHP vs Value condition node - compares CharacterHP with i32

use crate::nodes::evaluation_context::EvaluationContext;
use crate::nodes::unified_node::Node;
use crate::core::character_hp::CharacterHP;

pub struct CharacterHpVsValueConditionNode {
    pub left: Box<dyn Node<CharacterHP>>,
    pub right: Box<dyn Node<i32>>,
}

impl CharacterHpVsValueConditionNode {
    pub fn new(left: Box<dyn Node<CharacterHP>>, right: Box<dyn Node<i32>>) -> Self {
        Self { left, right }
    }
}

impl Node<bool> for CharacterHpVsValueConditionNode {
    fn evaluate(&self, eval_context: &EvaluationContext, rng: &mut dyn rand::RngCore) -> crate::core::NodeResult<bool> {
        let left_value = self.left.evaluate(eval_context, rng)?;
        let right_value = self.right.evaluate(eval_context, rng)?;
        Ok(left_value > right_value)
    }
}

pub struct ValueVsCharacterHpConditionNode {
    pub left: Box<dyn Node<i32>>,
    pub right: Box<dyn Node<CharacterHP>>,
}

impl ValueVsCharacterHpConditionNode {
    pub fn new(left: Box<dyn Node<i32>>, right: Box<dyn Node<CharacterHP>>) -> Self {
        Self { left, right }
    }
}

impl Node<bool> for ValueVsCharacterHpConditionNode {
    fn evaluate(&self, eval_context: &EvaluationContext, rng: &mut dyn rand::RngCore) -> crate::core::NodeResult<bool> {
        let left_value = self.left.evaluate(eval_context, rng)?;
        let right_value = self.right.evaluate(eval_context, rng)?;
        Ok(left_value > right_value.get_hp())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Character, Team, TeamSide, ActingCharacterNode, ConstantValueNode, CharacterToHpNode};
    use crate::nodes::character::BattleContext;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_character_hp_vs_value_condition() {
        let mut rng = StdRng::seed_from_u64(12345);
        
        let mut character = Character::new(1, "Test".to_string(), 100, 50, 25);
        character.hp = 80;
        
        let team = Team::new("Test Team".to_string(), vec![character.clone()]);
        let battle_context = BattleContext::new(&character, TeamSide::Player, &team, &team);
        let eval_context = EvaluationContext::new(&battle_context);
        
        // Test CharacterHP(80) > Value(50) = true
        let hp_node = Box::new(CharacterToHpNode::new(Box::new(ActingCharacterNode)));
        let value_node = Box::new(ConstantValueNode::new(50));
        let condition = CharacterHpVsValueConditionNode::new(hp_node, value_node);
        
        let result = Node::<bool>::evaluate(&condition, &eval_context, &mut rng).unwrap();
        assert!(result);
        
        // Test CharacterHP(80) > Value(100) = false
        let hp_node2 = Box::new(CharacterToHpNode::new(Box::new(ActingCharacterNode)));
        let value_node2 = Box::new(ConstantValueNode::new(100));
        let condition2 = CharacterHpVsValueConditionNode::new(hp_node2, value_node2);
        
        let result2 = Node::<bool>::evaluate(&condition2, &eval_context, &mut rng).unwrap();
        assert!(!result2);
    }

    #[test]
    fn test_value_vs_character_hp_condition() {
        let mut rng = StdRng::seed_from_u64(12345);
        
        let mut character = Character::new(1, "Test".to_string(), 100, 50, 25);
        character.hp = 60;
        
        let team = Team::new("Test Team".to_string(), vec![character.clone()]);
        let battle_context = BattleContext::new(&character, TeamSide::Player, &team, &team);
        let eval_context = EvaluationContext::new(&battle_context);
        
        // Test Value(80) > CharacterHP(60) = true
        let value_node = Box::new(ConstantValueNode::new(80));
        let hp_node = Box::new(CharacterToHpNode::new(Box::new(ActingCharacterNode)));
        let condition = ValueVsCharacterHpConditionNode::new(value_node, hp_node);
        
        let result = Node::<bool>::evaluate(&condition, &eval_context, &mut rng).unwrap();
        assert!(result);
        
        // Test Value(40) > CharacterHP(60) = false
        let value_node2 = Box::new(ConstantValueNode::new(40));
        let hp_node2 = Box::new(CharacterToHpNode::new(Box::new(ActingCharacterNode)));
        let condition2 = ValueVsCharacterHpConditionNode::new(value_node2, hp_node2);
        
        let result2 = Node::<bool>::evaluate(&condition2, &eval_context, &mut rng).unwrap();
        assert!(!result2);
    }
}