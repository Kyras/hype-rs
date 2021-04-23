use syn::{DeriveInput, DataEnum, Result, Type};
use proc_macro2::TokenStream;
use quote::{quote, format_ident};
use std::env::var;

pub(crate) fn impl_enum(input: &DeriveInput, data: &DataEnum) -> Result<TokenStream> {
    let ty = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let discriminant_type = get_enum_discriminant_type(input);

    let impls_feature_size = impls_feature_size_for_variants(data);
    let tmp = quote! {
        match self {
            #(#impls_feature_size)*
        }
    };

    println!("{}", tmp.to_string());

    Ok(quote! {
        impl #impl_generics AsFeatureVector for #ty #ty_generics #where_clause {

        }
    })
}

fn impls_feature_size_for_variants(data: &DataEnum) -> Vec<TokenStream> {
    Default::default()
}

fn get_enum_discriminant_type(input: &DeriveInput) -> Type {
    input.attrs.iter()
        // No filtering is needed, right now, only one attribute type can be captured here
        // .filter(|attr| attr.path.is_ident(format_ident!{}))
        .map(|attr| Type::Verbatim(attr.tokens.clone()))
        .next()
        .unwrap_or(Type::Verbatim(quote! {u32}))
}
