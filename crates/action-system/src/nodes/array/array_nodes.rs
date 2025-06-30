// Array node traits - nodes that evaluate to collections of characters

use crate::core::NodeResult;
use crate::nodes::character::BattleContext;

// Trait for nodes that evaluate to arrays of characters
pub trait CharacterArrayNode: Send + Sync + std::fmt::Debug {
    fn evaluate(&self, battle_context: &BattleContext, rng: &mut dyn rand::RngCore) -> NodeResult<Vec<crate::Character>>;
}

impl CharacterArrayNode for Box<dyn CharacterArrayNode> {
    fn evaluate(&self, battle_context: &BattleContext, rng: &mut dyn rand::RngCore) -> NodeResult<Vec<crate::Character>> {
        (**self).evaluate(battle_context, rng)
    }
}