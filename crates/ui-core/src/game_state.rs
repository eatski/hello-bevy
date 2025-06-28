// Game state management - independent of Bevy

#[derive(Default)]
pub struct GameState {
    pub mode: GameMode,
}

#[derive(Default, PartialEq, Clone, Debug)]
pub enum GameMode {
    #[default]
    RuleCreation, // ルール作成モード
    Battle,       // 戦闘モード
}

impl GameState {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn switch_to_battle(&mut self) {
        self.mode = GameMode::Battle;
    }
    
    pub fn switch_to_rule_creation(&mut self) {
        self.mode = GameMode::RuleCreation;
    }
    
    pub fn is_battle_mode(&self) -> bool {
        self.mode == GameMode::Battle
    }
    
    pub fn is_rule_creation_mode(&self) -> bool {
        self.mode == GameMode::RuleCreation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_state_creation() {
        let state = GameState::new();
        assert_eq!(state.mode, GameMode::RuleCreation);
        assert_eq!(state.is_rule_creation_mode(), true);
        assert_eq!(state.is_battle_mode(), false);
    }
    
    #[test]
    fn test_game_state_transitions() {
        let mut state = GameState::new();
        
        // Switch to battle
        state.switch_to_battle();
        assert_eq!(state.mode, GameMode::Battle);
        assert_eq!(state.is_battle_mode(), true);
        assert_eq!(state.is_rule_creation_mode(), false);
        
        // Switch back to rule creation
        state.switch_to_rule_creation();
        assert_eq!(state.mode, GameMode::RuleCreation);
        assert_eq!(state.is_rule_creation_mode(), true);
        assert_eq!(state.is_battle_mode(), false);
    }
}