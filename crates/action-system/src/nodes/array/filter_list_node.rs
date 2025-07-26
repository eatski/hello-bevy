// FilterList node - filters array elements based on condition
use crate::core::NodeResult;
use crate::nodes::unified_node::{CoreNode as Node, BoxedNode};
use crate::nodes::evaluation_context::EvaluationContext;
use crate::nodes::array::mapping_node::AsUnknownValue;

/// Generic FilterListNode that works with any type
pub struct FilterListNode<T: Clone + Send + Sync + 'static> {
    array: BoxedNode<Vec<T>>,
    condition: BoxedNode<bool>,
}

impl<T: Clone + Send + Sync + 'static> FilterListNode<T> {
    pub fn new(
        array: BoxedNode<Vec<T>>,
        condition: BoxedNode<bool>,
    ) -> Self {
        Self { array, condition }
    }
}

impl<'a, T> Node<Vec<T>, EvaluationContext<'a>> for FilterListNode<T>
where
    T: Clone + Send + Sync + AsUnknownValue + 'static,
{
    fn evaluate(&self, eval_context: &mut crate::nodes::evaluation_context::EvaluationContext) -> NodeResult<Vec<T>> {
        // Get the array to filter
        let items = self.array.evaluate(eval_context)?;
        
        let mut filtered = Vec::new();
        
        // For each item in the array, evaluate the condition
        for item in items {
            // Create an evaluation context with the current item as the element being processed
            // This allows the Element node to reference the current item being evaluated
            let mut element_eval_context = eval_context.with_current_element_from_context(item.as_unknown_value());
            
            // Evaluate condition with the element-specific context
            let condition_result = self.condition.evaluate(&mut element_eval_context)?;
            
            if condition_result {
                filtered.push(item);
            }
        }
        
        Ok(filtered)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::nodes::array::team_members_node::TeamMembersNode;
    use crate::nodes::condition::GreaterThanNode;
    use crate::nodes::character::character_hp_value_node::CharacterHpValueNode;
    use crate::nodes::character::element_node::ElementNode;
    use crate::nodes::value::constant_value_node::ConstantValueNode;
    use crate::{BattleContext, Character};
    use crate::nodes::evaluation_context::EvaluationContext;
    use crate::{Team, TeamSide};
    use rand::SeedableRng;

    #[test]
    fn test_filter_list_by_hp() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        // Create characters with different HP values
        let mut low_hp_char = Character::new(1, "Low HP".to_string(), 100, 100, 10);
        low_hp_char.hp = 30;
        let mut high_hp_char = Character::new(2, "High HP".to_string(), 100, 100, 15);
        high_hp_char.hp = 80;
        let mut medium_hp_char = Character::new(3, "Medium HP".to_string(), 100, 100, 12);
        medium_hp_char.hp = 50;
        
        let player_team = Team::new("Player".to_string(), vec![low_hp_char.clone(), high_hp_char.clone(), medium_hp_char.clone()]);
        let enemy_team = Team::new("Enemy".to_string(), vec![]);
        
        let battle_context = BattleContext::new(&low_hp_char, TeamSide::Player, &player_team, &enemy_team);
        
        // Create FilterList that filters characters with HP > 50
        let team_array = Box::new(TeamMembersNode::new(TeamSide::Player));
        let hp_condition = Box::new(GreaterThanNode::new(
            Box::new(CharacterHpValueNode::new(Box::new(ElementNode::<Character>::new()))), // Use Element node to reference current character being filtered
            Box::new(ConstantValueNode::new(50)),
        ));
        
        let filter_node = FilterListNode::new(team_array, hp_condition);
        
        let mut eval_context = EvaluationContext::new(&battle_context, &mut rng);
        let result = Node::<Vec<Character>, EvaluationContext>::evaluate(&filter_node, &mut eval_context).unwrap();
        
        // Should only return the high HP character (80 > 50)
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, 2);
        assert_eq!(result[0].name, "High HP");
    }
    
    #[test]
    fn test_filter_list_empty_result() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        // Create characters with low HP values
        let mut char1 = Character::new(1, "Char1".to_string(), 100, 100, 10);
        char1.hp = 20;
        let mut char2 = Character::new(2, "Char2".to_string(), 100, 100, 15);
        char2.hp = 30;
        
        let player_team = Team::new("Player".to_string(), vec![char1.clone(), char2.clone()]);
        let enemy_team = Team::new("Enemy".to_string(), vec![]);
        
        let battle_context = BattleContext::new(&char1, TeamSide::Player, &player_team, &enemy_team);
        
        // Create FilterList that filters characters with HP > 90 (none should match)
        let team_array = Box::new(TeamMembersNode::new(TeamSide::Player));
        let hp_condition = Box::new(GreaterThanNode::new(
            Box::new(CharacterHpValueNode::new(Box::new(ElementNode::<Character>::new()))),
            Box::new(ConstantValueNode::new(90)),
        ));
        
        let filter_node = FilterListNode::new(team_array, hp_condition);
        
        let mut eval_context = EvaluationContext::new(&battle_context, &mut rng);
        let result = Node::<Vec<Character>, EvaluationContext>::evaluate(&filter_node, &mut eval_context).unwrap();
        
        // Should return empty array
        assert_eq!(result.len(), 0);
    }
}