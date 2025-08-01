// Action system crate - node-based action resolution system

pub mod core;
pub mod nodes;
pub mod system;

// Re-export essential types only
pub use core::{Character, Team, TeamSide, CharacterHP, Action, BattleState, RuleNode, NodeError, NodeResult, Numeric};
// Export Node trait and related types for external crates
pub use nodes::unified_node::{CoreNode as Node, BoxedNode};
pub use nodes::condition::{ConditionCheckNode, RandomConditionNode, CharacterTeamNode, GreaterThanNode, LessThanNode};
pub use nodes::value::{ConstantValueNode, EnemyNode, HeroNode, NumericNode};
pub use nodes::character::{BattleContext, ActingCharacterNode, CharacterToHpNode, CharacterHpValueNode, CharacterHpToCharacterNode, ElementNode};
pub use nodes::evaluation_context::EvaluationContext;
pub use nodes::action::{StrikeActionNode, HealActionNode};
pub use nodes::array::{AllCharactersNode, TeamMembersNode, TeamMembersNodeWithNode, CountArrayNode, RandomPickNode, FilterListNode, MappingNode, AllTeamSidesNode, MaxNode, MinNode};
pub use system::ActionCalculationSystem;

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn test_all_characters_node() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let player = Character::new(1, "Player".to_string(), 100, 50, 10);
        let enemy = Character::new(2, "Enemy".to_string(), 80, 30, 15);
        
        let player_team = Team::new("Player Team".to_string(), vec![player.clone()]);
        let enemy_team = Team::new("Enemy Team".to_string(), vec![enemy.clone()]);
        let battle_context = BattleContext::new(&player, TeamSide::Player, &player_team, &enemy_team);
        
        let all_chars_node = AllCharactersNode::new();
        let mut eval_context = EvaluationContext::new(&battle_context, &mut rng);
        let result = all_chars_node.evaluate(&mut eval_context).unwrap();
        
        assert_eq!(result.len(), 2);
        assert!(result.iter().any(|c| c.name == "Player"));
        assert!(result.iter().any(|c| c.name == "Enemy"));
    }

    #[test]
    fn test_team_members_node() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let player_team = Team::new("Player Team".to_string(), vec![
            Character::new(3, "Player1".to_string(), 100, 50, 10),
            Character::new(4, "Player2".to_string(), 0, 30, 8),  // Dead
        ]);
        let enemy_team = Team::new("Enemy Team".to_string(), vec![
            Character::new(5, "Enemy1".to_string(), 80, 30, 15),
        ]);
        
        let acting_character = &player_team.members[0];
        let battle_context = BattleContext::new(
            acting_character,
            TeamSide::Player,
            &player_team,
            &enemy_team,
        );
        
        let team_node = TeamMembersNode::new(TeamSide::Player);
        let mut eval_context = EvaluationContext::new(&battle_context, &mut rng);
        let result = team_node.evaluate(&mut eval_context).unwrap();
        
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].name, "Player1");
        assert_eq!(result[1].name, "Player2");
    }

    #[test]
    fn test_count_array_node() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let player = Character::new(6, "Player".to_string(), 100, 50, 10);
        let enemy = Character::new(7, "Enemy".to_string(), 80, 30, 15);
        
        let player_team = Team::new("Player Team".to_string(), vec![player.clone()]);
        let enemy_team = Team::new("Enemy Team".to_string(), vec![enemy.clone()]);
        let battle_context = BattleContext::new(&player, TeamSide::Player, &player_team, &enemy_team);
        
        let all_chars_node = Box::new(AllCharactersNode::new());
        let count_node = CountArrayNode::new(all_chars_node);
        let mut eval_context = EvaluationContext::new(&battle_context, &mut rng);
        let result = count_node.evaluate(&mut eval_context).unwrap();
        
        assert_eq!(result, 2);
    }


    #[test]
    fn test_random_pick_character_node() {
        
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let player_team = Team::new("Player Team".to_string(), vec![
            Character::new(8, "Player1".to_string(), 100, 50, 10),
            Character::new(9, "Player2".to_string(), 80, 30, 8),
        ]);
        let enemy_team = Team::new("Enemy Team".to_string(), vec![
            Character::new(10, "Enemy1".to_string(), 60, 20, 12),
        ]);
        
        let acting_character = &player_team.members[0];
        let battle_context = BattleContext::new(
            acting_character,
            TeamSide::Player,
            &player_team,
            &enemy_team,
        );
        
        // Test random pick from character array
        let team_members_node = Box::new(TeamMembersNode::new(TeamSide::Player));
        let random_pick_node = RandomPickNode::<Character>::new(team_members_node);
        let mut eval_context = EvaluationContext::new(&battle_context, &mut rng);
        let result_character = random_pick_node.evaluate(&mut eval_context).unwrap();
        
        // Should pick one of the team members
        let player1_id = player_team.members[0].id;
        let player2_id = player_team.members[1].id;
        assert!(result_character.id == player1_id || result_character.id == player2_id);
    }


    #[test]
    fn test_random_pick_from_team() {
        
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let player_team = Team::new("Player Team".to_string(), vec![
            Character::new(11, "Player1".to_string(), 0, 50, 10),  // Dead
            Character::new(12, "Player2".to_string(), 0, 30, 8),   // Dead
        ]);
        let enemy_team = Team::new("Enemy Team".to_string(), vec![
            Character::new(13, "Enemy1".to_string(), 60, 20, 12),
        ]);
        
        let acting_character = &player_team.members[0];
        let battle_context = BattleContext::new(
            acting_character,
            TeamSide::Player,
            &player_team,
            &enemy_team,
        );
        
        // Test random pick from team (including dead members)
        let team_members_node = Box::new(TeamMembersNode::new(TeamSide::Player));
        let random_pick_node = RandomPickNode::<Character>::new(team_members_node);
        let mut eval_context = EvaluationContext::new(&battle_context, &mut rng);
        let result = random_pick_node.evaluate(&mut eval_context);
        
        // Should succeed since team has members (even if dead)
        assert!(result.is_ok());
        let character = result.unwrap();
        let player1_id = player_team.members[0].id;
        let player2_id = player_team.members[1].id;
        assert!(character.id == player1_id || character.id == player2_id);
    }

    #[test]
    fn test_filter_list_node_integration() {
        use crate::nodes::array::FilterListNode;
        use crate::nodes::character::ElementNode;
        use crate::nodes::character::character_hp_value_node::CharacterHpValueNode;
        use crate::nodes::condition::GreaterThanNode;
        use crate::nodes::value::ConstantValueNode;
        
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        // Create characters with different HP values
        let mut low_hp_char = Character::new(14, "Low HP".to_string(), 100, 100, 10);
        low_hp_char.hp = 30;
        let mut high_hp_char = Character::new(15, "High HP".to_string(), 100, 100, 15);
        high_hp_char.hp = 80;
        let mut medium_hp_char = Character::new(16, "Medium HP".to_string(), 100, 100, 12);
        medium_hp_char.hp = 60;
        
        let player_team = Team::new("Player".to_string(), vec![
            low_hp_char.clone(), 
            high_hp_char.clone(), 
            medium_hp_char.clone()
        ]);
        let enemy_team = Team::new("Enemy".to_string(), vec![]);
        
        let battle_context = BattleContext::new(&low_hp_char, TeamSide::Player, &player_team, &enemy_team);
        
        // Create FilterList that filters characters with HP > 50
        let team_array = Box::new(TeamMembersNode::new(TeamSide::Player));
        let hp_condition = Box::new(GreaterThanNode::new(
            Box::new(NumericNode::new(Box::new(CharacterHpValueNode::new(Box::new(ElementNode::<Character>::new()))))), // Use Element to reference current character
            Box::new(NumericNode::new(Box::new(ConstantValueNode::new(50)))),
        ));
        
        let filter_node = FilterListNode::<Character>::new(team_array, hp_condition);
        
        let mut eval_context = EvaluationContext::new(&battle_context, &mut rng);
        let result = filter_node.evaluate(&mut eval_context).unwrap();
        
        // Should return characters with HP > 50 (high_hp_char: 80, medium_hp_char: 60)
        assert_eq!(result.len(), 2);
        let result_ids: Vec<i32> = result.iter().map(|c| c.id).collect();
        assert!(result_ids.contains(&15)); // high_hp_char
        assert!(result_ids.contains(&16)); // medium_hp_char
        assert!(!result_ids.contains(&14)); // low_hp_char should be filtered out
    }

    #[test]
    fn test_element_node_integration() {
        use crate::nodes::character::ElementNode;
        use crate::nodes::unknown_value::UnknownValue;
        
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let character = Character::new(17, "Test Character".to_string(), 100, 100, 20);
        let current_element = Character::new(42, "Current Element".to_string(), 80, 80, 15);
        let team = Team::new("Test Team".to_string(), vec![character.clone(), current_element.clone()]);
        let battle_context = BattleContext::new(&character, TeamSide::Player, &team, &team);
        
        let element_node = ElementNode::<Character>::new();
        let mut eval_context = EvaluationContext::new(&battle_context, &mut rng);
        eval_context.current_element = Some(UnknownValue::Character(current_element.clone()));
        let result: Character = element_node.evaluate(&mut eval_context).unwrap();
        
        // Should return the current element
        assert_eq!(result.id, 42);
    }

    #[test]
    fn test_max_node_with_character_hp_integration() {
        use crate::nodes::array::{MaxNode, MappingNode, TeamMembersNode};
        use crate::nodes::character::{CharacterHpValueNode, ElementNode};
        
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        // Create characters with different HP values
        let mut char1 = Character::new(19, "Character 1".to_string(), 100, 100, 10);
        char1.hp = 30;
        let mut char2 = Character::new(20, "Character 2".to_string(), 100, 100, 15);
        char2.hp = 80;
        let mut char3 = Character::new(21, "Character 3".to_string(), 100, 100, 12);
        char3.hp = 60;
        
        let player_team = Team::new("Player".to_string(), vec![
            char1.clone(), 
            char2.clone(), 
            char3.clone()
        ]);
        let enemy_team = Team::new("Enemy".to_string(), vec![]);
        
        let battle_context = BattleContext::new(&char1, TeamSide::Player, &player_team, &enemy_team);
        
        // Create a mapping from team members to their HP values
        let team_array = Box::new(TeamMembersNode::new(TeamSide::Player));
        let hp_mapping = Box::new(CharacterHpValueNode::new(Box::new(ElementNode::<Character>::new())));
        let mapping_node = MappingNode::new(team_array, hp_mapping);
        
        // Find the maximum HP using MaxNode
        let max_node = MaxNode::new(Box::new(mapping_node));
        let mut eval_context = EvaluationContext::new(&battle_context, &mut rng);
        let result = max_node.evaluate(&mut eval_context).unwrap();
        
        // Should return the maximum HP value (80)
        assert_eq!(result, 80);
    }

}