// Integration tests for UI core functionality

use crate::{GameState, GameMode, CurrentRules, UITokenType};
use battle::{Battle, Character as GameCharacter};
use action_system::{ActionCalculationSystem, ActionType};
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
        let mut rules = CurrentRules::new();
        
        // Start in rule creation mode
        assert!(game_state.is_rule_creation_mode());
        assert!(!rules.has_valid_rules());
        
        // Create a simple strike rule
        rules.add_token_to_current_row(UITokenType::Strike);
        assert!(rules.has_valid_rules());
        assert_eq!(rules.non_empty_rule_count(), 1);
        
        // Switch to battle mode
        game_state.switch_to_battle();
        assert!(game_state.is_battle_mode());
        
        // Convert rules to battle system
        let rule_nodes = rules.convert_to_rule_nodes();
        assert!(!rule_nodes.is_empty());
    }
    
    #[test]
    fn test_complex_rule_integration_with_battle() {
        let mut rules = CurrentRules::new();
        
        // Create complex rule: Check → 50 → GreaterThan → HP → Heal
        rules.add_token_to_current_row(UITokenType::Check);
        rules.add_token_to_current_row(UITokenType::Number(50));
        rules.add_token_to_current_row(UITokenType::GreaterThan);
        rules.add_token_to_current_row(UITokenType::HP);
        rules.add_token_to_current_row(UITokenType::Heal);
        
        // Add fallback rule: Strike
        rules.select_next_row();
        rules.add_token_to_current_row(UITokenType::Strike);
        
        let rule_nodes = rules.convert_to_rule_nodes();
        assert_eq!(rule_nodes.len(), 2);
        
        // Test with actual battle
        let player = GameCharacter::new("Player".to_string(), 30, 50, 25); // Low HP
        let enemy = GameCharacter::new("Enemy".to_string(), 100, 50, 25);
        let rng = create_test_rng();
        let mut battle = Battle::new(
            player, 
            enemy, 
            rule_nodes,
            vec![Box::new(action_system::StrikeActionNode)],
            rng
        );
        
        let initial_player_hp = battle.player.hp;
        let initial_player_mp = battle.player.mp;
        
        battle.execute_player_action();
        
        // Low HP character should attempt to heal
        if battle.player.mp < initial_player_mp {
            assert!(battle.player.hp >= initial_player_hp, "Should heal when HP is low");
        }
    }
    
    #[test]
    fn test_rule_editing_operations() {
        let mut rules = CurrentRules::new();
        
        // Test adding and removing tokens
        rules.add_token_to_current_row(UITokenType::Check);
        rules.add_token_to_current_row(UITokenType::Strike);
        assert_eq!(rules.rules[0].len(), 2);
        
        // Remove last token
        rules.remove_last_token_from_current_row();
        assert_eq!(rules.rules[0].len(), 1);
        assert_eq!(rules.rules[0][0], UITokenType::Check);
        
        // Clear row
        rules.clear_current_row();
        assert!(rules.is_current_row_empty());
        
        // Test multi-row editing
        rules.add_token_to_current_row(UITokenType::Strike);
        rules.select_next_row();
        rules.add_token_to_current_row(UITokenType::Heal);
        
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
        assert!(game_state.is_rule_creation_mode());
        
        rules.add_token_to_current_row(UITokenType::TrueOrFalse);
        rules.add_token_to_current_row(UITokenType::Heal);
        rules.select_next_row();
        rules.add_token_to_current_row(UITokenType::Strike);
        
        assert_eq!(rules.non_empty_rule_count(), 2);
        
        // Switch to battle and verify rules work
        game_state.switch_to_battle();
        assert!(game_state.is_battle_mode());
        
        let rule_nodes = rules.convert_to_rule_nodes();
        assert_eq!(rule_nodes.len(), 2);
        
        // Create battle system
        let rng = create_test_rng();
        let mut action_system = ActionCalculationSystem::new(rule_nodes, rng);
        
        let character = GameCharacter::new("Test".to_string(), 50, 30, 25);
        let action = action_system.calculate_action(&character);
        assert!(action.is_some());
    }
    
    #[test]
    fn test_rule_validation_patterns() {
        // Test various rule patterns that should be valid/invalid
        
        // Valid pattern: Strike only
        let mut rules1 = CurrentRules::new();
        rules1.add_token_to_current_row(UITokenType::Strike);
        assert!(rules1.has_valid_rules());
        let nodes1 = rules1.convert_to_rule_nodes();
        assert!(!nodes1.is_empty());
        
        // Valid pattern: Heal only  
        let mut rules2 = CurrentRules::new();
        rules2.add_token_to_current_row(UITokenType::Heal);
        assert!(rules2.has_valid_rules());
        let nodes2 = rules2.convert_to_rule_nodes();
        assert!(!nodes2.is_empty());
        
        // Valid pattern: Complex conditional
        let mut rules3 = CurrentRules::new();
        rules3.add_token_to_current_row(UITokenType::Check);
        rules3.add_token_to_current_row(UITokenType::Number(50));
        rules3.add_token_to_current_row(UITokenType::GreaterThan);
        rules3.add_token_to_current_row(UITokenType::HP);
        rules3.add_token_to_current_row(UITokenType::Heal);
        assert!(rules3.has_valid_rules());
        let nodes3 = rules3.convert_to_rule_nodes();
        assert!(!nodes3.is_empty());
        
        // Invalid: Empty rules
        let rules4 = CurrentRules::new();
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
            vec![UITokenType::Check, UITokenType::TrueOrFalse, UITokenType::Heal],
            vec![UITokenType::Strike],
            vec![],
            vec![UITokenType::Check, UITokenType::Number(50), UITokenType::GreaterThan, UITokenType::HP, UITokenType::Strike],
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
}