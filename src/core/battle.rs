use std::collections::VecDeque;
use super::{Character, Skill};

#[derive(Debug)]
pub struct BattleLog {
    pub actor: String,
    pub action: String,
    pub target: String,
    pub damage: Option<i32>,
    pub healing: Option<i32>,
}

#[derive(Debug)]
pub struct Battle {
    pub player: Character,
    pub enemy: Character,
    pub turn_order: VecDeque<String>,
    pub logs: Vec<BattleLog>,
}

impl Battle {
    pub fn new(player: Character, enemy: Character) -> Self {
        let mut turn_order = VecDeque::new();
        // Simple speed-based turn order
        if player.stats.speed >= enemy.stats.speed {
            turn_order.push_back(player.name.clone());
            turn_order.push_back(enemy.name.clone());
        } else {
            turn_order.push_back(enemy.name.clone());
            turn_order.push_back(player.name.clone());
        }

        Self {
            player,
            enemy,
            turn_order,
            logs: Vec::new(),
        }
    }

    pub fn execute_turn(&mut self, skill: &Skill) -> bool {
        let current_turn = self.turn_order.pop_front().unwrap();
        let log = if current_turn == self.player.name {
            // Player's turn
            skill.execute(&self.player, &mut self.enemy)
        } else {
        // Enemy's turn - use Heal if HP is below 50%, otherwise Attack
        if self.enemy.hp_percentage() < 50.0 {
            Skill::Heal.execute(&self.enemy.clone(), &mut self.enemy)
        } else {
            Skill::Attack.execute(&self.enemy, &mut self.player)
        }
        };

        self.logs.push(log);
        self.turn_order.push_back(current_turn);

        // Check if battle is over
        !self.player.is_alive() || !self.enemy.is_alive()
    }

    pub fn is_player_winner(&self) -> Option<bool> {
        if !self.player.is_alive() {
            Some(false)
        } else if !self.enemy.is_alive() {
            Some(true)
        } else {
            None
        }
    }
}
