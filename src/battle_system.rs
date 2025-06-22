use crate::action_system::{ActionCalculationSystem, ActionType};

#[derive(Clone, Debug)]
pub struct Character {
    pub name: String,
    pub hp: i32,
    pub max_hp: i32,
    pub attack: i32,
    pub is_player: bool,
}

impl Character {
    pub fn new(name: String, max_hp: i32, attack: i32, is_player: bool) -> Self {
        Self {
            name,
            hp: max_hp,
            max_hp,
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
        Self {
            player,
            enemy,
            current_turn: 0,
            battle_over: false,
            winner: None,
            battle_log: Vec::new(),
            action_system: ActionCalculationSystem::new(),
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
                    let heal_amount = 20;
                    self.player.heal(heal_amount);
                    self.battle_log.push(format!(
                        "ターン{}: {}が{}回復！",
                        self.current_turn + 1, self.player.name, heal_amount
                    ));
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
                    let heal_amount = 20;
                    self.enemy.heal(heal_amount);
                    self.battle_log.push(format!(
                        "ターン{}: {}が{}回復！",
                        self.current_turn + 1, self.enemy.name, heal_amount
                    ));
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
        let character = Character::new("Test".to_string(), 100, 25, true);
        assert_eq!(character.name, "Test");
        assert_eq!(character.hp, 100);
        assert_eq!(character.max_hp, 100);
        assert_eq!(character.attack, 25);
        assert_eq!(character.is_player, true);
        assert!(character.is_alive());
    }

    #[test]
    fn test_character_damage_and_heal() {
        let mut character = Character::new("Test".to_string(), 100, 25, true);
        
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
    fn test_battle_creation() {
        let player = Character::new("Player".to_string(), 100, 25, true);
        let enemy = Character::new("Enemy".to_string(), 80, 20, false);
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
        let player = Character::new("Player".to_string(), 100, 25, true);
        let enemy = Character::new("Enemy".to_string(), 80, 20, false);
        let mut battle = Battle::new(player, enemy);
        
        assert!(battle.is_player_turn());
        
        battle.execute_player_action();
        assert!(!battle.is_player_turn());
        
        battle.execute_enemy_action();
        assert!(battle.is_player_turn());
    }
}