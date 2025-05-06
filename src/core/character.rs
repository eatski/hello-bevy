use super::{Stats, Skill};

#[derive(Debug, Clone)]
pub struct Character {
    pub name: String,
    pub stats: Stats,
    pub skills: Vec<Skill>,
}

impl Character {
    pub fn new(name: &str, hp: i32, attack: i32, defense: i32, speed: i32) -> Self {
        Self {
            name: name.to_string(),
            stats: Stats {
                hp,
                max_hp: hp,
                attack,
                defense,
                speed,
            },
            skills: vec![Skill::Attack, Skill::Heal],
        }
    }

    pub fn is_alive(&self) -> bool {
        self.stats.hp > 0
    }

    pub fn get_available_skills(&self) -> &[Skill] {
        &self.skills
    }

    pub fn hp_percentage(&self) -> f32 {
        self.stats.hp as f32 / self.stats.max_hp as f32 * 100.0
    }
}
