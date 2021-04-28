use proc_macro2::{TokenStream, Punct, TokenTree, Delimiter, Span};
use proc_macro_error::{abort};
use syn::{parse_macro_input, Attribute, Result, Expr, Error, spanned::Spanned};
use quote::{quote, ToTokens};
use enumflags2::{bitflags, BitFlags};
use itertools::Itertools;

#[bitflags]
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AttributeStyle {
    Flag = 0b1 << 0,
    Parentheses = 0b1 << 1,
    Equals = 0b1 << 2,
}

impl AttributeStyle {
    pub fn get_value(flags: BitFlags<AttributeStyle>, tokens: &TokenStream) -> Result<TokenStream> {
        for flag in flags.iter() {
            let internal_value = flag.internal_get_value(tokens);
            if internal_value.is_ok() {
                return internal_value;
            }
        }

        abort!(
            tokens, "Wrong attribute";
            note = "{}", Self::get_error_note(flags);
            help = "{}", Self::get_error_help(flags);
        )
    }

    fn internal_get_value(self, tokens: &TokenStream) -> Result<TokenStream> {
        match self {
            AttributeStyle::Flag => Self::extract_flag_value(tokens),
            AttributeStyle::Parentheses => Self::extract_paren_value(tokens),
            AttributeStyle::Equals => Self::extract_equals_value(tokens),
        }
    }

    fn extract_flag_value(tokens: &TokenStream) -> Result<TokenStream> {
        if !tokens.is_empty() {
            Err(Error::new(tokens.span(), "wrong flag attribute value"))
        } else {
            Ok(tokens.clone())
        }
    }

    fn extract_paren_value(tokens: &TokenStream) -> Result<TokenStream> {
        let err = Error::new(tokens.span(), "wrong parenthesis attribute value");
        let mut tokens_iter = tokens.into_token_stream().into_iter();
        let internal = tokens_iter.next().ok_or(err.clone())?;
        match &internal {
            TokenTree::Group(group) if group.delimiter() == Delimiter::Parenthesis => {
                Ok(group.stream())
            }
            _ => Err(err),
        }
    }

    fn extract_equals_value(tokens: &TokenStream) -> Result<TokenStream> {
        let err = Error::new(tokens.span(), "wrong equals attribute value");
        let mut tokens_iter = tokens.into_token_stream().into_iter();
        let internal = tokens_iter.next().unwrap();
        Ok(quote! {})
    }

    fn get_error_note(flags: BitFlags<AttributeStyle>) -> String {
        format!("Expected {}", flags.iter()
            .map(|flag| match flag {
                AttributeStyle::Flag => "no value",
                AttributeStyle::Parentheses => "value in parentheses",
                AttributeStyle::Equals => "value after equals sign",
            }.to_string())
            .map(|str| str.to_string())
            .join(" or "))
    }

    fn get_error_help(flags: BitFlags<AttributeStyle>) -> String {
        format!("Use {}", flags.iter()
            .map(|flag| match flag {
                AttributeStyle::Flag => "flag style attribute (#[attribute])",
                AttributeStyle::Parentheses => "parentheses style attribute (#[attribute(...)])",
                AttributeStyle::Equals => "equals style attribute (#[attribute = \"...\"])",
            }.to_string())
            .join(" or "))
    }
}

pub trait AttributeHandler {
    const ATTRIBUTE_NAME: &'static str;
    const ALLOWED_STYLES: BitFlags<AttributeStyle>;

    fn get_attribute(&self) -> &Attribute;
    fn get_value(&self) -> Option<TokenStream>;
}