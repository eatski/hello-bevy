use rand::Rng;

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
}

pub struct Battle {
    pub player: Character,
    pub enemy: Character,
    pub current_turn: usize,
    pub battle_over: bool,
    pub winner: Option<String>,
    pub battle_log: Vec<String>,
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
        }
    }

    pub fn is_player_turn(&self) -> bool {
        !self.battle_over && self.current_turn % 2 == 0
    }

    pub fn execute_player_action(&mut self) {
        if !self.is_player_turn() {
            return;
        }

        let damage = self.player.attack;
        self.enemy.take_damage(damage);
        self.battle_log.push(format!(
            "{}が{}に{}のダメージ！",
            self.player.name, self.enemy.name, damage
        ));

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

        let mut rng = rand::thread_rng();
        let damage = rng.gen_range(10..=self.enemy.attack);
        self.player.take_damage(damage);
        self.battle_log.push(format!(
            "{}が{}に{}のダメージ！",
            self.enemy.name, self.player.name, damage
        ));

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