#[derive(Clone, Debug, PartialEq)]
pub struct Character {
    pub id: i32,
    pub name: String,
    pub hp: i32,
    pub max_hp: i32,
    pub mp: i32,
    pub max_mp: i32,
    pub attack: i32,
}

#[derive(Clone, Debug)]
pub struct Team {
    pub name: String,
    pub members: Vec<Character>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TeamSide {
    Player,
    Enemy,
}

impl Character {
    pub fn new(id: i32, name: String, max_hp: i32, max_mp: i32, attack: i32) -> Self {
        Self {
            id,
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

impl Team {
    pub fn new(name: String, members: Vec<Character>) -> Self {
        Self { name, members }
    }

    pub fn alive_members(&self) -> Vec<&Character> {
        self.members.iter().filter(|character| character.is_alive()).collect()
    }

    pub fn alive_members_mut(&mut self) -> Vec<&mut Character> {
        self.members.iter_mut().filter(|character| character.is_alive()).collect()
    }

    pub fn is_defeated(&self) -> bool {
        self.alive_members().is_empty()
    }

    pub fn get_member(&self, index: usize) -> Option<&Character> {
        self.members.get(index)
    }

    pub fn get_member_mut(&mut self, index: usize) -> Option<&mut Character> {
        self.members.get_mut(index)
    }

    pub fn get_member_by_name(&self, name: &str) -> Option<&Character> {
        self.members.iter().find(|character| character.name == name)
    }

    pub fn get_member_by_name_mut(&mut self, name: &str) -> Option<&mut Character> {
        self.members.iter_mut().find(|character| character.name == name)
    }

    pub fn get_member_by_id(&self, id: i32) -> Option<&Character> {
        self.members.iter().find(|character| character.id == id)
    }

    pub fn get_member_by_id_mut(&mut self, id: i32) -> Option<&mut Character> {
        self.members.iter_mut().find(|character| character.id == id)
    }

    pub fn total_hp(&self) -> i32 {
        self.members.iter().map(|character| character.hp).sum()
    }

    pub fn total_max_hp(&self) -> i32 {
        self.members.iter().map(|character| character.max_hp).sum()
    }

    pub fn member_count(&self) -> usize {
        self.members.len()
    }

    pub fn alive_count(&self) -> usize {
        self.alive_members().len()
    }
}