use syn::{DeriveInput, Result, Data, DataStruct};
use proc_macro_error::abort;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use rand::prelude::*;
use crate::common::derive_patterns::{get_fields_idents, get_destructuring_pattern};

pub fn derive(input: DeriveInput) -> TokenStream {
    let ty = input.ident.clone();
    let ret = match input.data {
        Data::Struct(ref data) => {
            let data = data.clone();
            crate::derive_as_feature_vector::r#struct::implement(input, data)
        },
        Data::Enum(ref data) => {
            let data = data.clone();
            crate::derive_as_feature_vector::r#enum::implement(input, data)
        },
        Data::Union(_) => abort!(
            input, "Deriving AsFeatureVector for unions is not supported"
        ),
    };

    let mut rng = thread_rng();
    let scope = format_ident!("__impl_AsFeatureVector_for_{}_{}", ty.to_string(), rng.gen::<u64>());

    quote!(
        #[allow(non_upper_case_globals)]
        const #scope: () = {
            use hype_rs::prelude::*;

            #ret
        };
    )
}