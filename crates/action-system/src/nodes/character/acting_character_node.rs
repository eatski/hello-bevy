// Acting character node - returns the character currently performing action calculation

use crate::nodes::unified_node::Node;

#[derive(Debug)]
pub struct ActingCharacterNode;

impl Node<i32> for ActingCharacterNode {
    fn evaluate(&self, eval_context: &crate::nodes::evaluation_context::EvaluationContext, _rng: &mut dyn rand::RngCore) -> crate::core::NodeResult<i32> {
        Ok(eval_context.get_battle_context().get_acting_character().id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Character, Team, TeamSide};
    use crate::nodes::evaluation_context::EvaluationContext;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test] 
    fn test_acting_character_node_unified() {
        let player = Character::new(1, "Player".to_string(), 100, 50, 25);
        let enemy = Character::new(2, "Enemy".to_string(), 80, 30, 20);
        let acting_character = Character::new(3, "Test".to_string(), 100, 50, 25);
        
        let player_team = Team::new("Player Team".to_string(), vec![player.clone(), acting_character.clone()]);
        let enemy_team = Team::new("Enemy Team".to_string(), vec![enemy.clone()]);
        let battle_context = crate::BattleContext::new(&acting_character, TeamSide::Player, &player_team, &enemy_team);
        
        let mut rng = StdRng::from_entropy();
        
        // Test ActingCharacter node
        let acting_char_node = ActingCharacterNode;
        let eval_context = EvaluationContext::new(&battle_context);
        let returned_char_id = Node::<i32>::evaluate(&acting_char_node, &eval_context, &mut rng).unwrap();
        assert_eq!(returned_char_id, acting_character.id);

        // Test as boxed trait object
        let boxed_node: Box<dyn Node<i32>> = Box::new(ActingCharacterNode);
        let boxed_result = boxed_node.evaluate(&eval_context, &mut rng).unwrap();
        assert_eq!(boxed_result, acting_character.id);
    }
}