use crate::core::NodeResult;
use crate::nodes::evaluation_context::EvaluationContext;
use crate::nodes::unified_node::Node;
use crate::TeamSide;

#[derive(Debug)]
pub struct EnemyNode;

impl EnemyNode {
    pub fn new() -> Self {
        Self
    }
}

impl Node<TeamSide> for EnemyNode {
    fn evaluate(&self, _context: &mut EvaluationContext) -> NodeResult<TeamSide> {
        Ok(TeamSide::Enemy)
    }
}

#[derive(Debug)]
pub struct HeroNode;

impl HeroNode {
    pub fn new() -> Self {
        Self
    }
}

impl Node<TeamSide> for HeroNode {
    fn evaluate(&self, _context: &mut EvaluationContext) -> NodeResult<TeamSide> {
        Ok(TeamSide::Player)
    }
}