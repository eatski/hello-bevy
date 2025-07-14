use crate::{StructuredTokenInput, node_converter::{NodeConverter, ConverterRegistry, matches_token}};
use action_system::*;

// Base converters
pub struct AllCharactersConverter;

impl NodeConverter<Vec<Character>> for AllCharactersConverter {
    fn can_convert(&self, token: &StructuredTokenInput) -> bool {
        matches_token(token, "AllCharacters")
    }
    
    fn convert(&self, _token: &StructuredTokenInput, _registry: &ConverterRegistry) -> Result<Box<dyn Node<Vec<Character>>>, String> {
        Ok(Box::new(AllCharactersNode::new()))
    }
}

pub struct TeamMembersConverter;

impl NodeConverter<Vec<Character>> for TeamMembersConverter {
    fn can_convert(&self, token: &StructuredTokenInput) -> bool {
        matches_token(token, "TeamMembers")
    }
    
    fn convert(&self, token: &StructuredTokenInput, registry: &ConverterRegistry) -> Result<Box<dyn Node<Vec<Character>>>, String> {
        if let StructuredTokenInput::TeamMembers { team_side } = token {
            // Try to get a static TeamSide value first
            match team_side.as_ref() {
                StructuredTokenInput::Enemy => Ok(Box::new(TeamMembersNode::new(TeamSide::Enemy))),
                StructuredTokenInput::Hero => Ok(Box::new(TeamMembersNode::new(TeamSide::Player))),
                _ => {
                    // Dynamic TeamSide evaluation
                    let team_side_node = registry.convert::<TeamSide>(team_side)?;
                    Ok(Box::new(TeamMembersNode::new_with_node(team_side_node)))
                }
            }
        } else {
            Err("Expected TeamMembers token".to_string())
        }
    }
}

pub struct AllTeamSidesConverter;

impl NodeConverter<Vec<TeamSide>> for AllTeamSidesConverter {
    fn can_convert(&self, token: &StructuredTokenInput) -> bool {
        matches_token(token, "AllTeamSides")
    }
    
    fn convert(&self, _token: &StructuredTokenInput, _registry: &ConverterRegistry) -> Result<Box<dyn Node<Vec<TeamSide>>>, String> {
        Ok(Box::new(AllTeamSidesNode::new()))
    }
}

// RandomPick converters for different types
pub struct RandomPickCharacterConverter;

impl NodeConverter<Character> for RandomPickCharacterConverter {
    fn can_convert(&self, token: &StructuredTokenInput) -> bool {
        if let StructuredTokenInput::RandomPick { array } = token {
            // Check if the array would produce Vec<Character>
            matches_token(array, "AllCharacters") || matches_token(array, "TeamMembers") || matches_token(array, "FilterList")
        } else {
            false
        }
    }
    
    fn convert(&self, token: &StructuredTokenInput, registry: &ConverterRegistry) -> Result<Box<dyn Node<Character>>, String> {
        if let StructuredTokenInput::RandomPick { array } = token {
            let array_node = registry.convert::<Vec<Character>>(array)?;
            Ok(Box::new(CharacterRandomPickNode::new(array_node)))
        } else {
            Err("Expected RandomPick token".to_string())
        }
    }
}

// FilterList converters
pub struct FilterListCharacterConverter;

impl NodeConverter<Vec<Character>> for FilterListCharacterConverter {
    fn can_convert(&self, token: &StructuredTokenInput) -> bool {
        if let StructuredTokenInput::FilterList { array, .. } = token {
            matches_token(array, "AllCharacters") || matches_token(array, "TeamMembers")
        } else {
            false
        }
    }
    
    fn convert(&self, token: &StructuredTokenInput, registry: &ConverterRegistry) -> Result<Box<dyn Node<Vec<Character>>>, String> {
        if let StructuredTokenInput::FilterList { array, condition } = token {
            let array_node = registry.convert::<Vec<Character>>(array)?;
            let condition_node = registry.convert::<bool>(condition)?;
            Ok(Box::new(FilterListNode::new(array_node, condition_node)))
        } else {
            Err("Expected FilterList token".to_string())
        }
    }
}

// Map converters
pub struct MapCharacterToHpConverter;

impl NodeConverter<Vec<CharacterHP>> for MapCharacterToHpConverter {
    fn can_convert(&self, token: &StructuredTokenInput) -> bool {
        if let StructuredTokenInput::Map { transform, .. } = token {
            matches_token(transform, "CharacterToHp")
        } else {
            false
        }
    }
    
    fn convert(&self, token: &StructuredTokenInput, registry: &ConverterRegistry) -> Result<Box<dyn Node<Vec<CharacterHP>>>, String> {
        if let StructuredTokenInput::Map { array, .. } = token {
            let array_node = registry.convert::<Vec<Character>>(array)?;
            let transform_node = Box::new(CharacterToHpNode::new(Box::new(ElementNode::new())));
            Ok(Box::new(MappingNode::new(array_node, transform_node)))
        } else {
            Err("Expected Map token".to_string())
        }
    }
}

pub struct MapCharacterToI32Converter;

impl NodeConverter<Vec<i32>> for MapCharacterToI32Converter {
    fn can_convert(&self, token: &StructuredTokenInput) -> bool {
        if let StructuredTokenInput::Map { array, transform } = token {
            // Check if mapping Characters to i32 (e.g., via HP extraction)
            (matches_token(array, "AllCharacters") || matches_token(array, "TeamMembers")) &&
            !matches_token(transform, "CharacterToHp") // This would produce CharacterHP, not i32
        } else {
            false
        }
    }
    
    fn convert(&self, token: &StructuredTokenInput, registry: &ConverterRegistry) -> Result<Box<dyn Node<Vec<i32>>>, String> {
        if let StructuredTokenInput::Map { array, transform } = token {
            let array_node = registry.convert::<Vec<Character>>(array)?;
            let transform_node = registry.convert::<i32>(transform)?;
            Ok(Box::new(MappingNode::new(array_node, transform_node)))
        } else {
            Err("Expected Map token".to_string())
        }
    }
}

// Max/Min converters
pub struct MaxI32Converter;

impl NodeConverter<i32> for MaxI32Converter {
    fn can_convert(&self, token: &StructuredTokenInput) -> bool {
        matches_token(token, "Max") || matches_token(token, "NumericMax")
    }
    
    fn convert(&self, token: &StructuredTokenInput, registry: &ConverterRegistry) -> Result<Box<dyn Node<i32>>, String> {
        match token {
            StructuredTokenInput::Max { array } | StructuredTokenInput::NumericMax { array } => {
                // Try to convert as Vec<i32>
                if let Ok(array_node) = registry.convert::<Vec<i32>>(array) {
                    Ok(Box::new(MaxNodeI32::new(array_node)))
                } else {
                    Err("Max requires Vec<i32> array".to_string())
                }
            }
            _ => Err("Expected Max or NumericMax token".to_string())
        }
    }
}

pub struct MinI32Converter;

impl NodeConverter<i32> for MinI32Converter {
    fn can_convert(&self, token: &StructuredTokenInput) -> bool {
        matches_token(token, "Min") || matches_token(token, "NumericMin")
    }
    
    fn convert(&self, token: &StructuredTokenInput, registry: &ConverterRegistry) -> Result<Box<dyn Node<i32>>, String> {
        match token {
            StructuredTokenInput::Min { array } | StructuredTokenInput::NumericMin { array } => {
                // Try to convert as Vec<i32>
                if let Ok(array_node) = registry.convert::<Vec<i32>>(array) {
                    Ok(Box::new(MinNodeI32::new(array_node)))
                } else {
                    Err("Min requires Vec<i32> array".to_string())
                }
            }
            _ => Err("Expected Min or NumericMin token".to_string())
        }
    }
}

pub struct MaxCharacterHPConverter;

impl NodeConverter<CharacterHP> for MaxCharacterHPConverter {
    fn can_convert(&self, token: &StructuredTokenInput) -> bool {
        if let StructuredTokenInput::Max { array } | StructuredTokenInput::NumericMax { array } = token {
            // Check if array produces Vec<CharacterHP>
            if let StructuredTokenInput::Map { transform, .. } = array.as_ref() {
                matches_token(transform, "CharacterToHp")
            } else {
                false
            }
        } else {
            false
        }
    }
    
    fn convert(&self, token: &StructuredTokenInput, registry: &ConverterRegistry) -> Result<Box<dyn Node<CharacterHP>>, String> {
        match token {
            StructuredTokenInput::Max { array } | StructuredTokenInput::NumericMax { array } => {
                let array_node = registry.convert::<Vec<CharacterHP>>(array)?;
                Ok(Box::new(MaxNode::<CharacterHP>::new(array_node)))
            }
            _ => Err("Expected Max or NumericMax token".to_string())
        }
    }
}

pub struct MinCharacterHPConverter;

impl NodeConverter<CharacterHP> for MinCharacterHPConverter {
    fn can_convert(&self, token: &StructuredTokenInput) -> bool {
        if let StructuredTokenInput::Min { array } | StructuredTokenInput::NumericMin { array } = token {
            // Check if array produces Vec<CharacterHP>
            if let StructuredTokenInput::Map { transform, .. } = array.as_ref() {
                matches_token(transform, "CharacterToHp")
            } else {
                false
            }
        } else {
            false
        }
    }
    
    fn convert(&self, token: &StructuredTokenInput, registry: &ConverterRegistry) -> Result<Box<dyn Node<CharacterHP>>, String> {
        match token {
            StructuredTokenInput::Min { array } | StructuredTokenInput::NumericMin { array } => {
                let array_node = registry.convert::<Vec<CharacterHP>>(array)?;
                Ok(Box::new(MinNode::<CharacterHP>::new(array_node)))
            }
            _ => Err("Expected Min or NumericMin token".to_string())
        }
    }
}