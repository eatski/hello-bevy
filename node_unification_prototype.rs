// Node<T>統一化のプロトタイプ実装
use std::fmt::Debug;

// 統一されたNodeトレイト
pub trait Node<T>: Send + Sync + Debug {
    fn evaluate(&self, eval_context: &EvaluationContext, rng: &mut dyn rand::RngCore) -> NodeResult<T>;
}

// 型エイリアス（後方互換性）
pub type CharacterNode = dyn Node<i32>;
pub type ValueNode = dyn Node<i32>;  
pub type ConditionNode = dyn Node<bool>;
pub type CharacterArrayNode = dyn Node<Vec<Character>>;

// 型安全性の検証例
#[derive(Debug)]
pub struct ActingCharacterNode;

impl Node<i32> for ActingCharacterNode {
    fn evaluate(&self, eval_context: &EvaluationContext, _rng: &mut dyn rand::RngCore) -> NodeResult<i32> {
        Ok(eval_context.get_battle_context().get_acting_character().id)
    }
}

#[derive(Debug)]
pub struct RandomConditionNode;

impl Node<bool> for RandomConditionNode {
    fn evaluate(&self, _eval_context: &EvaluationContext, rng: &mut dyn rand::RngCore) -> NodeResult<bool> {
        use rand::Rng;
        Ok(rng.gen_bool(0.5))
    }
}

#[derive(Debug)]
pub struct AllCharactersNode;

impl Node<Vec<Character>> for AllCharactersNode {
    fn evaluate(&self, eval_context: &EvaluationContext, _rng: &mut dyn rand::RngCore) -> NodeResult<Vec<Character>> {
        let battle_context = eval_context.get_battle_context();
        let characters = battle_context.all_characters().into_iter().cloned().collect();
        Ok(characters)
    }
}

// ジェネリックRandomPickNode
#[derive(Debug)]
pub struct RandomPickNode<T> {
    array_node: Box<dyn Node<Vec<T>>>,
}

impl<T> RandomPickNode<T> {
    pub fn new(array_node: Box<dyn Node<Vec<T>>>) -> Self {
        Self { array_node }
    }
}

// CharacterRandomPickNode: Character配列からキャラクターIDを返す
impl Node<i32> for RandomPickNode<Character> {
    fn evaluate(&self, eval_context: &EvaluationContext, rng: &mut dyn rand::RngCore) -> NodeResult<i32> {
        use rand::Rng;
        let characters = self.array_node.evaluate(eval_context, rng)?;
        if characters.is_empty() {
            return Err(NodeError::EvaluationError("Cannot pick from empty character array".to_string()));
        }
        let index = rng.gen_range(0..characters.len());
        Ok(characters[index].id)
    }
}

// ValueRandomPickNode: 数値配列から数値を返す
impl Node<i32> for RandomPickNode<i32> {
    fn evaluate(&self, eval_context: &EvaluationContext, rng: &mut dyn rand::RngCore) -> NodeResult<i32> {
        use rand::Rng;
        let values = self.array_node.evaluate(eval_context, rng)?;
        if values.is_empty() {
            return Err(NodeError::EvaluationError("Cannot pick from empty value array".to_string()));
        }
        let index = rng.gen_range(0..values.len());
        Ok(values[index])
    }
}

// 使用例での型安全性
pub fn usage_examples() {
    // ✅ 型安全: 正しい型の組み合わせ
    let character_node: Box<dyn Node<i32>> = Box::new(ActingCharacterNode);
    let condition_node: Box<dyn Node<bool>> = Box::new(RandomConditionNode);
    let array_node: Box<dyn Node<Vec<Character>>> = Box::new(AllCharactersNode);
    
    // ✅ ジェネリック型の明示的指定
    let char_pick: RandomPickNode<Character> = RandomPickNode::new(array_node);
    let value_pick: RandomPickNode<i32> = RandomPickNode::new(Box::new(ConstantArrayNode::new(vec![1, 2, 3])));
    
    // ❌ コンパイルエラー: 型の不一致
    // let wrong: Box<dyn Node<String>> = Box::new(ActingCharacterNode); // Error!
    // let wrong2: RandomPickNode<bool> = RandomPickNode::new(array_node); // Error!
}

// メリット分析:
// 1. 統一性: 1つのトレイトですべて
// 2. 型安全性: コンパイル時型チェック
// 3. 拡張性: 新しい型(String, f64など)を簡単に追加
// 4. ジェネリック: RandomPickNode<T>で任意の型に対応

// デメリット分析:
// 1. 複雑性: 型パラメータの明示が必要
// 2. ボックス化: Box<dyn Node<T>>での型消去
// 3. 学習コスト: ジェネリクスの理解が必要

// 結論: メリット > デメリット
// - 型安全性の向上が最大の利点
// - 将来の拡張性が大幅に向上
// - 統一されたAPIで開発効率向上