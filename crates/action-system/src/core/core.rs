// Core traits and types for the action system

// Common error type for all resolvers and nodes
#[derive(Clone, Debug, PartialEq)]
pub enum NodeError {
    // General evaluation error
    EvaluationError(String),
    // Action resolution should break/skip this rule
    Break,
}

impl std::fmt::Display for NodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeError::EvaluationError(msg) => write!(f, "Evaluation error: {}", msg),
            NodeError::Break => write!(f, "Action resolution break"),
        }
    }
}

impl std::error::Error for NodeError {}

// Type alias for Result with NodeError
pub type NodeResult<T> = Result<T, NodeError>;

// Trait for nodes that can resolve to actions or break
pub trait ActionResolver: Send + Sync + std::fmt::Debug {
    fn resolve(&self, battle_context: &crate::BattleContext, rng: &mut dyn rand::RngCore) -> NodeResult<ActionType>;
}

impl ActionResolver for Box<dyn ActionResolver> {
    fn resolve(&self, battle_context: &crate::BattleContext, rng: &mut dyn rand::RngCore) -> NodeResult<ActionType> {
        (**self).resolve(battle_context, rng)
    }
}


#[derive(Clone, Debug, PartialEq)]
pub enum ActionType {
    Strike,
    Heal,
}

// Simplified rule system - all nodes are ActionResolvers
pub type RuleNode = Box<dyn ActionResolver>;