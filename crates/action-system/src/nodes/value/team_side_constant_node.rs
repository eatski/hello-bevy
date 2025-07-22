use crate::core::NodeResult;
use crate::nodes::evaluation_context::EvaluationContext;
use crate::nodes::unified_node::CoreNode as Node;
use crate::TeamSide;

#[derive(Debug)]
pub struct EnemyNode;

impl EnemyNode {
    pub fn new() -> Self {
        Self
    }
}

impl<'a> Node<TeamSide, EvaluationContext<'a>> for EnemyNode {
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

impl<'a> Node<TeamSide, EvaluationContext<'a>> for HeroNode {
    fn evaluate(&self, _context: &mut EvaluationContext) -> NodeResult<TeamSide> {
        Ok(TeamSide::Player)
    }
}