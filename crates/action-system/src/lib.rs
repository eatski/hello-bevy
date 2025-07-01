// Action system crate - node-based action resolution system

pub mod core;
pub mod nodes;
pub mod system;

// Re-export essential types only
pub use core::{Character, Team, TeamSide, ActionResolver, Action, BattleState, RuleNode, NodeError, NodeResult};
pub use nodes::condition::{ConditionNode, ConditionCheckNode, RandomConditionNode, GreaterThanConditionNode};
pub use nodes::value::{ValueNode, ConstantValueNode};
pub use nodes::character::{BattleContext, CharacterNode, ActingCharacterNode, CharacterHpNode};
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
        
        let player = Character::new(1, "Player".to_string(), 100, 50, 10);
        let enemy = Character::new(2, "Enemy".to_string(), 80, 30, 15);
        
        let player_team = Team::new("Player Team".to_string(), vec![player.clone()]);
        let enemy_team = Team::new("Enemy Team".to_string(), vec![enemy.clone()]);
        let battle_context = BattleContext::new(&player, TeamSide::Player, &player_team, &enemy_team);
        
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
        let result = team_node.evaluate(&battle_context, &mut rng).unwrap();
        
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
        let result = count_node.evaluate(&battle_context, &mut rng).unwrap();
        
        assert_eq!(result, 2);
    }


    #[test]
    fn test_random_pick_character_node() {
        use crate::nodes::array::RandomPickNode;
        
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
        let random_pick_node = RandomPickNode::new(team_members_node);
        let result_id = CharacterNode::evaluate(&random_pick_node, &battle_context, &mut rng).unwrap();
        
        // Should pick one of the team members
        let player1_id = player_team.members[0].id;
        let player2_id = player_team.members[1].id;
        assert!(result_id == player1_id || result_id == player2_id);
    }


    #[test]
    fn test_random_pick_from_team() {
        use crate::nodes::array::RandomPickNode;
        
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
        let random_pick_node = RandomPickNode::new(team_members_node);
        let result = CharacterNode::evaluate(&random_pick_node, &battle_context, &mut rng);
        
        // Should succeed since team has members (even if dead)
        assert!(result.is_ok());
        let character_id = result.unwrap();
        let player1_id = player_team.members[0].id;
        let player2_id = player_team.members[1].id;
        assert!(character_id == player1_id || character_id == player2_id);
    }

}