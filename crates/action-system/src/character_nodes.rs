// Character nodes - nodes that evaluate to character references for calculations

// Battle context containing all characters in the battle including the acting character
#[derive(Debug)]
pub struct BattleContext<'a> {
    pub acting_character: &'a crate::Character,
    pub player: &'a crate::Character,
    pub enemy: &'a crate::Character,
}

impl<'a> BattleContext<'a> {
    pub fn new(acting_character: &'a crate::Character, player: &'a crate::Character, enemy: &'a crate::Character) -> Self {
        Self { acting_character, player, enemy }
    }
    
    pub fn all_characters(&self) -> Vec<&'a crate::Character> {
        vec![self.player, self.enemy]
    }
    
    pub fn get_acting_character(&self) -> &'a crate::Character {
        self.acting_character
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
pub use crate::acting_character_node::ActingCharacterNode;
pub use crate::random_character_node::RandomCharacterNode;