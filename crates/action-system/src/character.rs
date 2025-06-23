#[derive(Clone, Debug)]
pub struct Character {
    pub name: String,
    pub hp: i32,
    pub max_hp: i32,
    pub mp: i32,
    pub max_mp: i32,
    pub attack: i32,
}

impl Character {
    pub fn new(name: String, max_hp: i32, max_mp: i32, attack: i32) -> Self {
        Self {
            name,
            hp: max_hp,
            max_hp,
            mp: max_mp,
            max_mp,
            attack,
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

    pub fn hp_percentage(&self) -> f32 {
        if self.max_hp > 0 {
            self.hp as f32 / self.max_hp as f32
        } else {
            0.0
        }
    }

    pub fn mp_percentage(&self) -> f32 {
        if self.max_mp > 0 {
            self.mp as f32 / self.max_mp as f32
        } else {
            0.0
        }
    }
}