// StructuredTokenInput → Node 変換 (extensible design)

use crate::{StructuredTokenInput, RuleSet};
use crate::node_converter::ConverterRegistry;
use action_system::{RuleNode, Action};
use std::any::Any;

// Legacy compatibility - keep the same public interface
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

// Global converter registry (thread-safe singleton would be better in production)
thread_local! {
    static CONVERTER_REGISTRY: ConverterRegistry = ConverterRegistry::new();
}

// Main conversion function using the new extensible system
pub fn convert_structured_to_node(token: &StructuredTokenInput) -> Result<ParsedResolver, String> {
    CONVERTER_REGISTRY.with(|registry| {
        let typed_node = registry.convert_typed(token)?;
        
        // Convert TypedNode to legacy ParsedResolver format
        Ok(ParsedResolver {
            node: typed_node.node,
            type_name: typed_node.type_name,
        })
    })
}

// RuleSet → Vec<RuleNode> conversion
pub fn convert_ruleset_to_nodes(ruleset: &RuleSet) -> Vec<RuleNode> {
    CONVERTER_REGISTRY.with(|registry| {
        ruleset.rules.iter()
            .filter_map(|token| {
                registry.convert::<Box<dyn Action>>(token)
                    .ok()
                    .map(|action_node| action_node as RuleNode)
            })
            .collect()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_extensible_conversion() {
        // Test that the new system produces the same results
        let strike_token = StructuredTokenInput::Strike {
            target: Box::new(StructuredTokenInput::ActingCharacter),
        };
        
        let result = convert_structured_to_node(&strike_token).unwrap();
        assert_eq!(result.type_name, "Action");
    }
    
    #[test]
    fn test_complex_conversion() {
        let complex_token = StructuredTokenInput::Check {
            condition: Box::new(StructuredTokenInput::GreaterThan {
                left: Box::new(StructuredTokenInput::CharacterToHp {
                    character: Box::new(StructuredTokenInput::ActingCharacter),
                }),
                right: Box::new(StructuredTokenInput::Number { value: 50 }),
            }),
            then_action: Box::new(StructuredTokenInput::Strike {
                target: Box::new(StructuredTokenInput::RandomPick {
                    array: Box::new(StructuredTokenInput::AllCharacters),
                }),
            }),
        };
        
        let result = convert_structured_to_node(&complex_token).unwrap();
        assert_eq!(result.type_name, "Action");
    }
}