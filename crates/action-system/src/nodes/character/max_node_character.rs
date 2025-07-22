use crate::{Node, EvaluationContext, Character, NodeError, NodeResult};

/// Character配列から最大HP/攻撃力を持つキャラクターを選択するノード
pub struct MaxNodeCharacter {
    array_node: Box<dyn for<'a> Node<Vec<Character>, EvaluationContext<'a>> + Send + Sync>,
}

impl MaxNodeCharacter {
    pub fn new(array_node: Box<dyn for<'a> Node<Vec<Character>, EvaluationContext<'a>> + Send + Sync>) -> Self {
        Self { array_node }
    }
}

impl<'a> Node<Character, EvaluationContext<'a>> for MaxNodeCharacter {
    fn evaluate(&self, context: &mut EvaluationContext<'a>) -> NodeResult<Character> {
        let characters = self.array_node.evaluate(context)?;
        
        // 最大HPを持つキャラクターを返す
        characters.into_iter()
            .max_by_key(|c| c.hp)
            .ok_or_else(|| NodeError::EvaluationError("No characters in array".to_string()))
    }
}