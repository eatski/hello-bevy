//! トークン定義マクロ
//!
//! 新しいトークンを簡単に追加するためのマクロシステム



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
    #[test]
    fn test_token_macro() {
        // マクロシステムが正しく動作することを確認
        // TokenMetadataRegistryが削除されたため、現在はdefine_tokenマクロも使用されていない
    }
}