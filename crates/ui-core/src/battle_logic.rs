// Battle logic and orchestration without Bevy dependencies
use battle::{TeamBattle, Team};
use json_rule::RuleSet;
use token_input::convert_to_rule_node;
use crate::{CurrentRules, GameMode, GameState};

pub struct BattleOrchestrator;

impl BattleOrchestrator {
    // Create and setup a new battle with the given teams and rules
    pub fn create_battle(
        current_rules: &CurrentRules,
        player_team: Team,
        enemy_team: Team,
        _enemy_rule_set: &RuleSet,
        rng: rand::rngs::StdRng,
    ) -> TeamBattle {
        // Convert UI rules for player characters
        let player_rules_per_character: Vec<_> = (0..player_team.members.len())
            .map(|_| current_rules.convert_to_rule_nodes())
            .collect();
        
        // Convert enemy rule set for each enemy character
        let enemy_rules_per_character: Vec<_> = (0..enemy_team.members.len())
            .map(|_| {
                _enemy_rule_set.rules.iter()
                    .filter_map(|token| convert_to_rule_node(token))
                    .collect::<Vec<_>>()
            })
            .collect();
        
        TeamBattle::new(
            player_team, 
            enemy_team, 
            player_rules_per_character, 
            enemy_rules_per_character, 
            rng
        )
    }
    
    // Check if we should start a new battle when switching to battle mode
    pub fn should_start_new_battle(game_state: &GameState, is_changed: bool) -> bool {
        is_changed && game_state.mode == GameMode::Battle
    }
    
    // Check if restart is requested
    pub fn should_restart_battle(battle_over: bool, shift_pressed: bool) -> bool {
        battle_over && shift_pressed
    }
    
    // Check if turn should be executed
    pub fn should_execute_turn(
        game_state: &GameState,
        battle_over: bool,
        space_pressed: bool,
    ) -> bool {
        game_state.mode == GameMode::Battle && !battle_over && space_pressed
    }
    
    // Execute turn and return info about what happened
    pub fn execute_turn(battle: &mut TeamBattle) -> (bool, usize) {
        battle.execute_turn();
        (true, battle.current_turn)
    }
    
    // Get UI text based on game mode
    pub fn get_battle_ui_text(game_state: &GameState) -> Option<&'static str> {
        match game_state.mode {
            GameMode::RuleCreation => Some("rule_creation_mode"),
            GameMode::Battle => None,
        }
    }
    
    // Get log UI text based on game mode  
    pub fn get_log_ui_text(game_state: &GameState) -> Option<&'static str> {
        match game_state.mode {
            GameMode::RuleCreation => Some("rule_creation_log"),
            GameMode::Battle => None,
        }
    }
}