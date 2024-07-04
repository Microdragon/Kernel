// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use syn::parse::{Parse, ParseStream};
use syn::{Expr, LitStr, Token};

/// Input to the [`crate::value`] proc-macro.
pub struct ValueMacroInput {
    pub option: LitStr,
    pub default: Option<Expr>,
}

impl Parse for ValueMacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let option = input.parse()?;
        let comma: Option<Token![,]> = input.parse()?;

        Ok(ValueMacroInput {
            option,
            default: if comma.is_some() {
                input.parse().ok()
            } else {
                None
            },
        })
    }
}

#[cfg(test)]
mod test {
    use super::ValueMacroInput;

    macro_rules! assert_query {
        ($input:expr, $option:expr, $comma:pat, $default:pat) => {
            let stream = syn::parse_str::<ValueMacroInput>($input).unwrap();
            assert_eq!(stream.option.value(), $option);
            assert!(matches!(stream.comma, $comma));
            assert!(matches!(stream.default, $default));
        };
        ($input:expr, $option:expr, $comma:pat) => {
            let stream = syn::parse_str::<ValueMacroInput>($input).unwrap();
            assert_eq!(stream.option.value(), $option);
            assert!(matches!(stream.comma, $comma));
            assert!(matches!(stream.default, None));
        };
        ($input:expr, $option:expr) => {
            let stream = syn::parse_str::<ValueMacroInput>($input).unwrap();
            assert_eq!(stream.option.value(), $option);
            assert!(matches!(stream.comma, None));
            assert!(matches!(stream.default, None));
        };
    }

    #[test]
    fn test_parse() {
        assert_query!("\"foo\"", "foo");

        assert_query!("\"foo.bar\"", "foo.bar");

        assert_query!("\"foo.'bar'\"", "foo.'bar'");

        assert_query!("\"foo.\\\"bar\\\"\"", "foo.\"bar\"");

        assert_query!("\"foo.bar.baz\",", "foo.bar.baz", Some(_));

        assert_query!("\"foo.'bar'\", true", "foo.'bar'", Some(_), Some(_));
    }
}
