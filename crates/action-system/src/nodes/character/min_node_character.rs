use crate::{Node, EvaluationContext, Character, NodeError, NodeResult};

/// Character配列から最小HP/攻撃力を持つキャラクターを選択するノード
pub struct MinNodeCharacter {
    array_node: Box<dyn for<'a> Node<Vec<Character>, EvaluationContext<'a>> + Send + Sync>,
}

impl MinNodeCharacter {
    pub fn new(array_node: Box<dyn for<'a> Node<Vec<Character>, EvaluationContext<'a>> + Send + Sync>) -> Self {
        Self { array_node }
    }
}

impl<'a> Node<Character, EvaluationContext<'a>> for MinNodeCharacter {
    fn evaluate(&self, context: &mut EvaluationContext<'a>) -> NodeResult<Character> {
        let characters = self.array_node.evaluate(context)?;
        
        // 最小HPを持つキャラクターを返す（HP0は除外）
        characters.into_iter()
            .filter(|c| c.hp > 0)
            .min_by_key(|c| c.hp)
            .ok_or_else(|| NodeError::EvaluationError("No alive characters in array".to_string()))
    }
}