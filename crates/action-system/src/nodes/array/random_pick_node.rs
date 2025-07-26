// Generic RandomPickNode - selects a random element from arrays of any type

use crate::core::{NodeError, NodeResult};
use crate::nodes::evaluation_context::EvaluationContext;
use crate::nodes::unified_node::{CoreNode as Node, BoxedNode};
use rand::Rng;

/// Generic RandomPickNode that can pick from arrays of any type
pub struct RandomPickNode<T> {
    array_node: BoxedNode<Vec<T>>,
}

impl<T> RandomPickNode<T> {
    pub fn new(array_node: BoxedNode<Vec<T>>) -> Self {
        Self { array_node }
    }
}


// Generic implementation for all cloneable types
impl<'a, T: Clone + Send + Sync + 'static> Node<T, EvaluationContext<'a>> for RandomPickNode<T> {
    fn evaluate(&self, eval_context: &mut EvaluationContext) -> NodeResult<T> {
        let items = self.array_node.evaluate(eval_context)?;
        if items.is_empty() {
            return Err(NodeError::EvaluationError("Cannot pick from empty array".to_string()));
        }
        let index = eval_context.rng.gen_range(0..items.len());
        Ok(items[index].clone())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::nodes::array::team_members_node::TeamMembersNode;
    use crate::{BattleContext, Character, Team, TeamSide};
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
        let pick_node = RandomPickNode::<Character>::new(empty_array);
        
        let mut eval_context = EvaluationContext::new(&battle_context, &mut rng);
        let result = pick_node.evaluate(&mut eval_context);
        
        // Should return error for empty array
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Cannot pick from empty array"));
    }

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
        let pick_node = RandomPickNode::<Character>::new(team_array);
        
        let mut eval_context = EvaluationContext::new(&battle_context, &mut rng);
        let picked_character = pick_node.evaluate(&mut eval_context).unwrap();
        
        // Should pick one of the characters
        assert!([1, 2, 3].contains(&picked_character.id));
        assert!(["Char1", "Char2", "Char3"].contains(&picked_character.name.as_str()));
    }
}