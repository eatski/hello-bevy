// Core traits and types for the action system

// Trait for nodes that can resolve to actions or break
pub trait ActionResolver: Send + Sync + std::fmt::Debug {
    fn resolve(&self, battle_context: &crate::BattleContext, rng: &mut dyn rand::RngCore) -> ActionResolverResult;
}

impl ActionResolver for Box<dyn ActionResolver> {
    fn resolve(&self, battle_context: &crate::BattleContext, rng: &mut dyn rand::RngCore) -> ActionResolverResult {
        (**self).resolve(battle_context, rng)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ActionResolverResult {
    Action(ActionType),  // 行はActionを決定
    Break,               // 行を中断
}

#[derive(Clone, Debug, PartialEq)]
pub enum ActionType {
    Strike,
    Heal,
}

// Simplified rule system - all nodes are ActionResolvers
pub type RuleNode = Box<dyn ActionResolver>;