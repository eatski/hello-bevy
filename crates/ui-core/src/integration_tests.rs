// Integration tests for UI core functionality

use crate::{GameState, CurrentRules, FlatTokenInput};
use battle::Character as GameCharacter;
use action_system::{ActionCalculationSystem, BattleContext, Team, TeamSide};
use rand::{SeedableRng, rngs::StdRng};

fn create_test_rng() -> StdRng {
    StdRng::seed_from_u64(12345)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_creation_workflow() {
        let mut game_state = GameState::new();
        let rules = CurrentRules::new();
        
        // Start in rule creation mode
        assert_eq!(game_state.is_rule_creation_mode(), true);
        assert_eq!(rules.has_valid_rules(), true); // Default rule exists
        
        // Use the default rule (already a simple strike rule)
        assert_eq!(rules.has_valid_rules(), true);
        assert_eq!(rules.non_empty_rule_count(), 1);
        
        // Switch to battle mode
        game_state.switch_to_battle();
        assert_eq!(game_state.is_battle_mode(), true);
        
        // Convert rules to battle system
        let rule_nodes = rules.convert_to_rule_nodes();
        assert_ne!(rule_nodes.len(), 0);
    }
    
    #[test]
    fn test_complex_rule_integration_with_action_system() {
        let mut rules = CurrentRules::new();
        
        // Create complex rule: Check → GreaterThan → Number(50) → HP → ActingCharacter → Heal → ActingCharacter
        rules.add_token_to_current_row(FlatTokenInput::Check);
        rules.add_token_to_current_row(FlatTokenInput::GreaterThan);
        rules.add_token_to_current_row(FlatTokenInput::Number(50));
        rules.add_token_to_current_row(FlatTokenInput::HP);
        rules.add_token_to_current_row(FlatTokenInput::ActingCharacter);
        rules.add_token_to_current_row(FlatTokenInput::Heal);
        rules.add_token_to_current_row(FlatTokenInput::ActingCharacter);
        
        // Add fallback rule: Strike → RandomPick → AllCharacters
        rules.select_next_row();
        rules.add_token_to_current_row(FlatTokenInput::Strike);
        rules.add_token_to_current_row(FlatTokenInput::RandomPick);
        rules.add_token_to_current_row(FlatTokenInput::AllCharacters);
        
        let rule_nodes = rules.convert_to_rule_nodes();
        assert_eq!(rule_nodes.len(), 2);
        
        // Test with action system
        let rng = create_test_rng();
        let mut action_system = ActionCalculationSystem::new(rule_nodes, rng);
        
        let player = GameCharacter::new(1, "Player".to_string(), 30, 50, 25); // Low HP
        let enemy = GameCharacter::new(2, "Enemy".to_string(), 100, 50, 25);
        let acting_character = GameCharacter::new(3, "Test".to_string(), 30, 50, 25); // Low HP
        
        let player_team = Team::new("Player Team".to_string(), vec![player.clone(), acting_character.clone()]);
        let enemy_team = Team::new("Enemy Team".to_string(), vec![enemy.clone()]);
        let battle_context = BattleContext::new(&acting_character, TeamSide::Player, &player_team, &enemy_team);
        
        let action = action_system.calculate_action(&battle_context);
        assert!(action.is_some(), "Should calculate an action for low HP character");
    }
    
    #[test]
    fn test_rule_editing_operations() {
        let mut rules = CurrentRules::new();
        
        // Clear default rule and test adding and removing tokens
        rules.clear_current_row();
        rules.add_token_to_current_row(FlatTokenInput::Check);
        rules.add_token_to_current_row(FlatTokenInput::Strike);
        rules.add_token_to_current_row(FlatTokenInput::ActingCharacter);
        assert_eq!(rules.rules[0].len(), 3);
        
        // Remove last token
        rules.remove_last_token_from_current_row();
        assert_eq!(rules.rules[0].len(), 2);
        assert_eq!(rules.rules[0][0], FlatTokenInput::Check);
        
        // Clear row
        rules.clear_current_row();
        assert_eq!(rules.is_current_row_empty(), true);
        
        // Test multi-row editing
        rules.add_token_to_current_row(FlatTokenInput::Strike);
        rules.add_token_to_current_row(FlatTokenInput::RandomPick);
        rules.add_token_to_current_row(FlatTokenInput::AllCharacters);
        rules.select_next_row();
        rules.add_token_to_current_row(FlatTokenInput::Heal);
        rules.add_token_to_current_row(FlatTokenInput::ActingCharacter);
        
        assert_eq!(rules.non_empty_rule_count(), 2);
        
        // Clear all
        rules.clear_all();
        assert_eq!(rules.non_empty_rule_count(), 0);
        assert_eq!(rules.selected_row, 0);
    }
    
    // Note: UI token display tests moved to bevy-ui crate
    
    #[test]
    fn test_game_state_battle_integration() {
        let mut game_state = GameState::new();
        let mut rules = CurrentRules::new();
        
        // Create rules in rule creation mode
        assert_eq!(game_state.is_rule_creation_mode(), true);
        
        rules.add_token_to_current_row(FlatTokenInput::Check);
        rules.add_token_to_current_row(FlatTokenInput::TrueOrFalse);
        rules.add_token_to_current_row(FlatTokenInput::Heal);
        rules.add_token_to_current_row(FlatTokenInput::ActingCharacter);
        rules.select_next_row();
        rules.add_token_to_current_row(FlatTokenInput::Strike);
        rules.add_token_to_current_row(FlatTokenInput::RandomPick);
        rules.add_token_to_current_row(FlatTokenInput::AllCharacters);
        
        assert_eq!(rules.non_empty_rule_count(), 2);
        
        // Switch to battle and verify rules work
        game_state.switch_to_battle();
        assert_eq!(game_state.is_battle_mode(), true);
        
        let rule_nodes = rules.convert_to_rule_nodes();
        assert_eq!(rule_nodes.len(), 2);
        
        // Create battle system
        let rng = create_test_rng();
        let mut action_system = ActionCalculationSystem::new(rule_nodes, rng);
        
        let player = GameCharacter::new(4, "Player".to_string(), 100, 50, 25);
        let enemy = GameCharacter::new(5, "Enemy".to_string(), 80, 30, 20);
        let acting_character = GameCharacter::new(6, "Test".to_string(), 50, 30, 25);
        
        let player_team = Team::new("Player Team".to_string(), vec![player.clone(), acting_character.clone()]);
        let enemy_team = Team::new("Enemy Team".to_string(), vec![enemy.clone()]);
        let battle_context = BattleContext::new(&acting_character, TeamSide::Player, &player_team, &enemy_team);
        let action = action_system.calculate_action(&battle_context);
        assert_eq!(action.is_some(), true);
    }
    
    #[test]
    fn test_rule_validation_patterns() {
        // Test various rule patterns that should be valid/invalid
        
        // Valid pattern: Strike only
        let mut rules1 = CurrentRules::new();
        rules1.add_token_to_current_row(FlatTokenInput::Strike);
        rules1.add_token_to_current_row(FlatTokenInput::RandomPick);
        rules1.add_token_to_current_row(FlatTokenInput::AllCharacters);
        assert_eq!(rules1.has_valid_rules(), true);
        let nodes1 = rules1.convert_to_rule_nodes();
        assert_ne!(nodes1.len(), 0);
        
        // Valid pattern: Heal only  
        let mut rules2 = CurrentRules::new();
        rules2.add_token_to_current_row(FlatTokenInput::Heal);
        rules2.add_token_to_current_row(FlatTokenInput::ActingCharacter);
        assert_eq!(rules2.has_valid_rules(), true);
        let nodes2 = rules2.convert_to_rule_nodes();
        assert!(!nodes2.is_empty());
        
        // Valid pattern: Complex conditional
        let mut rules3 = CurrentRules::new();
        rules3.add_token_to_current_row(FlatTokenInput::Check);
        rules3.add_token_to_current_row(FlatTokenInput::GreaterThan);
        rules3.add_token_to_current_row(FlatTokenInput::Number(50));
        rules3.add_token_to_current_row(FlatTokenInput::HP);
        rules3.add_token_to_current_row(FlatTokenInput::ActingCharacter);
        rules3.add_token_to_current_row(FlatTokenInput::Heal);
        rules3.add_token_to_current_row(FlatTokenInput::ActingCharacter);
        assert!(rules3.has_valid_rules());
        let nodes3 = rules3.convert_to_rule_nodes();
        assert!(!nodes3.is_empty());
        
        // Test cleared rules (empty after clearing)
        let mut rules4 = CurrentRules::new();
        rules4.clear_all();
        assert!(!rules4.has_valid_rules());
        let nodes4 = rules4.convert_to_rule_nodes();
        assert!(nodes4.is_empty());
    }
    
    #[test]
    fn test_row_navigation_edge_cases() {
        let mut rules = CurrentRules::new();
        
        // Test navigation at boundaries
        assert_eq!(rules.selected_row, 0);
        
        // Try to go below 0
        rules.select_previous_row();
        assert_eq!(rules.selected_row, 0);
        
        // Go to last row
        rules.select_row(4);
        assert_eq!(rules.selected_row, 4);
        
        // Try to go beyond last row
        rules.select_next_row();
        assert_eq!(rules.selected_row, 4);
        
        // Test out of bounds selection
        rules.select_row(10);
        assert_eq!(rules.selected_row, 4); // Should not change
    }
    
    #[test]
    fn test_rule_persistence_and_reconstruction() {
        // Test that rules can be reconstructed from data
        let original_rules = vec![
            vec![FlatTokenInput::Check, FlatTokenInput::TrueOrFalse, FlatTokenInput::Heal, FlatTokenInput::ActingCharacter],
            vec![FlatTokenInput::Strike, FlatTokenInput::ActingCharacter],
            vec![],
            vec![FlatTokenInput::Check, FlatTokenInput::GreaterThan, FlatTokenInput::Number(50), FlatTokenInput::HP, FlatTokenInput::ActingCharacter, FlatTokenInput::Strike, FlatTokenInput::ActingCharacter],
            vec![]
        ];
        
        let rules = CurrentRules::with_rules(original_rules.clone());
        assert_eq!(rules.rules, original_rules);
        assert_eq!(rules.non_empty_rule_count(), 3);
        
        // Test conversion
        let rule_nodes = rules.convert_to_rule_nodes();
        assert_eq!(rule_nodes.len(), 3);
        
        // Note: String formatting tests moved to bevy-ui crate
    }
    
    #[test]
    fn test_random_character_integration() {
        let mut rules = CurrentRules::new();
        
        // Create rule: Check → GreaterThan → Number(30) → HP → RandomPick → AllCharacters → Heal
        rules.add_token_to_current_row(FlatTokenInput::Check);
        rules.add_token_to_current_row(FlatTokenInput::GreaterThan);
        rules.add_token_to_current_row(FlatTokenInput::Number(30));
        rules.add_token_to_current_row(FlatTokenInput::HP);
        rules.add_token_to_current_row(FlatTokenInput::RandomPick);
        rules.add_token_to_current_row(FlatTokenInput::AllCharacters);
        rules.add_token_to_current_row(FlatTokenInput::Heal);
        rules.add_token_to_current_row(FlatTokenInput::ActingCharacter);
        
        let rule_nodes = rules.convert_to_rule_nodes();
        assert_eq!(rule_nodes.len(), 1, "RandomPick rule should convert successfully");
    }
}