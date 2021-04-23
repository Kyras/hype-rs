use syn::{DeriveInput, DataEnum, Result};
use proc_macro2::TokenStream;
use quote::quote;
use std::env::var;

pub(crate) fn impl_enum(_input: &DeriveInput, data: &DataEnum) -> Result<TokenStream> {
    for variant in &data.variants {
        let name = variant.ident.to_string();

        println!("{:?}", name)
    }
    Ok(quote! {})
}