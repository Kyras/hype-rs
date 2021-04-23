use syn::{DeriveInput, DataEnum, parse_quote, Path};
use proc_macro2::TokenStream;
use quote::quote;
use crate::common::{
    error::Result,
    derive_patterns::{add_trait_bound, get_destructuring_pattern, get_fields_idents},
};
use crate::derive_as_feature_vector::content::{derive_feature_size, derive_as_feature_vector};

pub fn implement(input: DeriveInput, data: DataEnum) -> Result<TokenStream> {
    let ty = &input.ident;
    let variants = data.variants;
    let generics = add_trait_bound(input.generics, parse_quote!(AsFeatureVector));
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut feature_size_impls = Vec::with_capacity(variants.len());
    let mut as_feature_impls = Vec::with_capacity(variants.len());

    for variant in variants {
        let ident = &variant.ident;
        let variant_path: Path = parse_quote!(#ty::#ident);
        let pat = get_destructuring_pattern(variant_path, &variant.fields);
        let idents = get_fields_idents(&variant.fields);
        let feature_size_impl = derive_feature_size(&idents);
        let as_feature_impl = derive_as_feature_vector(&idents);
        feature_size_impls.push(quote!(#pat => { #feature_size_impl }));
        as_feature_impls.push(quote!(#pat => { #as_feature_impl }));
    }

    Ok(quote!(
        impl #impl_generics AsFeatureVector for #ty #ty_generics #where_clause {
            fn feature_size(&self) -> usize {
                match self {
                    #(#feature_size_impls)*
                }
            }

            fn as_feature_vector(&self) -> FeatureVector {
                match self {
                    #(#as_feature_impls)*
                }
            }
        }
    ))
}