extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{
    parse_macro_input, Result, Token, ItemStruct,
    ItemEnum, Field, Fields, Ident, ExprField, Expr,
    parse::{Parse, ParseStream},
};
use quote::quote;

enum Item {
    Struct(ItemStruct),
    Enum(ItemEnum),
}

impl Parse for Item {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![struct]) {
            input.parse().map(Item::Struct)
        } else if lookahead.peek(Token![enum]) {
            input.parse().map(Item::Enum)
        } else {
            Err(lookahead.error())
        }
    }
}

#[proc_macro_derive(AsFeatureVector)]
pub fn derive_as_feature_vector(input: TokenStream) -> TokenStream {
    let item: Item = parse_macro_input!(input as Item);

    match item {
        Item::Struct(item) => {
            derive_for_struct(item)
        }
        Item::Enum(item) => {
            derive_for_enum(item)
        }
    }
}

fn derive_for_struct(input: ItemStruct) -> TokenStream {
    match input.fields {
        Fields::Named(_) => derive_for_named_struct(input),
        Fields::Unnamed(_) => derive_for_unnamed_struct(input),
        Fields::Unit => derive_for_unit_struct(input),
    }
}

fn derive_for_named_struct(input: ItemStruct) -> TokenStream {
    let name = &input.ident;
    let idents: Vec<Ident> = input.fields.iter().map(|field| field.ident.as_ref().unwrap().clone()).collect();
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    TokenStream::from(quote! {
        impl #impl_generics ::feature_vector::AsFeatureVector for #name #ty_generics #where_clause {
            #[inline]
            fn feature_size(&self) -> usize {
                #(
                    ::feature_vector::AsFeatureVector::feature_size(&self.#idents) +
                )* 0
            }

            fn as_feature_vector(&self) -> ::feature_vector::FeatureVector {
                let mut ret = ::feature_vector::FeatureVector::with_capacity(self.feature_size());
                #(
                    ret.extend(::feature_vector::AsFeatureVector::as_feature_vector(&self.#idents));
                )*
                ret
            }
        }
    })
}

fn derive_for_unnamed_struct(input: ItemStruct) -> TokenStream {
    TokenStream::new()
}

fn derive_for_unit_struct(input: ItemStruct) -> TokenStream {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    TokenStream::from(quote! {
        impl #impl_generics ::feature_vector::AsFeatureVector for #name #ty_generics #where_clause {
            #[inline]
            fn feature_size(&self) -> usize {
                0
            }

            fn as_feature_vector(&self) -> ::feature_vector::FeatureVector {
                ::feature_vector::FeatureVector::empty()
            }
        }
    })
}


fn derive_for_enum(_input: ItemEnum) -> TokenStream {
    // TODO: Implement AsFeature for enums
    TokenStream::new()
}
