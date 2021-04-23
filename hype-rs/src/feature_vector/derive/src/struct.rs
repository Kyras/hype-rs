use syn::{DeriveInput, DataStruct, Result, Fields, Ident};
use proc_macro2::TokenStream;
use quote::{quote, format_ident};

pub(crate) fn impl_struct(input: &DeriveInput, data: &DataStruct) -> Result<TokenStream> {
    let ty = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let pat = get_struct_destructuring_pattern(data);
    let fields = get_struct_fields(data);

    Ok(quote! {
        impl #impl_generics AsFeatureVector for #ty #ty_generics #where_clause {
            fn feature_size(&self) -> usize {
                let #pat = self;
                #(AsFeatureVector::feature_size(#fields) +)* 0
            }

            fn as_feature_vector(&self) -> FeatureVector {
                let #pat = self;
                let mut ret = FeatureVector::with_capacity(self.feature_size());

                #(FeatureVector::extend(&mut ret, #fields.as_feature_vector());)*

                ret
            }
        }
    })
}

pub(crate) fn get_struct_destructuring_pattern(data: &DataStruct) -> TokenStream {
    match &data.fields {
        Fields::Named(fields) => {
            let fields = fields.named
                .iter()
                .map(|field| &field.ident)
                .filter_map(|field| field.as_ref());
            quote!(Self { #(#fields),* })
        }
        Fields::Unnamed(fields) => {
            let fields = (0..fields.unnamed.len())
                .map(|i| format_ident!("_{}", i));
            quote!(Self(#(#fields),*))
        }
        Fields::Unit => quote!(Self),
    }
}

pub(crate) fn get_struct_fields(data: &DataStruct) -> Vec<Ident> {
    match &data.fields {
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