// AllTeamSidesNode - returns all possible team sides
use crate::core::NodeResult;
use crate::nodes::unified_node::Node;
use crate::nodes::evaluation_context::EvaluationContext;
use crate::TeamSide;

/// Node that returns an array of both team sides
/// This is useful for operations that need to work with both Player and Enemy teams
#[derive(Debug)]
pub struct AllTeamSidesNode;

impl AllTeamSidesNode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for AllTeamSidesNode {
    fn default() -> Self {
        Self::new()
    }
}

impl Node<Vec<TeamSide>> for AllTeamSidesNode {
    fn evaluate(&self, _eval_context: &mut EvaluationContext) -> NodeResult<Vec<TeamSide>> {
        Ok(vec![TeamSide::Player, TeamSide::Enemy])
    }
}

