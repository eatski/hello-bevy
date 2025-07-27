use crate::nodes::unified_node::{CoreNode as Node, BoxedNode};
use crate::core::{NodeResult, Numeric};
use crate::nodes::evaluation_context::EvaluationContext;

/// Node that wraps any Numeric value as Box<dyn Numeric>
pub struct NumericNode<T: Numeric + Clone + 'static> {
    inner: BoxedNode<T>,
}

impl<T: Numeric + Clone + 'static> NumericNode<T> {
    pub fn new(inner: BoxedNode<T>) -> Self {
        Self { inner }
    }
}

impl<'a, T: Numeric + Clone + 'static> Node<Box<dyn Numeric>, EvaluationContext<'a>> for NumericNode<T> {
    fn evaluate(&self, eval_context: &mut EvaluationContext) -> NodeResult<Box<dyn Numeric>> {
        let value = self.inner.evaluate(eval_context)?;
        Ok(value.clone_box())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Character, CharacterHP, TeamSide, Team};
    use crate::nodes::character::BattleContext;
    use crate::nodes::value::ConstantValueNode;
    use rand::SeedableRng;

    #[test]
    fn test_numeric_node_from_i32() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let character = Character::new(1, "Test".to_string(), 100, 100, 10);
        let team = Team::new("Test Team".to_string(), vec![character.clone()]);
        let battle_context = BattleContext::new(&character, TeamSide::Player, &team, &team);
        let mut eval_context = EvaluationContext::new(&battle_context, &mut rng);
        
        let inner_node = Box::new(ConstantValueNode::new(42));
        let numeric_node = NumericNode::new(inner_node);
        
        let result = numeric_node.evaluate(&mut eval_context).unwrap();
        assert_eq!(result.to_i32(), 42);
    }

    #[test]
    fn test_numeric_node_from_character_hp() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let character = Character::new(1, "Test".to_string(), 100, 100, 10);
        let team = Team::new("Test Team".to_string(), vec![character.clone()]);
        let battle_context = BattleContext::new(&character, TeamSide::Player, &team, &team);
        let mut eval_context = EvaluationContext::new(&battle_context, &mut rng);
        
        let test_char = Character::new(2, "TestChar".to_string(), 80, 80, 10);
        let char_hp = CharacterHP::from_character_with_hp(test_char, 75);
        
        // Helper node that returns a constant CharacterHP
        struct ConstantCharacterHPNode {
            value: CharacterHP,
        }
        
        impl<'a> Node<CharacterHP, EvaluationContext<'a>> for ConstantCharacterHPNode {
            fn evaluate(&self, _eval_context: &mut EvaluationContext) -> NodeResult<CharacterHP> {
                Ok(self.value.clone())
            }
        }
        
        let inner_node = Box::new(ConstantCharacterHPNode { value: char_hp });
        let numeric_node = NumericNode::new(inner_node);
        
        let result = numeric_node.evaluate(&mut eval_context).unwrap();
        assert_eq!(result.to_i32(), 75);
    }
}