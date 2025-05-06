use super::{Character, BattleLog};

#[derive(Debug, Clone)]
pub enum Skill {
    Attack,
    Heal,
}

impl Skill {
    pub fn execute(&self, user: &Character, target: &mut Character) -> BattleLog {
        match self {
            Skill::Attack => {
                let damage = (user.stats.attack - target.stats.defense / 2).max(1);
                target.stats.hp = (target.stats.hp - damage).max(0);
                
                BattleLog {
                    actor: user.name.clone(),
                    action: "attacks".to_string(),
                    target: target.name.clone(),
                    damage: Some(damage),
                    healing: None,
                }
            }
            Skill::Heal => {
                let healing = (user.stats.attack / 2).max(1);
                let new_hp = (target.stats.hp + healing).min(target.stats.max_hp);
                let actual_healing = new_hp - target.stats.hp;
                target.stats.hp = new_hp;

                BattleLog {
                    actor: user.name.clone(),
                    action: "heals".to_string(),
                    target: target.name.clone(),
                    damage: None,
                    healing: Some(actual_healing),
                }
            }
        }
    }
}
