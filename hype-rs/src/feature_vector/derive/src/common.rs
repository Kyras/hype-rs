use syn::{Ident, Fields};
use quote::{quote, format_ident};
use proc_macro2::TokenStream;

pub(crate) fn get_fields_idents(fields: &Fields) -> Vec<Ident> {
    match fields {
        Fields::Named(fields) => fields.named
            .iter()
            .map(|field| &field.ident)
            .filter_map(|field| field.as_ref())
            .map(|field| field.clone())
            .collect(),
        Fields::Unnamed(fields) => (0..fields.unnamed.len())
            .map(|i| format_ident!("_{}", i))
            .collect(),
        Fields::Unit => Default::default(),
    }
}

pub(crate) fn get_destructuring_pattern(identifier: Ident, fields: &Fields) -> TokenStream {
    let idents = get_fields_idents(fields);
    match fields {
        Fields::Named(_) => quote! {
            #identifier { #(#idents),* }
        },
        Fields::Unnamed(_) => quote! {
             #identifier( #(#idents),* )
        },
        Fields::Unit => quote!(#identifier),
    }
}

pub(crate) fn get_feature_size(field_idents: &[Ident]) -> TokenStream {
    quote! {
        #(AsFeatureVector::feature_size(#field_idents)+)* 0
    }
}

pub(crate) fn get_feature_vector(field_idents: &[Ident], additional_size: usize) -> TokenStream {
    quote! {
        {
            let mut ret = FeatureVector::with_capacity(FeatureVector::feature_size(self) + #additional_size);
            #(FeatureVector::extend(&mut ret, #field_idents.as_feature_vector());)*
            ret
        }
    }
}