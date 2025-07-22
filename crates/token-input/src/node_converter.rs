// Node converter trait system for extensible conversion

use crate::StructuredTokenInput;
use action_system::*;
use std::any::{Any, TypeId};
use std::collections::HashMap;

// Type alias for Node trait with action-system's EvaluationContext
type ActionSystemNode<T> = dyn for<'a> Node<T, EvaluationContext<'a>> + Send + Sync;

// Type-erased node wrapper that preserves type information
pub struct TypedNode {
    pub node: Box<dyn Any>,
    pub type_id: TypeId,
    pub type_name: String,
}

impl TypedNode {
    pub fn new<T: Any + 'static>(node: Box<ActionSystemNode<T>>, type_name: String) -> Self {
        Self {
            node: Box::new(node) as Box<dyn Any>,
            type_id: TypeId::of::<T>(),
            type_name,
        }
    }
    
    pub fn downcast<T: Any + 'static>(self) -> Result<Box<ActionSystemNode<T>>, String> {
        if self.type_id == TypeId::of::<T>() {
            self.node.downcast::<Box<ActionSystemNode<T>>>()
                .map(|boxed| *boxed)
                .map_err(|_| format!("Failed to downcast to {}", std::any::type_name::<T>()))
        } else {
            Err(format!("Type mismatch: expected {}, got {}", std::any::type_name::<T>(), self.type_name))
        }
    }
}

// Trait for converting StructuredTokenInput to specific node types
pub trait NodeConverter<T> {
    fn can_convert(&self, token: &StructuredTokenInput) -> bool;
    fn convert(&self, token: &StructuredTokenInput, registry: &ConverterRegistry) -> Result<Box<ActionSystemNode<T>>, String>;
}

// Registry to hold all converters
pub struct ConverterRegistry {
    converters: HashMap<TypeId, Vec<Box<dyn Any>>>,
}

impl ConverterRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            converters: HashMap::new(),
        };
        
        // Register default converters
        registry.register_default_converters();
        registry
    }
    
    pub fn register_converter<T: Any + 'static>(&mut self, converter: Box<dyn NodeConverter<T>>) {
        let type_id = TypeId::of::<T>();
        self.converters.entry(type_id)
            .or_insert_with(Vec::new)
            .push(Box::new(converter));
    }
    
    pub fn convert<T: Any + 'static>(&self, token: &StructuredTokenInput) -> Result<Box<ActionSystemNode<T>>, String> {
        let type_id = TypeId::of::<T>();
        
        if let Some(converters) = self.converters.get(&type_id) {
            for converter_any in converters {
                if let Some(converter) = converter_any.downcast_ref::<Box<dyn NodeConverter<T>>>() {
                    if converter.can_convert(token) {
                        return converter.convert(token, self);
                    }
                }
            }
        }
        
        Err(format!("No converter found for token {:?} to type {}", token, std::any::type_name::<T>()))
    }
    
    pub fn convert_typed(&self, token: &StructuredTokenInput) -> Result<TypedNode, String> {
        // Try each type in order of likelihood
        
        // Action nodes
        if let Ok(node) = self.convert::<Box<dyn Action>>(token) {
            return Ok(TypedNode::new(node, "Action".to_string()));
        }
        
        // Condition nodes
        if let Ok(node) = self.convert::<bool>(token) {
            return Ok(TypedNode::new(node, "bool".to_string()));
        }
        
        // Value nodes
        if let Ok(node) = self.convert::<i32>(token) {
            return Ok(TypedNode::new(node, "i32".to_string()));
        }
        
        // Character nodes
        if let Ok(node) = self.convert::<Character>(token) {
            return Ok(TypedNode::new(node, "Character".to_string()));
        }
        
        // CharacterHP nodes
        if let Ok(node) = self.convert::<CharacterHP>(token) {
            return Ok(TypedNode::new(node, "CharacterHP".to_string()));
        }
        
        // Array nodes
        if let Ok(node) = self.convert::<Vec<Character>>(token) {
            return Ok(TypedNode::new(node, "Vec<Character>".to_string()));
        }
        
        if let Ok(node) = self.convert::<Vec<CharacterHP>>(token) {
            return Ok(TypedNode::new(node, "Vec<CharacterHP>".to_string()));
        }
        
        if let Ok(node) = self.convert::<Vec<i32>>(token) {
            return Ok(TypedNode::new(node, "Vec<i32>".to_string()));
        }
        
        if let Ok(node) = self.convert::<Vec<TeamSide>>(token) {
            return Ok(TypedNode::new(node, "Vec<TeamSide>".to_string()));
        }
        
        // TeamSide nodes
        if let Ok(node) = self.convert::<TeamSide>(token) {
            return Ok(TypedNode::new(node, "TeamSide".to_string()));
        }
        
        Err(format!("Unable to convert token: {:?}", token))
    }
    
    fn register_default_converters(&mut self) {
        // Register all default converters
        use crate::node_converters::*;
        
        // Action converters
        self.register_converter(Box::new(StrikeActionConverter));
        self.register_converter(Box::new(HealActionConverter));
        self.register_converter(Box::new(CheckActionConverter));
        
        // Condition converters
        self.register_converter(Box::new(TrueOrFalseRandomConverter));
        self.register_converter(Box::new(GreaterThanConverter));
        self.register_converter(Box::new(EqConverter));
        
        // Value converters
        self.register_converter(Box::new(NumberConverter));
        
        // Character converters
        self.register_converter(Box::new(ActingCharacterConverter));
        self.register_converter(Box::new(ElementConverter));
        self.register_converter(Box::new(CharacterHpToCharacterConverter));
        
        // CharacterHP converters
        self.register_converter(Box::new(CharacterToHpConverter));
        
        // Array converters
        self.register_converter(Box::new(AllCharactersConverter));
        self.register_converter(Box::new(TeamMembersConverter));
        self.register_converter(Box::new(AllTeamSidesConverter));
        self.register_converter(Box::new(RandomPickCharacterConverter));
        self.register_converter(Box::new(FilterListCharacterConverter));
        self.register_converter(Box::new(MapCharacterToHpConverter));
        self.register_converter(Box::new(MapCharacterToI32Converter));
        
        // TeamSide converters
        self.register_converter(Box::new(EnemyConverter));
        self.register_converter(Box::new(HeroConverter));
        self.register_converter(Box::new(CharacterTeamConverter));
        
        // Max/Min converters
        self.register_converter(Box::new(MaxI32Converter));
        self.register_converter(Box::new(MinI32Converter));
        self.register_converter(Box::new(MaxCharacterHPConverter));
        self.register_converter(Box::new(MinCharacterHPConverter));
        self.register_converter(Box::new(MaxCharacterConverter));
        self.register_converter(Box::new(MinCharacterConverter));
    }
}

// Helper function to check if a token matches a specific variant
pub fn matches_token(token: &StructuredTokenInput, expected: &str) -> bool {
    match (token, expected) {
        (StructuredTokenInput::Strike { .. }, "Strike") => true,
        (StructuredTokenInput::Heal { .. }, "Heal") => true,
        (StructuredTokenInput::Check { .. }, "Check") => true,
        (StructuredTokenInput::TrueOrFalseRandom, "TrueOrFalseRandom") => true,
        (StructuredTokenInput::GreaterThan { .. }, "GreaterThan") => true,
        (StructuredTokenInput::Eq { .. }, "Eq") => true,
        (StructuredTokenInput::Number { .. }, "Number") => true,
        (StructuredTokenInput::CharacterToHp { .. }, "CharacterToHp") => true,
        (StructuredTokenInput::CharacterHpToCharacter { .. }, "CharacterHpToCharacter") => true,
        (StructuredTokenInput::ActingCharacter, "ActingCharacter") => true,
        (StructuredTokenInput::AllCharacters, "AllCharacters") => true,
        (StructuredTokenInput::TeamMembers { .. }, "TeamMembers") => true,
        (StructuredTokenInput::AllTeamSides, "AllTeamSides") => true,
        (StructuredTokenInput::RandomPick { .. }, "RandomPick") => true,
        (StructuredTokenInput::FilterList { .. }, "FilterList") => true,
        (StructuredTokenInput::Map { .. }, "Map") => true,
        (StructuredTokenInput::CharacterTeam { .. }, "CharacterTeam") => true,
        (StructuredTokenInput::Element, "Element") => true,
        (StructuredTokenInput::Enemy, "Enemy") => true,
        (StructuredTokenInput::Hero, "Hero") => true,
        (StructuredTokenInput::Max { .. }, "Max") => true,
        (StructuredTokenInput::Min { .. }, "Min") => true,
        (StructuredTokenInput::NumericMax { .. }, "NumericMax") => true,
        (StructuredTokenInput::NumericMin { .. }, "NumericMin") => true,
        _ => false,
    }
}