// Character nodes - nodes that evaluate to character references for calculations

// Trait for nodes that evaluate to character references
pub trait CharacterNode: Send + Sync + std::fmt::Debug {
    fn evaluate<'a>(&self, character: &'a crate::Character, rng: &mut dyn rand::RngCore) -> &'a crate::Character;
}

impl CharacterNode for Box<dyn CharacterNode> {
    fn evaluate<'a>(&self, character: &'a crate::Character, rng: &mut dyn rand::RngCore) -> &'a crate::Character {
        (**self).evaluate(character, rng)
    }
}

// Re-export individual character node modules
pub use crate::acting_character_node::ActingCharacterNode;