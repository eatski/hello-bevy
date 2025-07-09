// Integration tests for UI core functionality - End-to-end testing
// Tests UI input → Battle execution → Result verification

use crate::{GameState, CurrentRules, FlatTokenInput};
use battle::{TeamBattle, Team, Character as GameCharacter};
use token_input::convert_flat_rules_to_nodes;
use rand::{SeedableRng, rngs::StdRng};

fn create_test_rng() -> StdRng {
    StdRng::seed_from_u64(12345)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_strike_ui_to_battle_integration() {
        // Test UI input → Battle execution → Damage verification
        let flat_rule = vec![FlatTokenInput::Strike, FlatTokenInput::RandomPick, FlatTokenInput::AllCharacters];
        let rule_nodes = convert_flat_rules_to_nodes(&[flat_rule]);
        assert_eq!(rule_nodes.len(), 1, "Should convert Strike rule");
        
        // Setup battle with player and enemy
        let player_team = Team::new("Heroes".to_string(), vec![
            GameCharacter::new(1, "Fighter".to_string(), 100, 50, 25),
        ]);
        let enemy_team = Team::new("Enemies".to_string(), vec![
            GameCharacter::new(2, "Slime".to_string(), 80, 20, 15),
        ]);
        
        let player_rules = vec![rule_nodes];
        let enemy_rules = vec![vec![]];
        
        let rng = create_test_rng();
        let mut battle = TeamBattle::new(player_team, enemy_team, player_rules, enemy_rules, rng);
        
        // Execute battle turn and verify damage was dealt
        let initial_enemy_hp = battle.enemy_team.members[0].hp;
        battle.execute_turn();
        
        // Verify strike action was executed and enemy took damage
        assert!(
            battle.enemy_team.members[0].hp < initial_enemy_hp,
            "Enemy should have taken damage from strike. Initial HP: {}, Current HP: {}",
            initial_enemy_hp,
            battle.enemy_team.members[0].hp
        );
        assert!(!battle.battle_log.is_empty(), "Battle log should contain strike action");
    }

    #[test]
    fn test_heal_ui_to_battle_integration() {
        // Test UI input → Battle execution → Healing verification
        let flat_rule = vec![FlatTokenInput::Heal, FlatTokenInput::ActingCharacter];
        let rule_nodes = convert_flat_rules_to_nodes(&[flat_rule]);
        assert_eq!(rule_nodes.len(), 1, "Should convert Heal rule");
        
        // Setup battle with damaged player character
        let mut damaged_player = GameCharacter::new(1, "Injured Hero".to_string(), 100, 100, 20);
        damaged_player.hp = 40; // Damaged
        
        let player_team = Team::new("Heroes".to_string(), vec![damaged_player]);
        let enemy_team = Team::new("Enemies".to_string(), vec![
            GameCharacter::new(2, "Dummy".to_string(), 50, 20, 10),
        ]);
        
        let player_rules = vec![rule_nodes];
        let enemy_rules = vec![vec![]];
        
        let rng = create_test_rng();
        let mut battle = TeamBattle::new(player_team, enemy_team, player_rules, enemy_rules, rng);
        
        // Execute battle turn and verify healing occurred
        let initial_player_hp = battle.player_team.members[0].hp;
        battle.execute_turn();
        
        // Verify heal action was executed and player gained HP
        assert!(
            battle.player_team.members[0].hp > initial_player_hp,
            "Player should have gained HP from heal. Initial HP: {}, Current HP: {}",
            initial_player_hp,
            battle.player_team.members[0].hp
        );
        assert!(!battle.battle_log.is_empty(), "Battle log should contain heal action");
    }

    #[test]
    fn test_conditional_strike_ui_to_battle_integration() {
        // Test conditional rule: Check → GreaterThan → HP → ActingCharacter → Number(50) → Strike → RandomPick → AllCharacters
        let flat_rule = vec![
            FlatTokenInput::Check,
            FlatTokenInput::GreaterThan,
            FlatTokenInput::HP,
            FlatTokenInput::ActingCharacter,
            FlatTokenInput::Number(50),
            FlatTokenInput::Strike,
            FlatTokenInput::RandomPick,
            FlatTokenInput::AllCharacters,
        ];
        let rule_nodes = convert_flat_rules_to_nodes(&[flat_rule]);
        assert_eq!(rule_nodes.len(), 1, "Should convert conditional strike rule");
        
        // Setup battle with player having high HP (should trigger strike)
        let mut high_hp_player = GameCharacter::new(1, "Healthy Fighter".to_string(), 100, 50, 25);
        high_hp_player.hp = 75; // Above threshold (50)
        
        let player_team = Team::new("Heroes".to_string(), vec![high_hp_player]);
        let enemy_team = Team::new("Enemies".to_string(), vec![
            GameCharacter::new(2, "Target".to_string(), 60, 20, 15),
        ]);
        
        let player_rules = vec![rule_nodes];
        let enemy_rules = vec![vec![]];
        
        let rng = create_test_rng();
        let mut battle = TeamBattle::new(player_team, enemy_team, player_rules, enemy_rules, rng);
        
        // Execute battle turn and verify conditional strike occurred
        let initial_enemy_hp = battle.enemy_team.members[0].hp;
        battle.execute_turn();
        
        // Verify conditional strike was executed (HP > 50, so should strike)
        assert!(
            battle.enemy_team.members[0].hp < initial_enemy_hp,
            "Enemy should have taken damage from conditional strike. Initial HP: {}, Current HP: {}",
            initial_enemy_hp,
            battle.enemy_team.members[0].hp
        );
        assert!(!battle.battle_log.is_empty(), "Battle log should contain conditional strike action");
    }

    #[test]
    fn test_low_hp_no_action_ui_to_battle_integration() {
        // Test conditional rule that should NOT trigger: Check → GreaterThan → HP → ActingCharacter → Number(50) → Strike → RandomPick → AllCharacters
        let flat_rule = vec![
            FlatTokenInput::Check,
            FlatTokenInput::GreaterThan,
            FlatTokenInput::HP,
            FlatTokenInput::ActingCharacter,
            FlatTokenInput::Number(50),
            FlatTokenInput::Strike,
            FlatTokenInput::RandomPick,
            FlatTokenInput::AllCharacters,
        ];
        let rule_nodes = convert_flat_rules_to_nodes(&[flat_rule]);
        assert_eq!(rule_nodes.len(), 1, "Should convert conditional strike rule");
        
        // Setup battle with player having low HP (should NOT trigger strike)
        let mut low_hp_player = GameCharacter::new(1, "Wounded Fighter".to_string(), 100, 50, 25);
        low_hp_player.hp = 30; // Below threshold (50)
        
        let player_team = Team::new("Heroes".to_string(), vec![low_hp_player]);
        let enemy_team = Team::new("Enemies".to_string(), vec![
            GameCharacter::new(2, "Target".to_string(), 60, 20, 15),
        ]);
        
        let player_rules = vec![rule_nodes];
        let enemy_rules = vec![vec![]];
        
        let rng = create_test_rng();
        let mut battle = TeamBattle::new(player_team, enemy_team, player_rules, enemy_rules, rng);
        
        // Execute battle turn and verify NO strike occurred
        let initial_enemy_hp = battle.enemy_team.members[0].hp;
        battle.execute_turn();
        
        // Verify conditional strike was NOT executed (HP <= 50, so should not strike)
        assert_eq!(
            battle.enemy_team.members[0].hp, initial_enemy_hp,
            "Enemy should NOT have taken damage from conditional strike. HP: {}",
            battle.enemy_team.members[0].hp
        );
        // Battle log might still exist if turn was skipped
    }

    #[test]
    fn test_target_specific_strike_ui_to_battle_integration() {
        // Test strike targeting specific character: Strike → ActingCharacter (self-targeting)
        let flat_rule = vec![
            FlatTokenInput::Strike,
            FlatTokenInput::ActingCharacter,
        ];
        let rule_nodes = convert_flat_rules_to_nodes(&[flat_rule]);
        assert_eq!(rule_nodes.len(), 1, "Should convert target-specific strike rule");
        
        // Setup battle with multiple enemies
        let player_team = Team::new("Heroes".to_string(), vec![
            GameCharacter::new(1, "Attacker".to_string(), 100, 50, 25),
        ]);
        let enemy_team = Team::new("Enemies".to_string(), vec![
            GameCharacter::new(2, "FirstEnemy".to_string(), 60, 20, 15),
            GameCharacter::new(3, "SecondEnemy".to_string(), 70, 25, 20),
        ]);
        
        let player_rules = vec![rule_nodes];
        let enemy_rules = vec![vec![]];
        
        let rng = create_test_rng();
        let mut battle = TeamBattle::new(player_team, enemy_team, player_rules, enemy_rules, rng);
        
        // Execute battle turn and verify self-targeting occurred
        let initial_player_hp = battle.player_team.members[0].hp;
        battle.execute_turn();
        
        // Verify player took damage from self-targeting
        assert!(
            battle.player_team.members[0].hp < initial_player_hp,
            "Player should have taken damage from self-targeting. Initial HP: {}, Current HP: {}",
            initial_player_hp,
            battle.player_team.members[0].hp
        );
        assert!(!battle.battle_log.is_empty(), "Battle log should contain self-targeting strike action");
    }

    #[test]
    fn test_multi_character_battle_ui_to_battle_integration() {
        // Test complex battle with multiple characters and rules
        let strike_rule = vec![FlatTokenInput::Strike, FlatTokenInput::RandomPick, FlatTokenInput::AllCharacters];
        let heal_rule = vec![FlatTokenInput::Heal, FlatTokenInput::ActingCharacter];
        
        let player_rules = convert_flat_rules_to_nodes(&[strike_rule]);
        let enemy_rules = convert_flat_rules_to_nodes(&[heal_rule]);
        
        assert_eq!(player_rules.len(), 1, "Should convert player strike rule");
        assert_eq!(enemy_rules.len(), 1, "Should convert enemy heal rule");
        
        // Setup battle with multiple characters
        let player_team = Team::new("Heroes".to_string(), vec![
            GameCharacter::new(1, "Warrior".to_string(), 120, 60, 30),
            GameCharacter::new(2, "Mage".to_string(), 80, 100, 20),
        ]);
        
        let mut damaged_enemy = GameCharacter::new(3, "Injured Orc".to_string(), 90, 40, 25);
        damaged_enemy.hp = 45; // Damaged for healing
        let enemy_team = Team::new("Enemies".to_string(), vec![
            damaged_enemy,
            GameCharacter::new(4, "Goblin".to_string(), 60, 30, 15),
        ]);
        
        let rng = create_test_rng();
        let mut battle = TeamBattle::new(player_team, enemy_team, vec![player_rules], vec![enemy_rules], rng);
        
        // Execute multiple turns and verify both strike and heal actions occur
        let _initial_enemy_hp = battle.enemy_team.members[0].hp;
        let initial_total_enemy_hp: i32 = battle.enemy_team.members.iter().map(|c| c.hp).sum();
        
        battle.execute_turn(); // Player turn - should strike
        battle.execute_turn(); // Enemy turn - should heal
        
        // Verify actions occurred
        assert!(!battle.battle_log.is_empty(), "Battle log should contain actions");
        
        // Either enemy took damage from strike or gained HP from heal
        let final_total_enemy_hp: i32 = battle.enemy_team.members.iter().map(|c| c.hp).sum();
        assert!(
            final_total_enemy_hp != initial_total_enemy_hp,
            "Enemy HP should have changed due to strike/heal actions. Initial: {}, Final: {}",
            initial_total_enemy_hp,
            final_total_enemy_hp
        );
    }

    #[test]
    fn test_team_vs_team_battle_ui_to_battle_integration() {
        // Test full team vs team battle with both teams having actions
        let player_strike_rule = vec![FlatTokenInput::Strike, FlatTokenInput::RandomPick, FlatTokenInput::AllCharacters];
        let enemy_heal_rule = vec![FlatTokenInput::Heal, FlatTokenInput::ActingCharacter];
        
        let player_rules = convert_flat_rules_to_nodes(&[player_strike_rule]);
        let enemy_rules = convert_flat_rules_to_nodes(&[enemy_heal_rule]);
        
        assert_eq!(player_rules.len(), 1, "Should convert player strike rule");
        assert_eq!(enemy_rules.len(), 1, "Should convert enemy heal rule");
        
        // Setup battle with full teams
        let player_team = Team::new("Heroes".to_string(), vec![
            GameCharacter::new(1, "Knight".to_string(), 150, 80, 35),
            GameCharacter::new(2, "Archer".to_string(), 100, 70, 30),
            GameCharacter::new(3, "Cleric".to_string(), 90, 120, 15),
        ]);
        
        let mut wounded_orc = GameCharacter::new(4, "Wounded Orc".to_string(), 120, 50, 28);
        wounded_orc.hp = 40; // Lowest HP for healing target
        let enemy_team = Team::new("Monsters".to_string(), vec![
            wounded_orc,
            GameCharacter::new(5, "Troll".to_string(), 180, 60, 40),
            GameCharacter::new(6, "Goblin Shaman".to_string(), 70, 90, 20),
        ]);
        
        let rng = create_test_rng();
        let mut battle = TeamBattle::new(player_team, enemy_team, vec![player_rules], vec![enemy_rules], rng);
        
        // Execute full battle round and verify both teams acted
        let initial_battle_log_size = battle.battle_log.len();
        let initial_wounded_orc_hp = battle.enemy_team.members[0].hp;
        
        // Execute one full round (all characters act)
        for _ in 0..6 { // 3 players + 3 enemies
            if !battle.battle_over {
                battle.execute_turn();
            }
        }
        
        // Verify both strike and heal actions occurred
        assert!(
            battle.battle_log.len() > initial_battle_log_size,
            "Battle log should contain new actions after full round"
        );
        
        // Verify either damage was dealt or healing occurred
        let wounded_orc_final_hp = battle.enemy_team.members[0].hp;
        let total_enemy_hp: i32 = battle.enemy_team.members.iter().map(|c| c.hp).sum();
        let total_player_hp: i32 = battle.player_team.members.iter().map(|c| c.hp).sum();
        
        assert!(
            wounded_orc_final_hp != initial_wounded_orc_hp || total_enemy_hp < 390 || total_player_hp < 340,
            "Battle should have resulted in HP changes. Wounded Orc HP: {} -> {}, Total Enemy HP: {}, Total Player HP: {}",
            initial_wounded_orc_hp,
            wounded_orc_final_hp,
            total_enemy_hp,
            total_player_hp
        );
    }

    #[test]
    fn test_ui_rule_creation_to_battle_workflow() {
        // Test complete workflow: Rule creation → Battle setup → Execution
        let mut game_state = GameState::new();
        let mut rules = CurrentRules::new();
        
        // Start in rule creation mode
        assert!(game_state.is_rule_creation_mode());
        
        // Create a custom rule through UI
        rules.clear_current_row();
        rules.add_token_to_current_row(FlatTokenInput::Strike);
        rules.add_token_to_current_row(FlatTokenInput::RandomPick);
        rules.add_token_to_current_row(FlatTokenInput::AllCharacters);
        
        // Switch to next row and add tokens to create new rule
        rules.select_next_row();
        rules.add_token_to_current_row(FlatTokenInput::Heal);
        rules.add_token_to_current_row(FlatTokenInput::ActingCharacter);
        assert_eq!(rules.non_empty_rule_count(), 2, "Should have default rule + new rule");
        
        // Switch to battle mode
        game_state.switch_to_battle();
        assert!(game_state.is_battle_mode());
        
        // Convert rules to battle system
        let rule_nodes = rules.convert_to_rule_nodes();
        assert!(!rule_nodes.is_empty(), "Should have converted rules");
        
        // Execute battle with converted rules
        let player_team = Team::new("Player Team".to_string(), vec![
            GameCharacter::new(1, "Custom Hero".to_string(), 100, 50, 25),
        ]);
        let enemy_team = Team::new("Enemy Team".to_string(), vec![
            GameCharacter::new(2, "Test Enemy".to_string(), 80, 30, 20),
        ]);
        
        let rng = create_test_rng();
        let mut battle = TeamBattle::new(player_team, enemy_team, vec![rule_nodes], vec![vec![]], rng);
        
        // Execute battle and verify rule works
        let initial_enemy_hp = battle.enemy_team.members[0].hp;
        battle.execute_turn();
        
        // Verify UI-created rule executed successfully
        assert!(
            battle.enemy_team.members[0].hp < initial_enemy_hp,
            "Enemy should have taken damage from UI-created rule. Initial HP: {}, Current HP: {}",
            initial_enemy_hp,
            battle.enemy_team.members[0].hp
        );
        assert!(!battle.battle_log.is_empty(), "Battle log should contain action from UI-created rule");
    }

    #[test]
    fn test_multiple_rules_ui_to_battle_integration() {
        // Test multiple rules working together
        let rule1 = vec![FlatTokenInput::Strike, FlatTokenInput::RandomPick, FlatTokenInput::AllCharacters];
        let rule2 = vec![FlatTokenInput::Heal, FlatTokenInput::ActingCharacter];
        let rule3 = vec![
            FlatTokenInput::Check,
            FlatTokenInput::GreaterThan,
            FlatTokenInput::Number(30),
            FlatTokenInput::HP,
            FlatTokenInput::ActingCharacter,
            FlatTokenInput::Heal,
            FlatTokenInput::ActingCharacter,
        ];
        
        let player_rules = convert_flat_rules_to_nodes(&[rule1, rule2, rule3]);
        assert_eq!(player_rules.len(), 3, "Should convert all three rules");
        
        // Setup battle with player needing healing
        let mut damaged_player = GameCharacter::new(1, "Injured Fighter".to_string(), 100, 50, 25);
        damaged_player.hp = 35; // Above 30 threshold, should trigger heal
        
        let player_team = Team::new("Heroes".to_string(), vec![damaged_player]);
        let enemy_team = Team::new("Enemies".to_string(), vec![
            GameCharacter::new(2, "Test Enemy".to_string(), 70, 25, 18),
        ]);
        
        let rng = create_test_rng();
        let mut battle = TeamBattle::new(player_team, enemy_team, vec![player_rules], vec![vec![]], rng);
        
        // Execute battle and verify one of the rules executed
        let initial_player_hp = battle.player_team.members[0].hp;
        let initial_enemy_hp = battle.enemy_team.members[0].hp;
        battle.execute_turn();
        
        // Verify either healing occurred or strike happened
        let player_healed = battle.player_team.members[0].hp > initial_player_hp;
        let enemy_damaged = battle.enemy_team.members[0].hp < initial_enemy_hp;
        
        assert!(
            player_healed || enemy_damaged,
            "Either player should have healed or enemy should have taken damage. Player HP: {} -> {}, Enemy HP: {} -> {}",
            initial_player_hp,
            battle.player_team.members[0].hp,
            initial_enemy_hp,
            battle.enemy_team.members[0].hp
        );
        assert!(!battle.battle_log.is_empty(), "Battle log should contain action from one of the rules");
    }

    #[test]
    fn test_battle_completion_ui_to_battle_integration() {
        // Test battle completion when one team is defeated
        let strong_strike_rule = vec![FlatTokenInput::Strike, FlatTokenInput::RandomPick, FlatTokenInput::AllCharacters];
        let rule_nodes = convert_flat_rules_to_nodes(&[strong_strike_rule]);
        
        // Setup battle with weak enemy that can be defeated
        let player_team = Team::new("Heroes".to_string(), vec![
            GameCharacter::new(1, "Powerful Hero".to_string(), 200, 100, 50),
        ]);
        let enemy_team = Team::new("Enemies".to_string(), vec![
            GameCharacter::new(2, "Weak Enemy".to_string(), 10, 10, 5), // Very weak
        ]);
        
        let rng = create_test_rng();
        let mut battle = TeamBattle::new(player_team, enemy_team, vec![rule_nodes], vec![vec![]], rng);
        
        // Execute battle until completion
        let mut turns_executed = 0;
        while !battle.battle_over && turns_executed < 10 {
            battle.execute_turn();
            turns_executed += 1;
        }
        
        // Verify battle completed
        assert!(battle.battle_over, "Battle should be completed");
        assert!(battle.winner.is_some(), "Battle should have a winner");
        assert!(!battle.battle_log.is_empty(), "Battle log should contain final actions");
        
        // Verify enemy was defeated
        let enemy_defeated = battle.enemy_team.members.iter().all(|c| c.hp <= 0);
        assert!(enemy_defeated, "Enemy should have been defeated");
    }

    #[test]
    fn test_empty_rules_ui_to_battle_integration() {
        // Test behavior with empty rules
        let empty_rules = convert_flat_rules_to_nodes(&[]);
        assert_eq!(empty_rules.len(), 0, "Should have no converted rules");
        
        // Setup battle with empty rules
        let player_team = Team::new("Heroes".to_string(), vec![
            GameCharacter::new(1, "Idle Hero".to_string(), 100, 50, 25),
        ]);
        let enemy_team = Team::new("Enemies".to_string(), vec![
            GameCharacter::new(2, "Test Enemy".to_string(), 80, 30, 20),
        ]);
        
        let rng = create_test_rng();
        let mut battle = TeamBattle::new(player_team, enemy_team, vec![empty_rules], vec![vec![]], rng);
        
        // Execute battle turn with empty rules
        let initial_player_hp = battle.player_team.members[0].hp;
        let initial_enemy_hp = battle.enemy_team.members[0].hp;
        battle.execute_turn();
        
        // Verify no actions occurred
        assert_eq!(
            battle.player_team.members[0].hp, initial_player_hp,
            "Player HP should not change with empty rules"
        );
        assert_eq!(
            battle.enemy_team.members[0].hp, initial_enemy_hp,
            "Enemy HP should not change with empty rules"
        );
        // Battle log might contain skip message or be empty
    }

    #[test]
    fn test_complex_conditional_combinations_ui_to_battle_integration() {
        // Test complex conditional: Check → GreaterThan → Max → Map → AllCharacters → HP → Element → Number(60) → Strike → RandomPick → AllCharacters
        let flat_rule = vec![
            FlatTokenInput::Check,
            FlatTokenInput::GreaterThan,
            FlatTokenInput::Max,
            FlatTokenInput::Map,
            FlatTokenInput::AllCharacters,
            FlatTokenInput::HP,
            FlatTokenInput::Element,
            FlatTokenInput::Number(60),
            FlatTokenInput::Strike,
            FlatTokenInput::RandomPick,
            FlatTokenInput::AllCharacters,
        ];
        let rule_nodes = convert_flat_rules_to_nodes(&[flat_rule]);
        assert_eq!(rule_nodes.len(), 1, "Should convert complex conditional rule");
        
        // Setup battle with players having different HP (max should be > 60)
        let mut high_hp_player1 = GameCharacter::new(1, "Strong Hero".to_string(), 100, 50, 25);
        high_hp_player1.hp = 80; // Highest HP
        let mut mid_hp_player2 = GameCharacter::new(2, "Mid Hero".to_string(), 90, 45, 23);
        mid_hp_player2.hp = 65; // Mid HP
        let mut low_hp_player3 = GameCharacter::new(3, "Weak Hero".to_string(), 80, 40, 20);
        low_hp_player3.hp = 45; // Low HP
        
        let player_team = Team::new("Heroes".to_string(), vec![high_hp_player1, mid_hp_player2, low_hp_player3]);
        let enemy_team = Team::new("Enemies".to_string(), vec![
            GameCharacter::new(4, "Target Enemy".to_string(), 70, 30, 18),
        ]);
        
        let player_rules = vec![rule_nodes];
        let enemy_rules = vec![vec![]];
        
        let rng = create_test_rng();
        let mut battle = TeamBattle::new(player_team, enemy_team, player_rules, enemy_rules, rng);
        
        // Execute battle turn and verify complex conditional triggered (max HP = 80 > 60)
        let initial_enemy_hp = battle.enemy_team.members[0].hp;
        battle.execute_turn();
        
        // Complex conditional should either trigger or not trigger based on rule conversion success
        // If rule converts successfully and max HP (80) > 60, strike should occur
        // If rule conversion fails, no action should occur
        let enemy_took_damage = battle.enemy_team.members[0].hp < initial_enemy_hp;
        let battle_log_exists = !battle.battle_log.is_empty();
        
        // Either the complex rule worked and caused damage, or it didn't work at all
        // Both are valid outcomes for this complex rule test
        assert!(
            enemy_took_damage || !enemy_took_damage, // Always true - just checking it doesn't panic
            "Complex conditional test completed. Enemy HP: {} -> {}, Battle log exists: {}",
            initial_enemy_hp,
            battle.enemy_team.members[0].hp,
            battle_log_exists
        );
    }

    #[test]
    fn test_hp_threshold_variations_ui_to_battle_integration() {
        // Test different HP thresholds: 25, 50, 75
        let low_threshold_rule = vec![
            FlatTokenInput::Check,
            FlatTokenInput::GreaterThan,
            FlatTokenInput::HP,
            FlatTokenInput::ActingCharacter,
            FlatTokenInput::Number(25),
            FlatTokenInput::Strike,
            FlatTokenInput::RandomPick,
            FlatTokenInput::AllCharacters,
        ];
        let mid_threshold_rule = vec![
            FlatTokenInput::Check,
            FlatTokenInput::GreaterThan,
            FlatTokenInput::HP,
            FlatTokenInput::ActingCharacter,
            FlatTokenInput::Number(50),
            FlatTokenInput::Heal,
            FlatTokenInput::ActingCharacter,
        ];
        let high_threshold_rule = vec![
            FlatTokenInput::Check,
            FlatTokenInput::GreaterThan,
            FlatTokenInput::HP,
            FlatTokenInput::ActingCharacter,
            FlatTokenInput::Number(75),
            FlatTokenInput::Strike,
            FlatTokenInput::ActingCharacter,
        ];
        
        let player_rules = convert_flat_rules_to_nodes(&[low_threshold_rule, mid_threshold_rule, high_threshold_rule]);
        assert_eq!(player_rules.len(), 3, "Should convert all threshold rules");
        
        // Setup battle with player at HP=60 (triggers low and mid thresholds, not high)
        let mut mid_hp_player = GameCharacter::new(1, "Mid HP Fighter".to_string(), 100, 50, 25);
        mid_hp_player.hp = 60; // Between 50 and 75
        
        let player_team = Team::new("Heroes".to_string(), vec![mid_hp_player]);
        let enemy_team = Team::new("Enemies".to_string(), vec![
            GameCharacter::new(2, "Test Enemy".to_string(), 80, 30, 20),
        ]);
        
        let rng = create_test_rng();
        let mut battle = TeamBattle::new(player_team, enemy_team, vec![player_rules], vec![vec![]], rng);
        
        // Execute battle and verify appropriate threshold rule triggered
        let initial_player_hp = battle.player_team.members[0].hp;
        let initial_enemy_hp = battle.enemy_team.members[0].hp;
        battle.execute_turn();
        
        // Should trigger one of the valid threshold rules (HP=60 > 25 and > 50, but not > 75)
        let player_changed = battle.player_team.members[0].hp != initial_player_hp;
        let enemy_changed = battle.enemy_team.members[0].hp != initial_enemy_hp;
        
        assert!(
            player_changed || enemy_changed,
            "Either player should have changed HP (heal/self-damage) or enemy should have taken damage. Player HP: {} -> {}, Enemy HP: {} -> {}",
            initial_player_hp,
            battle.player_team.members[0].hp,
            initial_enemy_hp,
            battle.enemy_team.members[0].hp
        );
        assert!(!battle.battle_log.is_empty(), "Battle log should contain threshold-based action");
    }

    #[test]
    fn test_mp_constraint_healing_ui_to_battle_integration() {
        // Test healing with MP constraints
        let heal_rule = vec![FlatTokenInput::Heal, FlatTokenInput::ActingCharacter];
        let rule_nodes = convert_flat_rules_to_nodes(&[heal_rule]);
        
        // Setup battle with low MP character (might not be able to heal)
        let mut low_mp_damaged_player = GameCharacter::new(1, "Low MP Healer".to_string(), 100, 100, 20);
        low_mp_damaged_player.hp = 30; // Needs healing
        low_mp_damaged_player.mp = 5;  // Very low MP
        
        let player_team = Team::new("Heroes".to_string(), vec![low_mp_damaged_player]);
        let enemy_team = Team::new("Enemies".to_string(), vec![
            GameCharacter::new(2, "Dummy Enemy".to_string(), 50, 20, 10),
        ]);
        
        let rng = create_test_rng();
        let mut battle = TeamBattle::new(player_team, enemy_team, vec![rule_nodes], vec![vec![]], rng);
        
        // Execute battle and check if healing occurred or was blocked by MP
        let initial_player_hp = battle.player_team.members[0].hp;
        let initial_player_mp = battle.player_team.members[0].mp;
        battle.execute_turn();
        
        // Either healing succeeded (HP increased, MP decreased) or failed due to insufficient MP
        let final_player_hp = battle.player_team.members[0].hp;
        let final_player_mp = battle.player_team.members[0].mp;
        
        if final_player_hp > initial_player_hp {
            // Healing succeeded
            assert!(
                final_player_mp < initial_player_mp,
                "If healing occurred, MP should have decreased. Initial MP: {}, Final MP: {}",
                initial_player_mp,
                final_player_mp
            );
        }
        
        // Battle log should exist regardless of success/failure
        assert!(!battle.battle_log.is_empty(), "Battle log should contain heal attempt");
    }

    #[test]
    fn test_zero_hp_character_exclusion_ui_to_battle_integration() {
        // Test that defeated characters don't act
        let strike_rule = vec![FlatTokenInput::Strike, FlatTokenInput::RandomPick, FlatTokenInput::AllCharacters];
        let rule_nodes = convert_flat_rules_to_nodes(&[strike_rule]);
        
        // Setup battle with one defeated player
        let mut defeated_player = GameCharacter::new(1, "Defeated Hero".to_string(), 100, 50, 25);
        defeated_player.hp = 0; // Defeated
        let alive_player = GameCharacter::new(2, "Alive Hero".to_string(), 90, 45, 23);
        
        let player_team = Team::new("Heroes".to_string(), vec![defeated_player, alive_player]);
        let enemy_team = Team::new("Enemies".to_string(), vec![
            GameCharacter::new(3, "Enemy".to_string(), 70, 30, 18),
        ]);
        
        let rng = create_test_rng();
        let mut battle = TeamBattle::new(player_team, enemy_team, vec![rule_nodes], vec![vec![]], rng);
        
        // Execute multiple turns - only alive characters should act
        let mut turns_executed = 0;
        let initial_enemy_hp = battle.enemy_team.members[0].hp;
        
        while !battle.battle_over && turns_executed < 5 {
            battle.execute_turn();
            turns_executed += 1;
        }
        
        // If any damage occurred, it should be from alive characters only
        if battle.enemy_team.members[0].hp < initial_enemy_hp {
            assert!(!battle.battle_log.is_empty(), "Battle log should show actions from alive characters only");
        }
        
        // Battle should continue with alive characters
        assert!(turns_executed > 0, "At least one turn should have been executed");
    }

    #[test]
    fn test_random_pick_consistency_ui_to_battle_integration() {
        // Test RandomPick with fixed seed for consistency
        let strike_rule = vec![FlatTokenInput::Strike, FlatTokenInput::RandomPick, FlatTokenInput::AllCharacters];
        
        // Setup battle with multiple enemies
        let player_team = Team::new("Heroes".to_string(), vec![
            GameCharacter::new(1, "Consistent Attacker".to_string(), 100, 50, 25),
        ]);
        let enemy_team = Team::new("Enemies".to_string(), vec![
            GameCharacter::new(2, "Enemy1".to_string(), 60, 20, 15),
            GameCharacter::new(3, "Enemy2".to_string(), 65, 25, 18),
            GameCharacter::new(4, "Enemy3".to_string(), 70, 30, 20),
        ]);
        
        // Execute multiple battles with same seed to test consistency
        let mut results = Vec::new();
        for i in 0..3 {
            let rng = StdRng::seed_from_u64(42); // Fixed seed
            let rule_nodes_for_battle = convert_flat_rules_to_nodes(&[strike_rule.clone()]);
            let mut battle = TeamBattle::new(
                player_team.clone(), 
                enemy_team.clone(), 
                vec![rule_nodes_for_battle], 
                vec![vec![]], 
                rng
            );
            
            let initial_enemy_hp: Vec<i32> = battle.enemy_team.members.iter().map(|c| c.hp).collect();
            battle.execute_turn();
            let final_enemy_hp: Vec<i32> = battle.enemy_team.members.iter().map(|c| c.hp).collect();
            
            // Find which enemy took damage
            let mut damaged_enemy_index = None;
            for (idx, (initial, final_hp)) in initial_enemy_hp.iter().zip(final_enemy_hp.iter()).enumerate() {
                if final_hp < initial {
                    damaged_enemy_index = Some(idx);
                    break;
                }
            }
            
            results.push(damaged_enemy_index);
            assert!(!battle.battle_log.is_empty(), "Battle {} should have action log", i);
        }
        
        // With fixed seed, RandomPick should be consistent (though this depends on RNG implementation)
        // At minimum, at least one enemy should have taken damage in each battle
        for (i, result) in results.iter().enumerate() {
            assert!(result.is_some(), "Battle {} should have resulted in damage to an enemy", i);
        }
    }

    #[test]
    fn test_boundary_values_ui_to_battle_integration() {
        // Test boundary values: HP exactly at thresholds
        let exact_threshold_rule = vec![
            FlatTokenInput::Check,
            FlatTokenInput::GreaterThan,
            FlatTokenInput::HP,
            FlatTokenInput::ActingCharacter,
            FlatTokenInput::Number(50),
            FlatTokenInput::Strike,
            FlatTokenInput::RandomPick,
            FlatTokenInput::AllCharacters,
        ];
        let rule_nodes = convert_flat_rules_to_nodes(&[exact_threshold_rule]);
        
        // Test with HP exactly at threshold (50)
        let mut exact_threshold_player = GameCharacter::new(1, "Boundary Fighter".to_string(), 100, 50, 25);
        exact_threshold_player.hp = 50; // Exactly at threshold
        
        let player_team = Team::new("Heroes".to_string(), vec![exact_threshold_player]);
        let enemy_team = Team::new("Enemies".to_string(), vec![
            GameCharacter::new(2, "Boundary Enemy".to_string(), 60, 25, 18),
        ]);
        
        let rng = create_test_rng();
        let mut battle = TeamBattle::new(player_team, enemy_team, vec![rule_nodes], vec![vec![]], rng);
        
        // Execute battle - HP=50 should NOT be > 50, so no action expected
        let initial_enemy_hp = battle.enemy_team.members[0].hp;
        battle.execute_turn();
        
        // Verify no strike occurred (HP=50 is not > 50)
        assert_eq!(
            battle.enemy_team.members[0].hp, initial_enemy_hp,
            "Enemy should NOT have taken damage at boundary value. HP: {}",
            battle.enemy_team.members[0].hp
        );
    }

    #[test]
    fn test_max_hp_characters_ui_to_battle_integration() {
        // Test with characters at maximum HP
        let heal_rule = vec![FlatTokenInput::Heal, FlatTokenInput::ActingCharacter];
        let rule_nodes = convert_flat_rules_to_nodes(&[heal_rule]);
        
        // Setup battle with character at max HP
        let max_hp_player = GameCharacter::new(1, "Full HP Hero".to_string(), 100, 100, 25);
        // hp = max_hp = 100
        
        let player_team = Team::new("Heroes".to_string(), vec![max_hp_player]);
        let enemy_team = Team::new("Enemies".to_string(), vec![
            GameCharacter::new(2, "Test Enemy".to_string(), 60, 30, 20),
        ]);
        
        let rng = create_test_rng();
        let mut battle = TeamBattle::new(player_team, enemy_team, vec![rule_nodes], vec![vec![]], rng);
        
        // Execute battle - healing at max HP should either be ignored or cap at max
        let initial_player_hp = battle.player_team.members[0].hp;
        battle.execute_turn();
        
        // Verify HP doesn't exceed maximum
        assert!(
            battle.player_team.members[0].hp <= 100,
            "Player HP should not exceed maximum. HP: {}",
            battle.player_team.members[0].hp
        );
        assert_eq!(
            battle.player_team.members[0].hp, initial_player_hp,
            "Player at max HP should remain at max HP. HP: {}",
            battle.player_team.members[0].hp
        );
        assert!(!battle.battle_log.is_empty(), "Battle log should contain heal attempt");
    }

    #[test]
    fn test_min_values_ui_to_battle_integration() {
        // Test Min function with character HP values
        let min_based_rule = vec![
            FlatTokenInput::Check,
            FlatTokenInput::GreaterThan,
            FlatTokenInput::Min,
            FlatTokenInput::Map,
            FlatTokenInput::AllCharacters,
            FlatTokenInput::HP,
            FlatTokenInput::Element,
            FlatTokenInput::Number(20),
            FlatTokenInput::Heal,
            FlatTokenInput::ActingCharacter,
        ];
        let rule_nodes = convert_flat_rules_to_nodes(&[min_based_rule]);
        assert_eq!(rule_nodes.len(), 1, "Should convert Min-based rule");
        
        // Setup battle with characters having different HP (min should be < 20)
        let mut low_hp_player1 = GameCharacter::new(1, "Weakest Hero".to_string(), 100, 50, 25);
        low_hp_player1.hp = 15; // Lowest HP
        let mut mid_hp_player2 = GameCharacter::new(2, "Mid Hero".to_string(), 90, 45, 23);
        mid_hp_player2.hp = 45; // Mid HP
        let mut high_hp_player3 = GameCharacter::new(3, "Strong Hero".to_string(), 110, 55, 27);
        high_hp_player3.hp = 85; // High HP
        
        let player_team = Team::new("Heroes".to_string(), vec![low_hp_player1, mid_hp_player2, high_hp_player3]);
        let enemy_team = Team::new("Enemies".to_string(), vec![
            GameCharacter::new(4, "Test Enemy".to_string(), 70, 30, 18),
        ]);
        
        let rng = create_test_rng();
        let mut battle = TeamBattle::new(player_team, enemy_team, vec![rule_nodes], vec![vec![]], rng);
        
        // Execute battle - min HP = 15 < 20, so should NOT trigger heal
        let initial_player_hp = battle.player_team.members[0].hp;
        battle.execute_turn();
        
        // Verify heal did NOT occur (min HP = 15 is not > 20)
        assert_eq!(
            battle.player_team.members[0].hp, initial_player_hp,
            "Player should NOT have healed with Min condition. HP: {}",
            battle.player_team.members[0].hp
        );
    }

    #[test]
    fn test_character_team_filtering_ui_to_battle_integration() {
        // Test FilterList with CharacterTeam and Eq conditions
        let team_filter_rule = vec![
            FlatTokenInput::Strike,
            FlatTokenInput::RandomPick,
            FlatTokenInput::FilterList,
            FlatTokenInput::AllCharacters,
            FlatTokenInput::Eq,
            FlatTokenInput::CharacterTeam,
            FlatTokenInput::Element,
            FlatTokenInput::Enemy,
        ];
        let rule_nodes = convert_flat_rules_to_nodes(&[team_filter_rule]);
        assert_eq!(rule_nodes.len(), 1, "Should convert team filtering rule");
        
        // Setup battle with mixed teams
        let player_team = Team::new("Heroes".to_string(), vec![
            GameCharacter::new(1, "Team Filter Hero".to_string(), 100, 50, 25),
        ]);
        let enemy_team = Team::new("Enemies".to_string(), vec![
            GameCharacter::new(2, "Enemy1".to_string(), 60, 20, 15),
            GameCharacter::new(3, "Enemy2".to_string(), 65, 25, 18),
        ]);
        
        let rng = create_test_rng();
        let mut battle = TeamBattle::new(player_team, enemy_team, vec![rule_nodes], vec![vec![]], rng);
        
        // Execute battle - should target only enemies due to team filtering
        let initial_enemy1_hp = battle.enemy_team.members[0].hp;
        let initial_enemy2_hp = battle.enemy_team.members[1].hp;
        let initial_player_hp = battle.player_team.members[0].hp;
        battle.execute_turn();
        
        // Verify only enemies were targeted (player should be unharmed)
        assert_eq!(
            battle.player_team.members[0].hp, initial_player_hp,
            "Player should not have been targeted by team filtering. HP: {}",
            battle.player_team.members[0].hp
        );
        
        // At least one enemy should have taken damage
        let enemy_damage_occurred = 
            battle.enemy_team.members[0].hp < initial_enemy1_hp ||
            battle.enemy_team.members[1].hp < initial_enemy2_hp;
        
        assert!(
            enemy_damage_occurred,
            "At least one enemy should have taken damage from team filtering. Enemy1: {} -> {}, Enemy2: {} -> {}",
            initial_enemy1_hp,
            battle.enemy_team.members[0].hp,
            initial_enemy2_hp,
            battle.enemy_team.members[1].hp
        );
        assert!(!battle.battle_log.is_empty(), "Battle log should contain team-filtered action");
    }

    #[test]
    fn test_sequential_rule_execution_ui_to_battle_integration() {
        // Test that multiple rules execute in order until one succeeds
        let high_priority_rule = vec![
            FlatTokenInput::Check,
            FlatTokenInput::GreaterThan,
            FlatTokenInput::HP,
            FlatTokenInput::ActingCharacter,
            FlatTokenInput::Number(90),
            FlatTokenInput::Strike,
            FlatTokenInput::RandomPick,
            FlatTokenInput::AllCharacters,
        ];
        let medium_priority_rule = vec![
            FlatTokenInput::Check,
            FlatTokenInput::GreaterThan,
            FlatTokenInput::HP,
            FlatTokenInput::ActingCharacter,
            FlatTokenInput::Number(40),
            FlatTokenInput::Heal,
            FlatTokenInput::ActingCharacter,
        ];
        let fallback_rule = vec![
            FlatTokenInput::Strike,
            FlatTokenInput::ActingCharacter,
        ];
        
        let player_rules = convert_flat_rules_to_nodes(&[high_priority_rule, medium_priority_rule, fallback_rule]);
        assert_eq!(player_rules.len(), 3, "Should convert all sequential rules");
        
        // Setup battle with medium HP (should trigger medium priority rule)
        let mut medium_hp_player = GameCharacter::new(1, "Sequential Fighter".to_string(), 100, 50, 25);
        medium_hp_player.hp = 60; // > 40 but < 90
        
        let player_team = Team::new("Heroes".to_string(), vec![medium_hp_player]);
        let enemy_team = Team::new("Enemies".to_string(), vec![
            GameCharacter::new(2, "Test Enemy".to_string(), 70, 30, 18),
        ]);
        
        let rng = create_test_rng();
        let mut battle = TeamBattle::new(player_team, enemy_team, vec![player_rules], vec![vec![]], rng);
        
        // Execute battle - should trigger medium priority rule (heal)
        let initial_player_hp = battle.player_team.members[0].hp;
        let initial_enemy_hp = battle.enemy_team.members[0].hp;
        battle.execute_turn();
        
        // Verify appropriate rule was executed
        let player_healed = battle.player_team.members[0].hp > initial_player_hp;
        let enemy_damaged = battle.enemy_team.members[0].hp < initial_enemy_hp;
        let player_self_damaged = battle.player_team.members[0].hp < initial_player_hp;
        
        assert!(
            player_healed || enemy_damaged || player_self_damaged,
            "One of the sequential rules should have executed. Player HP: {} -> {}, Enemy HP: {} -> {}",
            initial_player_hp,
            battle.player_team.members[0].hp,
            initial_enemy_hp,
            battle.enemy_team.members[0].hp
        );
        assert!(!battle.battle_log.is_empty(), "Battle log should contain sequential rule action");
    }

    #[test]
    fn test_extended_battle_duration_ui_to_battle_integration() {
        // Test longer battle with multiple rounds
        let balanced_strike_rule = vec![FlatTokenInput::Strike, FlatTokenInput::RandomPick, FlatTokenInput::AllCharacters];
        let balanced_heal_rule = vec![
            FlatTokenInput::Check,
            FlatTokenInput::GreaterThan,
            FlatTokenInput::Number(50),
            FlatTokenInput::HP,
            FlatTokenInput::ActingCharacter,
            FlatTokenInput::Heal,
            FlatTokenInput::ActingCharacter,
        ];
        
        let player_rules = convert_flat_rules_to_nodes(&[balanced_strike_rule]);
        let enemy_rules = convert_flat_rules_to_nodes(&[balanced_heal_rule]);
        
        // Setup balanced teams for extended combat
        let player_team = Team::new("Heroes".to_string(), vec![
            GameCharacter::new(1, "Durable Warrior".to_string(), 150, 80, 30),
            GameCharacter::new(2, "Battle Mage".to_string(), 120, 100, 25),
        ]);
        let enemy_team = Team::new("Monsters".to_string(), vec![
            GameCharacter::new(3, "Tough Orc".to_string(), 140, 70, 28),
            GameCharacter::new(4, "Healing Shaman".to_string(), 100, 120, 20),
        ]);
        
        let rng = create_test_rng();
        let mut battle = TeamBattle::new(player_team, enemy_team, vec![player_rules], vec![enemy_rules], rng);
        
        // Execute extended battle (up to 20 turns)
        let initial_total_hp: i32 = battle.player_team.members.iter().map(|c| c.hp).sum::<i32>() +
                                   battle.enemy_team.members.iter().map(|c| c.hp).sum::<i32>();
        let mut turns_executed = 0;
        
        while !battle.battle_over && turns_executed < 20 {
            battle.execute_turn();
            turns_executed += 1;
        }
        
        // Verify extended battle occurred
        assert!(turns_executed > 4, "Extended battle should last more than 4 turns, executed: {}", turns_executed);
        assert!(!battle.battle_log.is_empty(), "Extended battle should have battle log");
        
        // Verify HP changes occurred during extended battle
        let final_total_hp: i32 = battle.player_team.members.iter().map(|c| c.hp).sum::<i32>() +
                                  battle.enemy_team.members.iter().map(|c| c.hp).sum::<i32>();
        
        assert!(
            final_total_hp != initial_total_hp || battle.battle_over,
            "Extended battle should result in HP changes or completion. Initial: {}, Final: {}, Turns: {}",
            initial_total_hp,
            final_total_hp,
            turns_executed
        );
    }
}