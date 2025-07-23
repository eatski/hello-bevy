//! トークン定義マクロ
//!
//! 新しいトークンを簡単に追加するためのマクロシステム

/// トークンを定義するマクロ
/// 
/// # 使用例
/// ```ignore
/// define_token! {
///     Strike { target: Character } -> Action
/// }
/// ```
#[macro_export]
macro_rules! define_token {
    // 単一トークンの定義
    ($name:ident { $($arg_name:ident : $arg_type:expr),* $(,)? } -> $output_type:expr) => {
        define_token!(@register_metadata $name, [$($arg_name : $arg_type),*], $output_type);
    };
    
    // 引数なしトークンの定義
    ($name:ident -> $output_type:expr) => {
        define_token!(@register_metadata $name, [], $output_type);
    };
    
    // メタデータ登録の内部マクロ
    (@register_metadata $name:ident, [$($arg_name:ident : $arg_type:expr),*], $output_type:expr) => {
        paste::paste! {
            /// トークンメタデータの自動登録
            #[allow(non_snake_case)]
            pub fn [<register_ $name _metadata>](registry: &mut $crate::type_system::TokenMetadataRegistry) {
                use $crate::type_system::{TokenMetadata, ArgumentMetadata, Type};
                
                registry.register(TokenMetadata {
                    token_type: stringify!($name).to_string(),
                    arguments: vec![
                        $(
                            ArgumentMetadata {
                                name: stringify!($arg_name).to_string(),
                                expected_type: $arg_type,
                                required: true,
                                default_value: None,
                            },
                        )*
                    ],
                    output_type: $output_type,
                    custom_validator: None,
                    output_type_inference: None,
                    argument_context_provider: None,
                });
            }
        }
    };
}

/// 複数のトークンを一度に定義するマクロ
#[macro_export]
macro_rules! define_tokens {
    (
        $(
            $name:ident $({ $($arg_name:ident : $arg_type:expr),* $(,)? })? -> $output_type:expr
        ),* $(,)?
    ) => {
        $(
            define_tokens!(@single $name $({ $($arg_name : $arg_type),* })? -> $output_type);
        )*
        
        /// すべてのトークンメタデータを登録
        pub fn register_all_token_metadata(registry: &mut $crate::type_system::TokenMetadataRegistry) {
            $(
                paste::paste! {
                    [<register_ $name _metadata>](registry);
                }
            )*
        }
    };
    
    // 単一トークンの処理
    (@single $name:ident { $($arg_name:ident : $arg_type:expr),* } -> $output_type:expr) => {
        define_token!($name { $($arg_name : $arg_type),* } -> $output_type);
    };
    
    (@single $name:ident -> $output_type:expr) => {
        define_token!($name -> $output_type);
    };
}

/// トークンコンバーターを自動生成するマクロ
#[macro_export]
macro_rules! impl_token_converter {
    (
        $name:ident<$output_type:ty> {
            args: { $($arg_name:ident : $arg_type:ty),* $(,)? },
            convert: |$($arg_var:ident),*| $convert_body:expr
        }
    ) => {
        paste::paste! {
            pub struct [<$name Converter>];
            
            impl $crate::typed_node_converter::TypedNodeConverter<$output_type> for [<$name Converter>] {
                fn can_convert(
                    &self,
                    token: &$crate::structured_token::StructuredTokenInput,
                    expected_type: &$crate::type_system::Type,
                ) -> bool {
                    matches!(token, $crate::structured_token::StructuredTokenInput::$name { .. })
                }
                
                fn convert(
                    &self,
                    typed_ast: &$crate::type_system::TypedAst,
                    registry: &dyn $crate::typed_node_converter::TypedConverterRegistry,
                ) -> Result<Box<dyn action_system::Node<$output_type>>, String> {
                    use $crate::typed_node_converter::convert_child;
                    
                    let children = &typed_ast.children;
                    $(
                        let $arg_var = children.get(stringify!($arg_name))
                            .ok_or_else(|| format!("Missing {} argument", stringify!($arg_name)))?;
                        let $arg_var = convert_child::<$arg_type>($arg_var, registry)?;
                    )*
                    
                    Ok($convert_body)
                }
            }
        }
    };
}

/// 使用例（これはテスト用）
#[cfg(test)]
#[allow(dead_code)]
mod tests {
    #[allow(unused_imports)]
    use crate::type_system::{Type, TokenMetadataRegistry};
    
    // 新しいトークンを定義
    define_tokens! {
        // 既存のトークン
        Strike { target: Type::Character } -> Type::Action,
        Heal { target: Type::Character } -> Type::Action,
        
        // 新しいトークンの例
        DoubleStrike { target: Type::Character, power: Type::I32 } -> Type::Action,
        Shield { target: Type::Character } -> Type::Action,
    }
    
    #[test]
    fn test_token_macro() {
        let mut registry = TokenMetadataRegistry::new();
        register_Strike_metadata(&mut registry);
        
        // メタデータが正しく登録されているか確認
        let metadata = registry.get("Strike").unwrap();
        assert_eq!(metadata.token_type, "Strike");
        assert_eq!(metadata.arguments.len(), 1);
        assert_eq!(metadata.arguments[0].name, "target");
    }
}