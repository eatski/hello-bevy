// Node core trait - Generic node evaluation trait for all node types

/// Result type for node evaluations
pub type NodeResult<T> = Result<T, NodeError>;

/// Errors that can occur during node evaluation
#[derive(Debug, Clone, PartialEq)]
pub enum NodeError {
    /// Break condition encountered - stop processing current rule chain
    Break,
    /// Type mismatch error when expecting one type but got another
    TypeMismatch(String),
    /// General evaluation error with a message
    EvaluationError(String),
}

impl std::fmt::Display for NodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeError::Break => write!(f, "Break condition encountered"),
            NodeError::TypeMismatch(msg) => write!(f, "Type mismatch: {}", msg),
            NodeError::EvaluationError(msg) => write!(f, "Evaluation error: {}", msg),
        }
    }
}

impl std::error::Error for NodeError {}

/// Unified generic trait for all node types
/// Replaces CharacterNode, ValueNode, ConditionNode, and ArrayNode<T>
/// 
/// The trait is generic over the evaluation context type to allow
/// different contexts in different domains while maintaining type safety.
pub trait Node<T, TContext>: Send + Sync {
    fn evaluate(&self, eval_context: &mut TContext) -> NodeResult<T>;
}

// Box implementation for trait objects
impl<T, TContext> Node<T, TContext> for Box<dyn Node<T, TContext> + Send + Sync> {
    fn evaluate(&self, eval_context: &mut TContext) -> NodeResult<T> {
        (**self).evaluate(eval_context)
    }
}