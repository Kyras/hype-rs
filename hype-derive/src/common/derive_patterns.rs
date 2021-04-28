use syn::{Path, Fields, Ident, FieldsNamed, FieldsUnnamed, WhereClause, TypeGenerics, Generics, GenericParam, TypeParamBound};
use proc_macro2::TokenStream;
use itertools::Itertools;
use quote::{quote, format_ident};

/// Creates destructuring pattern for given full-qualified identifier and its fields combination.
/// _UNIONS ARE NOT SUPPORTED_
///
///
/// # Arguments
/// * `path` - Full-qualified identifier (path) for fields. E.G.: MyEnum::EnumVariant
/// * `fields` - Fields definition. Contains information about type of definition.
///
/// # Examples
/// Destructuring pattern depends on type of definition. There are three types of definitions:
/// 1. Struct-like definition
/// 2. Tuple-like definition
/// 3. Unit definition
/// There is no difference, if it defines an `enum` variant or a `struct`, syntax is same,
/// only path differs. For struct, the path is its identifier, whilst for enum it is combination
/// of enum identifier and variant identifier. Destructuring pattern is usually used in let-bindings
/// `let <pattern> = <expr>`, if-let expressions `if let <pattern> = <expr> {...}` or
/// match statements `match <expr> { (<pattern> => {<expr>})+ }`
///
/// For enum:
/// ```
/// enum Example {
///   // Struct-like
///   Struct {
///     field1: u32,
///     field2: String,
///   },
///   // Tuple-like
///   Tuple(u32, String),
///   // Unit
///   Unit
/// }
/// ```
/// Destructuring patterns for enum variants are:
/// * `Example::Struct`
/// ```
/// Example::Struct { field1, field2 }
/// ```
/// * `Example::Tuple`
/// ```
/// Example::Tuple(_0, _1)
/// ```
/// * `Example::Unit`
/// ```
/// Example::Unit
/// ```
pub fn get_destructuring_pattern(path: Path, fields: &Fields) -> TokenStream {
    let idents = get_fields_idents(fields);
    match fields {
        Fields::Named(named) => {
            let idents = get_named_idents(named);
            quote!(#path { #(#idents),* })
        }
        Fields::Unnamed(unnamed) => {
            let idents = get_unnamed_idents(unnamed);
            quote!(#path(#(#idents),*))
        }
        Fields::Unit => quote!(#path)
    }
}

/// Extract all fields identifiers, in order, they are defined.
///
/// # Arguments
/// * `fields` - Fields definition. Contains information about type of definition.
///
/// # Example
/// Fields identifiers, are either their given name, or are somehow assigned to them otherwise.
/// For enum:
/// ```
/// enum Example {
///   // Struct-like
///   Struct {
///     field1: u32,
///     field2: String,
///   },
///   // Tuple-like
///   Tuple(u32, String),
///   // Unit
///   Unit
/// }
/// ```
/// Fields identifiers for variants are:
/// * `Example::Struct` - `field1` and `field2`
/// * `Example::Tuple` - `_0` and `_1`. _Tuples will be always assigned identifier by their order!_
/// * `Example::Unit` - None
pub fn get_fields_idents(fields: &Fields) -> Vec<Ident> {
    match fields {
        Fields::Named(fields) => get_named_idents(fields)
            .collect(),
        Fields::Unnamed(fields) => get_unnamed_idents(fields)
            .collect(),
        Fields::Unit => Default::default(),
    }
}

/// Bind all definition generics to satisfy a trait with given identifier.
///
/// This should take all generics from type definition, e.g.:
/// ```
/// struct MyType<T, U> {
/// // ...
/// }
/// ```
/// and bind all type generics to some trait with identifier, for example, deriving trait, which
/// is propagation of trait call for all fields, require, that those fields also satisfies that
/// trait, requiring where clause, which requires all type generics to implement that trait, for
/// example deriving `Debug` for `MyType` requires that type `T` and `U` also satisfies `Debug`:
/// ```
/// impl<T: Debug, U: Debug> Debug for MyType<T, U>
/// {
/// // ...
/// }
/// ```
///
/// # Arguments
/// * `generics` - Generics from definition to be bounded by the trait
pub fn add_trait_bound(
    mut generics: Generics,
    param_bound: TypeParamBound,
) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(param_bound.clone());
        }
    }
    generics
}

fn get_named_idents<'a>(fields: &'a FieldsNamed) -> impl Iterator<Item=Ident> + 'a {
    fields.named
        .iter()
        .filter_map(|field| field.ident.as_ref().cloned())
}

fn get_unnamed_idents<'a>(fields: &'a FieldsUnnamed) -> impl Iterator<Item=Ident> + 'a {
    (0..fields.unnamed.len())
        .map(|idx| format_ident!("_{}", idx))
}