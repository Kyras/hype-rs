use syn::{DeriveInput, DataStruct, Result};
use proc_macro2::TokenStream;
use quote::{quote, format_ident};
use crate::common::{get_fields_idents, get_destructuring_pattern, get_feature_size, get_feature_vector};

pub(crate) fn impl_struct(input: &DeriveInput, data: &DataStruct) -> Result<TokenStream> {
    let ty = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let fields = &data.fields;

    let pat = get_destructuring_pattern(format_ident!("Self"), fields);
    let fields = get_fields_idents(fields);
    let feature_size = get_feature_size(&fields);
    let as_feature_vector = get_feature_vector(&fields, 0);

    Ok(quote! {
        impl #impl_generics AsFeatureVector for #ty #ty_generics #where_clause {
            fn feature_size(&self) -> usize {
                let #pat = self;
                #feature_size
            }

            fn as_feature_vector(&self) -> FeatureVector {
                let #pat = self;
                #as_feature_vector
            }
        }
    })
}