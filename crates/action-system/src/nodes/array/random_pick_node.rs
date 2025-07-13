// Generic RandomPickNode - selects a random element from arrays of any type

use crate::core::{NodeError, NodeResult};
use crate::nodes::evaluation_context::EvaluationContext;
use crate::nodes::unified_node::Node;
use crate::Character;
use rand::Rng;

/// Generic RandomPickNode that can pick from arrays of any type
pub struct GenericRandomPickNode<T> {
    array_node: Box<dyn Node<Vec<T>>>,
}

impl<T> GenericRandomPickNode<T> {
    pub fn new(array_node: Box<dyn Node<Vec<T>>>) -> Self {
        Self { array_node }
    }
}

/// Character-specific RandomPickNode (returns character ID)
pub type CharacterRandomPickNode = GenericRandomPickNode<Character>;

/// Value-specific RandomPickNode (returns picked value)
pub type ValueRandomPickNode = GenericRandomPickNode<i32>;


// Unified implementations

impl Node<Character> for CharacterRandomPickNode {
    fn evaluate(&self, eval_context: &EvaluationContext, rng: &mut dyn rand::RngCore) -> NodeResult<Character> {
        let characters = self.array_node.evaluate(eval_context, rng)?;
        if characters.is_empty() {
            return Err(NodeError::EvaluationError("Cannot pick from empty character array".to_string()));
        }
        let index = rng.gen_range(0..characters.len());
        Ok(characters[index].clone())
    }
}

impl Node<i32> for ValueRandomPickNode {
    fn evaluate(&self, eval_context: &EvaluationContext, rng: &mut dyn rand::RngCore) -> NodeResult<i32> {
        let values = self.array_node.evaluate(eval_context, rng)?;
        if values.is_empty() {
            return Err(NodeError::EvaluationError("Cannot pick from empty value array".to_string()));
        }
        let index = rng.gen_range(0..values.len());
        Ok(values[index])
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::nodes::array::team_members_node::TeamMembersNode;
    // ConstantArrayNode removed - using team members in tests
    use crate::{BattleContext, Team, TeamSide};
    use rand::SeedableRng;


    #[test]
    fn test_character_random_pick_empty_array() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let char1 = Character::new(1, "Char1".to_string(), 100, 100, 10);
        let player_team = Team::new("Player".to_string(), vec![char1.clone()]);
        let enemy_team = Team::new("Enemy".to_string(), vec![]);
        
        let battle_context = BattleContext::new(&char1, TeamSide::Player, &player_team, &enemy_team);
        
        // Create empty team array
        let empty_array = Box::new(TeamMembersNode::new(TeamSide::Enemy)); // Enemy team is empty
        let pick_node = CharacterRandomPickNode::new(empty_array);
        
        let eval_context = EvaluationContext::new(&battle_context);
        let result = Node::<Character>::evaluate(&pick_node, &eval_context, &mut rng);
        
        // Should return error for empty array
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Cannot pick from empty character array"));
    }

    // Removed test_value_random_pick_node - ConstantArrayNode deleted

    // Removed test_value_random_pick_empty_array - ConstantArrayNode deleted

    // Unified Node<T> tests

    // Removed test_value_random_pick_node_unified - ConstantArrayNode deleted

    #[test]
    fn test_character_random_pick_node_returns_character() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        // Create characters
        let char1 = Character::new(1, "Char1".to_string(), 100, 100, 10);
        let char2 = Character::new(2, "Char2".to_string(), 100, 100, 15);
        let char3 = Character::new(3, "Char3".to_string(), 100, 100, 12);
        
        let player_team = Team::new("Player".to_string(), vec![char1.clone(), char2.clone(), char3.clone()]);
        let enemy_team = Team::new("Enemy".to_string(), vec![]);
        
        let battle_context = BattleContext::new(&char1, TeamSide::Player, &player_team, &enemy_team);
        
        // Create CharacterRandomPickNode
        let team_array = Box::new(TeamMembersNode::new(TeamSide::Player));
        let pick_node = CharacterRandomPickNode::new(team_array);
        
        let eval_context = EvaluationContext::new(&battle_context);
        let picked_character = Node::<Character>::evaluate(&pick_node, &eval_context, &mut rng).unwrap();
        
        // Should pick one of the characters
        assert!([1, 2, 3].contains(&picked_character.id));
        assert!(["Char1", "Char2", "Char3"].contains(&picked_character.name.as_str()));
    }
}