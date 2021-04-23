#[allow(unused_extern_crates)]
extern crate proc_macro;

mod expand;
mod r#struct;
mod r#enum;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(AsFeatureVector, attributes(discriminant_type))]
pub fn derive_as_feature_vector(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    expand::derive(&input)
        .unwrap_or_else(|err| err.into_compile_error())
        .into()
}
