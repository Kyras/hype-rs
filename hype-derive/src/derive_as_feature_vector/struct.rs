use syn::{DeriveInput, DataStruct, parse_quote};
use proc_macro2::TokenStream;
use quote::{quote, format_ident};
use crate::common::derive_patterns::{add_trait_bound, get_destructuring_pattern, get_fields_idents};
use crate::derive_as_feature_vector::derive_content::{derive_feature_size, derive_as_feature_vector};

pub fn implement(input: DeriveInput, data: DataStruct) -> TokenStream {
    let ty = &input.ident;
    let fields = data.fields;
    let generics = add_trait_bound(input.generics, parse_quote!(AsFeatureVector));
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let pat = get_destructuring_pattern(parse_quote!(#ty), &fields);
    let idents = get_fields_idents(&fields);
    let feature_size_impl = derive_feature_size(&idents);
    let as_feature_impl = derive_as_feature_vector(&idents);

    quote!(
        impl #impl_generics AsFeatureVector for #ty #ty_generics #where_clause {
            fn feature_size(&self) -> usize {
                let #pat = self;
                #feature_size_impl
            }

            fn as_feature_vector(&self) -> FeatureVector {
                let #pat = self;
                #as_feature_impl
            }
        }
    )
}