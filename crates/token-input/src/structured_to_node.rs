// StructuredTokenInput → Node 変換

use crate::{StructuredTokenInput, RuleSet};
use action_system::{RuleNode, Character, Node, Action, TeamSide};
use std::any::Any;

// パース結果を表すAnyベースのResolver
pub struct ParsedResolver {
    pub node: Box<dyn Any>,
    pub type_name: String,
}

impl ParsedResolver {
    pub fn new<T: Any + 'static>(node: T, type_name: String) -> Self {
        Self {
            node: Box::new(node),
            type_name,
        }
    }
}

// StructuredTokenInput → Node 変換
pub fn convert_structured_to_node(token: &StructuredTokenInput) -> Result<ParsedResolver, String> {
    todo!()
}

// RuleSet → Vec<RuleNode> 変換（JSON入力経路）
pub fn convert_ruleset_to_nodes(ruleset: &RuleSet) -> Vec<RuleNode> {
    todo!()
}