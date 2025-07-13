// Default team configurations for the game
use battle::{Team, Character as GameCharacter};

pub const DEFAULT_PLAYER_TEAM_NAME: &str = "勇者パーティー";
pub const DEFAULT_ENEMY_TEAM_NAME: &str = "モンスター軍団";
pub const DEFAULT_ENEMY_RULES_PATH: &str = "rules/enemy_rules.json";

pub fn create_default_player_team() -> Team {
    Team::new(DEFAULT_PLAYER_TEAM_NAME.to_string(), vec![
        GameCharacter::new(1, "勇者".to_string(), 100, 80, 25),
        GameCharacter::new(2, "戦士".to_string(), 120, 50, 30),
        GameCharacter::new(3, "魔法使い".to_string(), 70, 100, 15),
    ])
}

pub fn create_default_enemy_team() -> Team {
    Team::new(DEFAULT_ENEMY_TEAM_NAME.to_string(), vec![
        GameCharacter::new(4, "オーク".to_string(), 150, 30, 20),
        GameCharacter::new(5, "ゴブリン".to_string(), 80, 40, 15),
        GameCharacter::new(6, "スライム".to_string(), 60, 60, 10),
    ])
}