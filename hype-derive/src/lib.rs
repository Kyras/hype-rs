#[allow(unused_extern_crates)]
extern crate proc_macro;

mod common;
mod derive_as_feature_vector;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use syn::{parse_macro_input, DeriveInput, Path, Result, Error};
use quote::{format_ident, quote, quote_spanned};
use syn::spanned::Spanned;

#[proc_macro_error]
#[proc_macro_derive(AsFeatureVector)]
pub fn derive_as_feature_vector_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    derive_as_feature_vector::derive(input)
        .into()
}
