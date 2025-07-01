// FilterList node - filters array elements based on condition
use crate::core::NodeResult;
use crate::nodes::array::CharacterArrayNode;
use crate::nodes::condition::ConditionNode;
use crate::Character;

/// Node that filters an array of characters based on a condition
#[derive(Debug)]
pub struct FilterListNode {
    array: Box<dyn CharacterArrayNode>,
    condition: Box<dyn ConditionNode>,
}

impl FilterListNode {
    pub fn new(
        array: Box<dyn CharacterArrayNode>,
        condition: Box<dyn ConditionNode>,
    ) -> Self {
        Self { array, condition }
    }
}

impl CharacterArrayNode for FilterListNode {
    fn evaluate(&self, eval_context: &crate::nodes::evaluation_context::EvaluationContext, rng: &mut dyn rand::RngCore) -> NodeResult<Vec<Character>> {
        // Get the array to filter
        let characters = self.array.evaluate(eval_context, rng)?;
        
        let mut filtered = Vec::new();
        
        // For each character in the array, evaluate the condition
        for character in characters {
            // Create an evaluation context with the current character as the element being processed
            // This allows the Element node to reference the current character being evaluated
            let element_eval_context = eval_context.with_new_element(&character);
            
            // Evaluate condition with the element-specific context
            let condition_result = self.condition.evaluate(&element_eval_context, rng)?;
            
            if condition_result {
                filtered.push(character);
            }
        }
        
        Ok(filtered)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nodes::array::team_members_node::TeamMembersNode;
    use crate::nodes::condition::greater_than_condition_node::GreaterThanConditionNode;
    use crate::nodes::character::character_hp_node::CharacterHpNode;
    use crate::nodes::character::element_node::ElementNode;
    use crate::nodes::value::constant_value_node::ConstantValueNode;
    use crate::{BattleContext};
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
        let hp_condition = Box::new(GreaterThanConditionNode::new(
            Box::new(CharacterHpNode::new(Box::new(ElementNode))), // Use Element node to reference current character being filtered
            Box::new(ConstantValueNode::new(50)),
        ));
        
        let filter_node = FilterListNode::new(team_array, hp_condition);
        
        let eval_context = EvaluationContext::new(&battle_context);
        let result = filter_node.evaluate(&eval_context, &mut rng).unwrap();
        
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
        let hp_condition = Box::new(GreaterThanConditionNode::new(
            Box::new(CharacterHpNode::new(Box::new(ElementNode))),
            Box::new(ConstantValueNode::new(90)),
        ));
        
        let filter_node = FilterListNode::new(team_array, hp_condition);
        
        let eval_context = EvaluationContext::new(&battle_context);
        let result = filter_node.evaluate(&eval_context, &mut rng).unwrap();
        
        // Should return empty array
        assert_eq!(result.len(), 0);
    }
}