use syn::Ident;
use proc_macro2::TokenStream;
use quote::quote;

pub fn derive_feature_size(idents: &[Ident]) -> TokenStream {
    quote!(
        0 #(+ AsFeatureVector::feature_size(#idents))*
    )
}

pub fn derive_as_feature_vector(idents: &[Ident]) -> TokenStream {
    quote!(
        let mut feature = FeatureVector::with_capacity(self.feature_size());

        #(feature.extend(AsFeatureVector::as_feature_vector(#idents));)*

        feature
    )
}