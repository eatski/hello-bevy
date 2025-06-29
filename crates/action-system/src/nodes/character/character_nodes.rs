// Character nodes - nodes that evaluate to character references for calculations

// Team battle context enum for different battle types
#[derive(Debug)]
pub enum BattleContext<'a> {
    SingleCharacter {
        acting_character: &'a crate::Character,
        player: &'a crate::Character,
        enemy: &'a crate::Character,
    },
    TeamBattle {
        acting_character: &'a crate::Character,
        acting_team: crate::TeamSide,
        player_team: &'a crate::Team,
        enemy_team: &'a crate::Team,
    },
}

impl<'a> BattleContext<'a> {
    // New constructor for team battles
    pub fn new_team(
        acting_character: &'a crate::Character,
        acting_team: crate::TeamSide,
        player_team: &'a crate::Team,
        enemy_team: &'a crate::Team,
    ) -> Self {
        Self::TeamBattle {
            acting_character,
            acting_team,
            player_team,
            enemy_team,
        }
    }

    // Legacy constructor for backward compatibility
    pub fn new(acting_character: &'a crate::Character, player: &'a crate::Character, enemy: &'a crate::Character) -> Self {
        Self::SingleCharacter {
            acting_character,
            player,
            enemy,
        }
    }
    
    pub fn all_characters(&self) -> Vec<&'a crate::Character> {
        match self {
            Self::SingleCharacter { player, enemy, .. } => vec![*player, *enemy],
            Self::TeamBattle { player_team, enemy_team, .. } => {
                let mut characters = Vec::new();
                characters.extend(player_team.members.iter());
                characters.extend(enemy_team.members.iter());
                characters
            }
        }
    }

    pub fn get_team_members(&self, team: crate::TeamSide) -> Vec<&'a crate::Character> {
        match self {
            Self::SingleCharacter { player, enemy, .. } => {
                match team {
                    crate::TeamSide::Player => vec![*player],
                    crate::TeamSide::Enemy => vec![*enemy],
                }
            }
            Self::TeamBattle { player_team, enemy_team, .. } => {
                match team {
                    crate::TeamSide::Player => player_team.members.iter().collect(),
                    crate::TeamSide::Enemy => enemy_team.members.iter().collect(),
                }
            }
        }
    }

    pub fn get_alive_team_members(&self, team: crate::TeamSide) -> Vec<&'a crate::Character> {
        match self {
            Self::SingleCharacter { player, enemy, .. } => {
                match team {
                    crate::TeamSide::Player => if player.is_alive() { vec![*player] } else { vec![] },
                    crate::TeamSide::Enemy => if enemy.is_alive() { vec![*enemy] } else { vec![] },
                }
            }
            Self::TeamBattle { player_team, enemy_team, .. } => {
                match team {
                    crate::TeamSide::Player => player_team.alive_members(),
                    crate::TeamSide::Enemy => enemy_team.alive_members(),
                }
            }
        }
    }

    pub fn get_opposing_team(&self, team: crate::TeamSide) -> crate::TeamSide {
        match team {
            crate::TeamSide::Player => crate::TeamSide::Enemy,
            crate::TeamSide::Enemy => crate::TeamSide::Player,
        }
    }
    
    pub fn get_acting_character(&self) -> &'a crate::Character {
        match self {
            Self::SingleCharacter { acting_character, .. } => *acting_character,
            Self::TeamBattle { acting_character, .. } => *acting_character,
        }
    }

    pub fn get_acting_team(&self) -> crate::TeamSide {
        match self {
            Self::SingleCharacter { acting_character, player, .. } => {
                if std::ptr::eq(*acting_character, *player) {
                    crate::TeamSide::Player
                } else {
                    crate::TeamSide::Enemy
                }
            }
            Self::TeamBattle { acting_team, .. } => *acting_team,
        }
    }

    // Backward compatibility methods
    pub fn player(&self) -> &'a crate::Character {
        match self {
            Self::SingleCharacter { player, .. } => *player,
            Self::TeamBattle { player_team, .. } => {
                player_team.alive_members().first().copied().unwrap_or(&player_team.members[0])
            }
        }
    }

    pub fn enemy(&self) -> &'a crate::Character {
        match self {
            Self::SingleCharacter { enemy, .. } => *enemy,
            Self::TeamBattle { enemy_team, .. } => {
                enemy_team.alive_members().first().copied().unwrap_or(&enemy_team.members[0])
            }
        }
    }
}

// Trait for nodes that evaluate to character references
pub trait CharacterNode: Send + Sync + std::fmt::Debug {
    fn evaluate<'a>(&self, battle_context: &BattleContext<'a>, rng: &mut dyn rand::RngCore) -> &'a crate::Character;
}

impl CharacterNode for Box<dyn CharacterNode> {
    fn evaluate<'a>(&self, battle_context: &BattleContext<'a>, rng: &mut dyn rand::RngCore) -> &'a crate::Character {
        (**self).evaluate(battle_context, rng)
    }
}

// Re-export individual character node modules
pub use super::acting_character_node::ActingCharacterNode;
pub use super::random_character_node::RandomCharacterNode;