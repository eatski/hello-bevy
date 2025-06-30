// Action system crate - node-based action resolution system

pub mod core;
pub mod nodes;
pub mod system;

// Re-export essential types only
pub use core::{Character, Team, TeamSide, ActionResolver, ActionType, RuleNode, NodeError, NodeResult};
pub use nodes::condition::{ConditionNode, ConditionCheckNode, RandomConditionNode, GreaterThanConditionNode};
pub use nodes::value::{ValueNode, ConstantValueNode};
pub use nodes::character::{BattleContext, CharacterNode, ActingCharacterNode, RandomCharacterNode, CharacterHpFromNode};
pub use nodes::action::{StrikeActionNode, HealActionNode};
pub use nodes::array::{CharacterArrayNode, AllCharactersNode, TeamMembersNode, CountArrayNode, RandomPickNode};
pub use system::ActionCalculationSystem;

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn test_all_characters_node() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let player = Character::new("Player".to_string(), 100, 50, 10);
        let enemy = Character::new("Enemy".to_string(), 80, 30, 15);
        
        let battle_context = BattleContext::new(&player, &player, &enemy);
        
        let all_chars_node = AllCharactersNode::new();
        let result = all_chars_node.evaluate(&battle_context, &mut rng).unwrap();
        
        assert_eq!(result.len(), 2);
        assert!(result.iter().any(|c| c.name == "Player"));
        assert!(result.iter().any(|c| c.name == "Enemy"));
    }

    #[test]
    fn test_team_members_node() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let player_team = Team::new("Player Team".to_string(), vec![
            Character::new("Player1".to_string(), 100, 50, 10),
            Character::new("Player2".to_string(), 0, 30, 8),  // Dead
        ]);
        let enemy_team = Team::new("Enemy Team".to_string(), vec![
            Character::new("Enemy1".to_string(), 80, 30, 15),
        ]);
        
        let acting_character = &player_team.members[0];
        let battle_context = BattleContext::new_team(
            acting_character,
            TeamSide::Player,
            &player_team,
            &enemy_team,
        );
        
        let team_node = TeamMembersNode::new(TeamSide::Player);
        let result = team_node.evaluate(&battle_context, &mut rng).unwrap();
        
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].name, "Player1");
        assert_eq!(result[1].name, "Player2");
    }

    #[test]
    fn test_count_array_node() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let player = Character::new("Player".to_string(), 100, 50, 10);
        let enemy = Character::new("Enemy".to_string(), 80, 30, 15);
        
        let battle_context = BattleContext::new(&player, &player, &enemy);
        
        let all_chars_node = Box::new(AllCharactersNode::new());
        let count_node = CountArrayNode::new(all_chars_node);
        let result = count_node.evaluate(&battle_context, &mut rng).unwrap();
        
        assert_eq!(result, 2);
    }


    #[test]
    fn test_random_pick_character_node() {
        use crate::nodes::array::RandomPickNode;
        
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let player_team = Team::new("Player Team".to_string(), vec![
            Character::new("Player1".to_string(), 100, 50, 10),
            Character::new("Player2".to_string(), 80, 30, 8),
        ]);
        let enemy_team = Team::new("Enemy Team".to_string(), vec![
            Character::new("Enemy1".to_string(), 60, 20, 12),
        ]);
        
        let acting_character = &player_team.members[0];
        let battle_context = BattleContext::new_team(
            acting_character,
            TeamSide::Player,
            &player_team,
            &enemy_team,
        );
        
        // Test random pick from character array
        let team_members_node = Box::new(TeamMembersNode::new(TeamSide::Player));
        let random_pick_node = RandomPickNode::new(team_members_node);
        let result = CharacterNode::evaluate(&random_pick_node, &battle_context, &mut rng).unwrap();
        
        // Should pick one of the alive team members
        assert!(result.name == "Player1" || result.name == "Player2");
    }


    #[test]
    fn test_random_pick_from_team() {
        use crate::nodes::array::RandomPickNode;
        
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let player_team = Team::new("Player Team".to_string(), vec![
            Character::new("Player1".to_string(), 0, 50, 10),  // Dead
            Character::new("Player2".to_string(), 0, 30, 8),   // Dead
        ]);
        let enemy_team = Team::new("Enemy Team".to_string(), vec![
            Character::new("Enemy1".to_string(), 60, 20, 12),
        ]);
        
        let acting_character = &player_team.members[0];
        let battle_context = BattleContext::new_team(
            acting_character,
            TeamSide::Player,
            &player_team,
            &enemy_team,
        );
        
        // Test random pick from team (including dead members)
        let team_members_node = Box::new(TeamMembersNode::new(TeamSide::Player));
        let random_pick_node = RandomPickNode::new(team_members_node);
        let result = CharacterNode::evaluate(&random_pick_node, &battle_context, &mut rng);
        
        // Should succeed since team has members (even if dead)
        assert!(result.is_ok());
        let character = result.unwrap();
        assert!(character.name == "Player1" || character.name == "Player2");
    }

}