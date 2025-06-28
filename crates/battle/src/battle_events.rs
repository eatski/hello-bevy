// Battle events - language-independent battle log events

#[derive(Clone, Debug, PartialEq)]
pub enum BattleEvent {
    TurnStart(u32), // Turn number
    NoAction { character_name: String, turn: u32 },
    PlayerAttack { damage: i32, turn: u32 },
    EnemyAttack { damage: i32, turn: u32 },
    PlayerHeal { amount: i32, turn: u32 },
    EnemyHeal { amount: i32, turn: u32 },
    Victory { winner_name: String },
}

impl BattleEvent {
    pub fn turn_number(&self) -> u32 {
        match self {
            BattleEvent::TurnStart(turn) => *turn,
            BattleEvent::NoAction { turn, .. } => *turn,
            BattleEvent::PlayerAttack { turn, .. } => *turn,
            BattleEvent::EnemyAttack { turn, .. } => *turn,
            BattleEvent::PlayerHeal { turn, .. } => *turn,
            BattleEvent::EnemyHeal { turn, .. } => *turn,
            BattleEvent::Victory { .. } => 0, // Victory doesn't have a turn
        }
    }
}