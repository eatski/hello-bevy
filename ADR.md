# 未実装のアーキテクチャデザイン

## ElementNodeを廃止し、Node(Fn)の引数として渡す

- ElementNodeを廃止
  - EvaluationContext#current_elementも廃止
- 入力側のElementトークンは生かす
  - `FilterList -> AllCharacter -> GraterThan -> CharacterHp -> Element -> Number(30)` のようにFilterListの引数をElementで参照できるように
  - `FilterList -> AllCharacter -> GraterThan -> CharacterHp -> Element -> Number(30)` が `FilterList {array: AllCharacter, condition: (element) => gt(hp(element),30)}` に変換される
