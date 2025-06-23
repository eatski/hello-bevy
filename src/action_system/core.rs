// Core traits and types for the action system

// Trait for tokens that can resolve to actions or break
pub trait ActionResolver: Send + Sync + std::fmt::Debug {
    fn resolve(&self, character: &crate::battle_system::Character, rng: &mut dyn rand::RngCore) -> ActionResolverResult;
}

impl ActionResolver for Box<dyn ActionResolver> {
    fn resolve(&self, character: &crate::battle_system::Character, rng: &mut dyn rand::RngCore) -> ActionResolverResult {
        (**self).resolve(character, rng)
    }
}

#[derive(Clone, Debug)]
pub enum ActionResolverResult {
    Action(ActionType),  // 行はActionを決定
    Break,               // 行を中断
}

#[derive(Clone, Debug, PartialEq)]
pub enum ActionType {
    Strike,
    Heal,
}

// Simplified rule system - all tokens are ActionResolvers
pub type RuleToken = Box<dyn ActionResolver>;