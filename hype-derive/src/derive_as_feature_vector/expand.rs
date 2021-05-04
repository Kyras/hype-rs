use syn::{DeriveInput, Data};
use proc_macro2::TokenStream;
use quote::quote;
use crate::common::error::DeriveError;

pub fn derive(input: DeriveInput) -> TokenStream {
    let ret = match input.data {
        Data::Struct(ref data) => {
            let data = data.clone();
            crate::derive_as_feature_vector::r#struct::implement(input, data)
        }
        Data::Enum(ref data) => {
            let data = data.clone();
            crate::derive_as_feature_vector::r#enum::implement(input, data)
        }
        Data::Union(_) => Err(DeriveError::new(input, "Unions are not supported"))
    };

    match ret {
        Ok(implementation) => quote!(
            #implementation
        ),
        Err(error) => error.abort(),
    }
}