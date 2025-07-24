//! 新しいトークンを追加する例
//! 
//! このファイルは、新しいトークンの追加がどれだけ簡単になったかを示すデモです。

use crate::type_system::Type;
use crate::token_definition_macro::impl_token_converter;
use action_system::{Character, NodeRegistry};
use node_core::Node;

// =====================================================
// ステップ1: トークンの定義
// =====================================================
// TokenMetadataRegistryが削除されたため、define_token!マクロは使用されない
// 新しいトークンはStructuredTokenInputに直接バリアントを追加することで定義される

// =====================================================
// ステップ2: 実行ノードの実装
// =====================================================
pub struct DoubleStrikeNode {
    target: Box<dyn Node<Character>>,
    multiplier: Box<dyn Node<i32>>,
}

impl DoubleStrikeNode {
    pub fn new(target: Box<dyn Node<Character>>, multiplier: Box<dyn Node<i32>>) -> Self {
        Self { target, multiplier }
    }
}

impl Node<action_system::ActionResult, NodeRegistry> for DoubleStrikeNode {
    fn execute(&self, registry: &NodeRegistry) -> action_system::ActionResult {
        let target = self.target.execute(registry);
        let multiplier = self.multiplier.execute(registry);
        
        // 2回攻撃を実行
        for _ in 0..multiplier {
            // 実際の攻撃処理（簡略化）
            println!("Double Strike on {:?}!", target);
        }
        
        action_system::ActionResult::Continue
    }
}

// =====================================================
// ステップ3: コンバーターの実装（マクロで簡略化）
// =====================================================
impl_token_converter! {
    DoubleStrike<action_system::ActionResult> {
        args: { target: Character, multiplier: i32 },
        convert: |target, multiplier| Box::new(DoubleStrikeNode::new(target, multiplier))
    }
}

// =====================================================
// ステップ4: StructuredTokenInputへの追加
// =====================================================
// 注: 実際の実装では、StructuredTokenInputのenumに以下を追加する必要があります：
// ```
// DoubleStrike {
//     target: Box<StructuredTokenInput>,
//     multiplier: Box<StructuredTokenInput>,
// }
// ```

// =====================================================
// ステップ5: FlatTokenInputへの追加（オプション）
// =====================================================
// UIから使用する場合は、FlatTokenInputにも同様に追加

// =====================================================
// 使用例
// =====================================================
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_token_registration() {
        // TokenMetadataRegistryが削除されたため、
        // 新しいトークンの型情報はStructuredTokenInputから直接取得される
        
        // DoubleStrikeトークンがコンバーターレジストリに登録されていることを確認
        // 実際の使用では、StructuredTokenInputに新しいバリアントを追加し、
        // expected_argument_types()とoutput_type()メソッドを実装することで
        // 型システムに統合される
    }
}

// =====================================================
// 従来の方法との比較
// =====================================================
// 
// 【従来の方法】
// 1. StructuredTokenInputにenumバリアントを追加
// 2. FlatTokenInputにenumバリアントを追加
// 3. flat_to_structuredに変換ロジックを追加
// 4. token_metadata.rsにメタデータを手動で登録
// 5. type_checker.rsのget_token_typeとextract_argumentsを更新
// 6. TypedTokenにバリアントを追加
// 7. typed_code_generator.rsに変換ロジックを追加
// 8. typed_*_converters.rsにコンバーターを実装
// 9. typed_converter_registry.rsにコンバーターを登録
// 
// 【新しい方法】
// 1. define_token!マクロでトークンを定義（メタデータ自動生成）
// 2. Nodeを実装
// 3. impl_token_converter!マクロでコンバーターを定義
// 4. StructuredTokenInput/FlatTokenInputにバリアントを追加
// 
// 変更箇所が9箇所から4箇所に削減！