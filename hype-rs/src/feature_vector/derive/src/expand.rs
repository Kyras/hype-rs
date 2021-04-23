use syn::{DeriveInput, Result, Data, Error};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub(crate) fn derive(input: &DeriveInput) -> Result<TokenStream> {
    let impls = match &input.data {
        Data::Struct(data) => crate::r#struct::impl_struct(input, data),
        Data::Enum(data) => crate::r#enum::impl_enum(input, data),
        Data::Union(_data) => Err(Error::new_spanned(input, "Unions are not supported"))
    }?;

    let attrs = &input.attrs;
    for attr in attrs {
        println!("{:?}", attr.path.segments.first().unwrap().ident)
    }

    let dummy_const = format_ident!("__DERIVE_AsFeatureVector_FOR_{}", input.ident);
    Ok(quote! {
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const #dummy_const: () = {
            use hype_rs::prelude::*;
            #impls
        };
    })
}