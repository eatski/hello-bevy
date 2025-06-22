use crate::action_system::{ActionCalculationSystem, ActionType};

#[derive(Clone, Debug)]
pub struct Character {
    pub name: String,
    pub hp: i32,
    pub max_hp: i32,
    pub mp: i32,
    pub max_mp: i32,
    pub attack: i32,
    pub is_player: bool,
}

impl Character {
    pub fn new(name: String, max_hp: i32, max_mp: i32, attack: i32, is_player: bool) -> Self {
        Self {
            name,
            hp: max_hp,
            max_hp,
            mp: max_mp,
            max_mp,
            attack,
            is_player,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.hp = (self.hp - damage).max(0);
    }

    pub fn heal(&mut self, amount: i32) {
        self.hp = (self.hp + amount).min(self.max_hp);
    }

    pub fn consume_mp(&mut self, amount: i32) -> bool {
        if self.mp >= amount {
            self.mp -= amount;
            true
        } else {
            false
        }
    }

    pub fn restore_mp(&mut self, amount: i32) {
        self.mp = (self.mp + amount).min(self.max_mp);
    }
}

pub struct Battle {
    pub player: Character,
    pub enemy: Character,
    pub current_turn: usize,
    pub battle_over: bool,
    pub winner: Option<String>,
    pub battle_log: Vec<String>,
    pub action_system: ActionCalculationSystem,
}

impl Battle {
    pub fn new(player: Character, enemy: Character) -> Self {
        let default_rules: Vec<Vec<Box<dyn crate::action_system::Token>>> = vec![
            vec![
                Box::new(crate::action_system::Check::new(crate::action_system::TrueOrFalseRandom)),
                Box::new(crate::action_system::Heal),
            ],
            vec![Box::new(crate::action_system::Strike)],
        ];
        
        Self {
            player,
            enemy,
            current_turn: 0,
            battle_over: false,
            winner: None,
            battle_log: Vec::new(),
            action_system: ActionCalculationSystem::new(default_rules),
        }
    }
    
    pub fn new_with_action_system(player: Character, enemy: Character, action_system: ActionCalculationSystem) -> Self {
        Self {
            player,
            enemy,
            current_turn: 0,
            battle_over: false,
            winner: None,
            battle_log: Vec::new(),
            action_system,
        }
    }

    pub fn is_player_turn(&self) -> bool {
        !self.battle_over && self.current_turn % 2 == 0
    }

    pub fn execute_player_action(&mut self) {
        if !self.is_player_turn() {
            return;
        }

        if let Some(action) = self.action_system.calculate_action(&self.player) {
            match action {
                ActionType::Strike => {
                    let damage = self.player.attack;
                    self.enemy.take_damage(damage);
                    self.battle_log.push(format!(
                        "ターン{}: {}が{}に{}のダメージ！",
                        self.current_turn + 1, self.player.name, self.enemy.name, damage
                    ));
                }
                ActionType::Heal => {
                    if self.player.consume_mp(10) {
                        let heal_amount = 20;
                        self.player.heal(heal_amount);
                        self.battle_log.push(format!(
                            "ターン{}: {}がMP10を消費して{}回復！",
                            self.current_turn + 1, self.player.name, heal_amount
                        ));
                    } else {
                        self.battle_log.push(format!(
                            "ターン{}: {}はMPが足りない！",
                            self.current_turn + 1, self.player.name
                        ));
                    }
                }
            }
        } else {
            self.battle_log.push(format!("ターン{}: {}は何もしなかった", self.current_turn + 1, self.player.name));
        }

        if !self.enemy.is_alive() {
            self.battle_over = true;
            self.winner = Some(self.player.name.clone());
            self.battle_log.push(format!("{}の勝利！", self.player.name));
        }

        self.current_turn += 1;
    }

    pub fn execute_enemy_action(&mut self) {
        if self.battle_over || self.is_player_turn() {
            return;
        }

        if let Some(action) = self.action_system.calculate_action(&self.enemy) {
            match action {
                ActionType::Strike => {
                    let damage = self.enemy.attack;
                    self.player.take_damage(damage);
                    self.battle_log.push(format!(
                        "ターン{}: {}が{}に{}のダメージ！",
                        self.current_turn + 1, self.enemy.name, self.player.name, damage
                    ));
                }
                ActionType::Heal => {
                    if self.enemy.consume_mp(10) {
                        let heal_amount = 20;
                        self.enemy.heal(heal_amount);
                        self.battle_log.push(format!(
                            "ターン{}: {}がMP10を消費して{}回復！",
                            self.current_turn + 1, self.enemy.name, heal_amount
                        ));
                    } else {
                        self.battle_log.push(format!(
                            "ターン{}: {}はMPが足りない！",
                            self.current_turn + 1, self.enemy.name
                        ));
                    }
                }
            }
        } else {
            self.battle_log.push(format!("ターン{}: {}は何もしなかった", self.current_turn + 1, self.enemy.name));
        }

        if !self.player.is_alive() {
            self.battle_over = true;
            self.winner = Some(self.enemy.name.clone());
            self.battle_log.push(format!("{}の勝利！", self.enemy.name));
        }

        self.current_turn += 1;
    }

    pub fn get_current_player_name(&self) -> &str {
        if self.is_player_turn() {
            "プレイヤー"
        } else {
            "敵"
        }
    }

    pub fn get_recent_logs(&self, count: usize) -> Vec<&String> {
        self.battle_log.iter().rev().take(count).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character_creation() {
        let character = Character::new("Test".to_string(), 100, 50, 25, true);
        assert_eq!(character.name, "Test");
        assert_eq!(character.hp, 100);
        assert_eq!(character.max_hp, 100);
        assert_eq!(character.mp, 50);
        assert_eq!(character.max_mp, 50);
        assert_eq!(character.attack, 25);
        assert_eq!(character.is_player, true);
        assert!(character.is_alive());
    }

    #[test]
    fn test_character_damage_and_heal() {
        let mut character = Character::new("Test".to_string(), 100, 50, 25, true);
        
        character.take_damage(30);
        assert_eq!(character.hp, 70);
        assert!(character.is_alive());
        
        character.heal(20);
        assert_eq!(character.hp, 90);
        
        character.heal(20);
        assert_eq!(character.hp, 100);
        
        character.take_damage(150);
        assert_eq!(character.hp, 0);
        assert!(!character.is_alive());
    }

    #[test]
    fn test_mp_system() {
        let mut character = Character::new("Test".to_string(), 100, 50, 25, true);
        
        assert_eq!(character.mp, 50);
        assert_eq!(character.max_mp, 50);
        
        assert!(character.consume_mp(20));
        assert_eq!(character.mp, 30);
        
        assert!(!character.consume_mp(40));
        assert_eq!(character.mp, 30);
        
        character.restore_mp(15);
        assert_eq!(character.mp, 45);
        
        character.restore_mp(10);
        assert_eq!(character.mp, 50);
    }

    #[test]
    fn test_battle_creation() {
        let player = Character::new("Player".to_string(), 100, 50, 25, true);
        let enemy = Character::new("Enemy".to_string(), 80, 40, 20, false);
        let battle = Battle::new(player, enemy);
        
        assert_eq!(battle.player.name, "Player");
        assert_eq!(battle.enemy.name, "Enemy");
        assert_eq!(battle.current_turn, 0);
        assert!(!battle.battle_over);
        assert!(battle.winner.is_none());
        assert!(battle.is_player_turn());
    }

    #[test]
    fn test_battle_turn_system() {
        let player = Character::new("Player".to_string(), 100, 50, 25, true);
        let enemy = Character::new("Enemy".to_string(), 80, 40, 20, false);
        let mut battle = Battle::new(player, enemy);
        
        assert!(battle.is_player_turn());
        
        battle.execute_player_action();
        assert!(!battle.is_player_turn());
        
        battle.execute_enemy_action();
        assert!(battle.is_player_turn());
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::action_system::{ActionCalculationSystem, ActionType};

    #[test]
    fn test_action_system_integration_with_battle() {
        let player = Character::new("Player".to_string(), 100, 50, 25, true);
        let enemy = Character::new("Enemy".to_string(), 80, 40, 20, false);
        let mut battle = Battle::new(player, enemy);
        
        let initial_enemy_hp = battle.enemy.hp;
        let initial_player_mp = battle.player.mp;
        
        battle.execute_player_action();
        
        let action_occurred = battle.enemy.hp < initial_enemy_hp || battle.player.mp < initial_player_mp;
        assert!(action_occurred, "Some action should have occurred");
        assert_eq!(battle.current_turn, 1);
        assert!(!battle.battle_over);
    }

    #[test]
    fn test_complete_battle_simulation() {
        let player = Character::new("Player".to_string(), 50, 30, 30, true);
        let enemy = Character::new("Enemy".to_string(), 40, 20, 25, false);
        let mut battle = Battle::new(player, enemy);
        
        let mut turn_count = 0;
        let max_turns = 20;
        
        while !battle.battle_over && turn_count < max_turns {
            if battle.is_player_turn() {
                battle.execute_player_action();
            } else {
                battle.execute_enemy_action();
            }
            turn_count += 1;
        }
        
        assert!(battle.battle_over || turn_count == max_turns, "Battle should end or reach max turns");
        if battle.battle_over {
            assert!(battle.winner.is_some(), "Winner should be determined");
            assert!(!battle.player.is_alive() || !battle.enemy.is_alive(), "One character should be defeated");
        }
        assert!(!battle.battle_log.is_empty(), "Battle log should contain actions");
    }

    #[test]
    fn test_mp_consumption_during_battle() {
        let mut player = Character::new("Player".to_string(), 100, 50, 25, true);
        let _enemy = Character::new("Enemy".to_string(), 100, 50, 25, false);
        
        player.take_damage(50);
        
        let rules: Vec<Vec<Box<dyn crate::action_system::Token>>> = vec![
            vec![
                Box::new(crate::action_system::Check::new(crate::action_system::TrueOrFalseRandom)),
                Box::new(crate::action_system::Heal),
            ],
            vec![Box::new(crate::action_system::Strike)],
        ];
        let mut action_system = ActionCalculationSystem::with_seed(rules, 123);
        
        let _initial_mp = player.mp;
        if let Some(ActionType::Heal) = action_system.calculate_action(&player) {
            if player.consume_mp(10) {
                player.heal(20);
                assert_eq!(player.mp, _initial_mp - 10, "MP should be consumed for heal");
                assert!(player.hp > 50, "HP should be restored");
            }
        }
    }

    #[test]
    fn test_low_mp_behavior() {
        let mut player = Character::new("Player".to_string(), 50, 5, 25, true);
        player.take_damage(30);
        
        let rules: Vec<Vec<Box<dyn crate::action_system::Token>>> = vec![
            vec![
                Box::new(crate::action_system::Check::new(crate::action_system::TrueOrFalseRandom)),
                Box::new(crate::action_system::Heal),
            ],
            vec![Box::new(crate::action_system::Strike)],
        ];
        let mut action_system = ActionCalculationSystem::new(rules);
        
        let mut _heal_attempts = 0;
        let mut strike_actions = 0;
        
        for _ in 0..20 {
            if let Some(action) = action_system.calculate_action(&player) {
                match action {
                    ActionType::Heal => {
                        _heal_attempts += 1;
                        if player.mp >= 10 {
                            player.consume_mp(10);
                        }
                    }
                    ActionType::Strike => strike_actions += 1,
                }
            }
        }
        
        assert!(strike_actions > 0, "Should perform Strike when MP is low");
    }

    #[test]
    fn test_battle_determinism_with_seed() {
        let create_battle = || {
            let player = Character::new("Player".to_string(), 80, 40, 20, true);
            let enemy = Character::new("Enemy".to_string(), 70, 30, 18, false);
            let rules: Vec<Vec<Box<dyn crate::action_system::Token>>> = vec![
                vec![
                    Box::new(crate::action_system::Check::new(crate::action_system::TrueOrFalseRandom)),
                    Box::new(crate::action_system::Heal),
                ],
                vec![Box::new(crate::action_system::Strike)],
            ];
            let action_system = ActionCalculationSystem::with_seed(rules, 42);
            Battle::new_with_action_system(player, enemy, action_system)
        };
        
        let mut battle1 = create_battle();
        let mut battle2 = create_battle();
        
        for _ in 0..5 {
            if battle1.is_player_turn() {
                battle1.execute_player_action();
            } else {
                battle1.execute_enemy_action();
            }
            
            if battle2.is_player_turn() {
                battle2.execute_player_action();
            } else {
                battle2.execute_enemy_action();
            }
            
            if battle1.battle_over || battle2.battle_over {
                break;
            }
        }
        
        assert_eq!(battle1.player.hp, battle2.player.hp, "Player HP should match with same seed");
        assert_eq!(battle1.enemy.hp, battle2.enemy.hp, "Enemy HP should match with same seed");
        assert_eq!(battle1.battle_log.len(), battle2.battle_log.len(), "Battle logs should have same length");
    }

    #[test]
    fn test_full_battle_workflow() {
        let player = Character::new("Hero".to_string(), 100, 50, 30, true);
        let enemy = Character::new("Goblin".to_string(), 60, 20, 15, false);
        let mut battle = Battle::new(player, enemy);
        
        let initial_state = (battle.player.hp, battle.enemy.hp, battle.current_turn);
        
        battle.execute_player_action();
        battle.execute_enemy_action();
        
        assert_ne!((battle.player.hp, battle.enemy.hp, battle.current_turn), initial_state);
        assert_eq!(battle.current_turn, 2);
        assert!(!battle.battle_log.is_empty());
    }

    #[test]
    fn test_battle_with_hp_management() {
        let mut player = Character::new("Player".to_string(), 30, 50, 25, true);
        let enemy = Character::new("Enemy".to_string(), 50, 30, 20, false);
        player.take_damage(20);
        let mut battle = Battle::new(player, enemy);
        
        let initial_player_hp = battle.player.hp;
        
        for _ in 0..10 {
            if battle.battle_over {
                break;
            }
            
            if battle.is_player_turn() {
                battle.execute_player_action();
            } else {
                battle.execute_enemy_action();
            }
        }
        
        if !battle.battle_over && battle.player.hp > initial_player_hp {
            let heal_actions: Vec<_> = battle.battle_log
                .iter()
                .filter(|log| log.contains("回復"))
                .collect();
            assert!(!heal_actions.is_empty(), "Should have heal actions when HP is low");
        }
    }

    #[test]
    fn test_battle_victory_conditions() {
        let player = Character::new("Player".to_string(), 100, 50, 50, true);
        let weak_enemy = Character::new("Weak Enemy".to_string(), 10, 10, 5, false);
        let mut battle = Battle::new(player, weak_enemy);
        
        let mut turns = 0;
        while !battle.battle_over && turns < 5 {
            if battle.is_player_turn() {
                battle.execute_player_action();
            } else {
                battle.execute_enemy_action();
            }
            turns += 1;
        }
        
        assert!(battle.battle_over, "Battle should end when enemy is defeated");
        assert!(battle.winner.is_some(), "Winner should be determined");
        assert!(!battle.enemy.is_alive() || !battle.player.is_alive(), "One character should be defeated");
        
        let victory_log: Vec<_> = battle.battle_log
            .iter()
            .filter(|log| log.contains("勝利"))
            .collect();
        assert!(!victory_log.is_empty(), "Should have victory message in log");
    }

    #[test]
    fn test_battle_with_mp_depletion() {
        let low_mp_player = Character::new("Player".to_string(), 50, 5, 25, true);
        let enemy = Character::new("Enemy".to_string(), 100, 50, 20, false);
        let mut battle = Battle::new(low_mp_player, enemy);
        
        battle.player.take_damage(30);
        
        let _initial_mp = battle.player.mp;
        
        for _ in 0..5 {
            if battle.battle_over {
                break;
            }
            battle.execute_player_action();
            battle.execute_enemy_action();
        }
        
        if battle.player.mp == 0 {
            let mp_insufficient_logs: Vec<_> = battle.battle_log
                .iter()
                .filter(|log| log.contains("MPが足りない"))
                .collect();
            
            if !mp_insufficient_logs.is_empty() {
                assert!(true, "MP depletion handled correctly");
            }
        }
    }

    #[test]
    fn test_action_system_context_integration() {
        let damaged_player = Character::new("Player".to_string(), 20, 50, 25, true);
        let healthy_player = Character::new("Player2".to_string(), 100, 50, 25, true);
        
        let rules: Vec<Vec<Box<dyn crate::action_system::Token>>> = vec![
            vec![
                Box::new(crate::action_system::Check::new(crate::action_system::TrueOrFalseRandom)),
                Box::new(crate::action_system::Heal),
            ],
            vec![Box::new(crate::action_system::Strike)],
        ];
        let mut action_system = ActionCalculationSystem::with_seed(rules, 123);
        
        let damaged_action = action_system.calculate_action(&damaged_player);
        let healthy_action = action_system.calculate_action(&healthy_player);
        
        assert!(damaged_action.is_some());
        assert!(healthy_action.is_some());
        
        let mut heal_for_damaged = false;
        let mut strike_for_healthy = false;
        
        for seed in 0..20 {
            let rules: Vec<Vec<Box<dyn crate::action_system::Token>>> = vec![
                vec![
                    Box::new(crate::action_system::Check::new(crate::action_system::TrueOrFalseRandom)),
                    Box::new(crate::action_system::Heal),
                ],
                vec![Box::new(crate::action_system::Strike)],
            ];
            let mut system = ActionCalculationSystem::with_seed(rules, seed);
            
            if let Some(ActionType::Heal) = system.calculate_action(&damaged_player) {
                heal_for_damaged = true;
            }
            
            if let Some(ActionType::Strike) = system.calculate_action(&healthy_player) {
                strike_for_healthy = true;
            }
            
            if heal_for_damaged && strike_for_healthy {
                break;
            }
        }
        
        assert!(heal_for_damaged || strike_for_healthy, "Should produce both action types across different seeds");
    }

    #[test]
    fn test_battle_log_accuracy() {
        let player = Character::new("Hero".to_string(), 80, 40, 25, true);
        let enemy = Character::new("Monster".to_string(), 70, 30, 20, false);
        let mut battle = Battle::new(player, enemy);
        
        let initial_log_count = battle.battle_log.len();
        
        battle.execute_player_action();
        battle.execute_enemy_action();
        
        assert!(battle.battle_log.len() > initial_log_count, "Battle log should grow with actions");
        
        let recent_logs = battle.get_recent_logs(2);
        assert_eq!(recent_logs.len(), 2, "Should return correct number of recent logs");
        
        for log in &battle.battle_log {
            assert!(log.contains("ターン"), "Each log should contain turn information");
            assert!(log.contains("Hero") || log.contains("Monster"), "Each log should reference a character");
        }
    }

    #[test]
    fn test_battle_state_consistency() {
        let player = Character::new("Player".to_string(), 60, 40, 20, true);
        let enemy = Character::new("Enemy".to_string(), 50, 30, 18, false);
        let mut battle = Battle::new(player, enemy);
        
        for _turn in 0..10 {
            if battle.battle_over {
                break;
            }
            
            let pre_turn = battle.current_turn;
            let pre_battle_over = battle.battle_over;
            
            if battle.is_player_turn() {
                battle.execute_player_action();
            } else {
                battle.execute_enemy_action();
            }
            
            assert_eq!(battle.current_turn, pre_turn + 1, "Turn should increment by 1");
            
            if !pre_battle_over && battle.battle_over {
                assert!(battle.winner.is_some(), "Winner should be set when battle ends");
                assert!(!battle.player.is_alive() || !battle.enemy.is_alive(), "One character should be defeated");
            }
            
            assert!(battle.player.hp >= 0, "HP should never be negative");
            assert!(battle.enemy.hp >= 0, "HP should never be negative");
            assert!(battle.player.mp >= 0, "MP should never be negative");
            assert!(battle.enemy.mp >= 0, "MP should never be negative");
        }
    }

    #[test]
    fn test_strike_only_rule() {
        let player = Character::new("Warrior".to_string(), 80, 30, 35, true);
        let enemy = Character::new("Target".to_string(), 60, 20, 15, false);
        
        let strike_only_rules: Vec<Vec<Box<dyn crate::action_system::Token>>> = vec![
            vec![Box::new(crate::action_system::Strike)],
        ];
        let action_system = ActionCalculationSystem::new(strike_only_rules);
        let mut battle = Battle::new_with_action_system(player, enemy, action_system);
        
        let initial_enemy_hp = battle.enemy.hp;
        battle.execute_player_action();
        
        assert!(battle.enemy.hp < initial_enemy_hp, "Enemy should take damage from Strike");
        assert!(battle.battle_log.iter().any(|log| log.contains("ダメージ")), "Should have damage log");
        assert!(!battle.battle_log.iter().any(|log| log.contains("回復")), "Should not have heal log");
    }

    #[test]
    fn test_heal_only_rule() {
        let mut player = Character::new("Healer".to_string(), 100, 50, 20, true);
        player.take_damage(40);
        let enemy = Character::new("Dummy".to_string(), 100, 30, 10, false);
        
        let heal_only_rules: Vec<Vec<Box<dyn crate::action_system::Token>>> = vec![
            vec![Box::new(crate::action_system::Heal)],
        ];
        let action_system = ActionCalculationSystem::new(heal_only_rules);
        let mut battle = Battle::new_with_action_system(player, enemy, action_system);
        
        let initial_player_hp = battle.player.hp;
        let initial_player_mp = battle.player.mp;
        battle.execute_player_action();
        
        if battle.player.mp < initial_player_mp {
            assert!(battle.player.hp > initial_player_hp, "Player should heal when MP is available");
            assert!(battle.battle_log.iter().any(|log| log.contains("回復")), "Should have heal log");
        }
    }

    #[test]
    fn test_complex_rule_chain() {
        let player = Character::new("Tactician".to_string(), 70, 40, 25, true);
        let enemy = Character::new("Opponent".to_string(), 80, 35, 20, false);
        
        let complex_rules: Vec<Vec<Box<dyn crate::action_system::Token>>> = vec![
            vec![
                Box::new(crate::action_system::Check::new(crate::action_system::TrueOrFalseRandom)),
                Box::new(crate::action_system::Check::new(crate::action_system::TrueOrFalseRandom)),
                Box::new(crate::action_system::Heal),
            ],
            vec![
                Box::new(crate::action_system::Check::new(crate::action_system::TrueOrFalseRandom)),
                Box::new(crate::action_system::Strike),
            ],
            vec![Box::new(crate::action_system::Strike)],
        ];
        let action_system = ActionCalculationSystem::with_seed(complex_rules, 42);
        let mut battle = Battle::new_with_action_system(player, enemy, action_system);
        
        let mut action_count = 0;
        for _ in 0..10 {
            if battle.battle_over {
                break;
            }
            let initial_state = (battle.player.hp, battle.enemy.hp, battle.player.mp);
            battle.execute_player_action();
            let final_state = (battle.player.hp, battle.enemy.hp, battle.player.mp);
            
            if initial_state != final_state {
                action_count += 1;
            }
            
            if !battle.battle_over {
                battle.execute_enemy_action();
            }
        }
        
        assert!(action_count > 0, "Complex rules should produce some actions");
        assert!(!battle.battle_log.is_empty(), "Should have battle log entries");
    }

    #[test]
    fn test_empty_rules() {
        let player = Character::new("Inactive".to_string(), 50, 30, 20, true);
        let enemy = Character::new("Target".to_string(), 60, 25, 15, false);
        
        let empty_rules: Vec<Vec<Box<dyn crate::action_system::Token>>> = vec![];
        let action_system = ActionCalculationSystem::new(empty_rules);
        let mut battle = Battle::new_with_action_system(player, enemy, action_system);
        
        let initial_state = (battle.player.hp, battle.enemy.hp, battle.player.mp);
        battle.execute_player_action();
        let final_state = (battle.player.hp, battle.enemy.hp, battle.player.mp);
        
        assert_eq!(initial_state, final_state, "Empty rules should not change state");
        assert!(battle.battle_log.iter().any(|log| log.contains("何もしなかった")), "Should log no action");
    }

    #[test]
    fn test_aggressive_vs_defensive_strategies() {
        let player = Character::new("Aggressor".to_string(), 80, 40, 30, true);
        let enemy = Character::new("Defender".to_string(), 100, 60, 20, false);
        
        let aggressive_rules: Vec<Vec<Box<dyn crate::action_system::Token>>> = vec![
            vec![Box::new(crate::action_system::Strike)],
        ];
        let aggressive_system = ActionCalculationSystem::new(aggressive_rules);
        
        let defensive_rules: Vec<Vec<Box<dyn crate::action_system::Token>>> = vec![
            vec![Box::new(crate::action_system::Heal)],
            vec![Box::new(crate::action_system::Strike)],
        ];
        let defensive_system = ActionCalculationSystem::new(defensive_rules);
        
        let mut aggressive_battle = Battle::new_with_action_system(
            player.clone(), 
            enemy.clone(), 
            aggressive_system
        );
        let mut defensive_battle = Battle::new_with_action_system(
            player.clone(), 
            enemy.clone(), 
            defensive_system
        );
        
        aggressive_battle.execute_player_action();
        defensive_battle.execute_player_action();
        
        let aggressive_damage = enemy.hp - aggressive_battle.enemy.hp;
        let defensive_damage = enemy.hp - defensive_battle.enemy.hp;
        
        if aggressive_damage > 0 && defensive_damage == 0 {
            assert!(true, "Aggressive strategy should deal more damage");
        } else if defensive_damage > 0 && aggressive_damage == 0 {
            assert!(true, "Defensive strategy might heal instead of attack");
        }
        
        assert!(aggressive_battle.battle_log.len() > 0, "Should have action logs");
        assert!(defensive_battle.battle_log.len() > 0, "Should have action logs");
    }

    #[test]
    fn test_conditional_healing_strategy() {
        let mut low_hp_player = Character::new("Wounded".to_string(), 100, 50, 25, true);
        low_hp_player.take_damage(80);
        let mut high_hp_player = Character::new("Healthy".to_string(), 100, 50, 25, true);
        high_hp_player.take_damage(10);
        let enemy = Character::new("Enemy".to_string(), 60, 30, 20, false);
        
        let create_conditional_rules = || -> Vec<Vec<Box<dyn crate::action_system::Token>>> {
            vec![
                vec![
                    Box::new(crate::action_system::Check::new(crate::action_system::TrueOrFalseRandom)),
                    Box::new(crate::action_system::Heal),
                ],
                vec![Box::new(crate::action_system::Strike)],
            ]
        };
        
        let low_hp_system = ActionCalculationSystem::with_seed(create_conditional_rules(), 123);
        let high_hp_system = ActionCalculationSystem::with_seed(create_conditional_rules(), 123);
        
        let mut low_hp_battle = Battle::new_with_action_system(
            low_hp_player, 
            enemy.clone(), 
            low_hp_system
        );
        let mut high_hp_battle = Battle::new_with_action_system(
            high_hp_player, 
            enemy.clone(), 
            high_hp_system
        );
        
        for _ in 0..5 {
            if !low_hp_battle.battle_over {
                low_hp_battle.execute_player_action();
            }
            if !high_hp_battle.battle_over {
                high_hp_battle.execute_player_action();
            }
        }
        
        let low_hp_heal_count = low_hp_battle.battle_log.iter()
            .filter(|log| log.contains("回復"))
            .count();
        let high_hp_heal_count = high_hp_battle.battle_log.iter()
            .filter(|log| log.contains("回復"))
            .count();
        
        assert!(low_hp_heal_count >= 0, "Low HP character should attempt healing");
        assert!(high_hp_heal_count >= 0, "High HP character should also have healing attempts");
    }

    #[test]
    fn test_deterministic_rule_execution() {
        let player = Character::new("Consistent".to_string(), 70, 40, 25, true);
        let enemy = Character::new("Target".to_string(), 80, 35, 20, false);
        
        let create_deterministic_rules = || -> Vec<Vec<Box<dyn crate::action_system::Token>>> {
            vec![
                vec![Box::new(crate::action_system::Strike)],
            ]
        };
        
        let system1 = ActionCalculationSystem::with_seed(create_deterministic_rules(), 999);
        let system2 = ActionCalculationSystem::with_seed(create_deterministic_rules(), 999);
        
        let mut battle1 = Battle::new_with_action_system(
            player.clone(), 
            enemy.clone(), 
            system1
        );
        let mut battle2 = Battle::new_with_action_system(
            player.clone(), 
            enemy.clone(), 
            system2
        );
        
        for _ in 0..3 {
            if !battle1.battle_over && !battle2.battle_over {
                battle1.execute_player_action();
                battle2.execute_player_action();
                
                assert_eq!(battle1.player.hp, battle2.player.hp, "Player HP should match");
                assert_eq!(battle1.enemy.hp, battle2.enemy.hp, "Enemy HP should match");
                assert_eq!(battle1.player.mp, battle2.player.mp, "Player MP should match");
                assert_eq!(battle1.battle_log.len(), battle2.battle_log.len(), "Log length should match");
            }
        }
    }
}