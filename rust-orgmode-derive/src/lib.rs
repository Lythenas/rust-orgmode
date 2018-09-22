#![feature(proc_macro_diagnostic)]
extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro2::TokenStream;
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Field, Fields, Ident, PathArguments, Type};

/// Searched for the given field on the data.
///
/// Panics if `data` is not `Data::Struct` or there is no field named `field_name` in the struct.
/// Panic messages will include the `trait_name`.
fn get_field<'a>(trait_name: &str, field_name: &str, data: &'a Data) -> &'a Field {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => fields
                .named
                .iter()
                .find(|ref field| field.ident.as_ref().unwrap() == field_name)
                .unwrap_or_else(|| {
                    panic!("{} needs a field named \"{}\".", trait_name, field_name)
                }),
            _ => panic!(
                "{} can only be derived on a struct with named fields.",
                trait_name
            ),
        },
        _ => panic!("{} can only be derived on a struct.", trait_name),
    }
}

/// Implements `SharedBehavior`.
///
/// This is not derivable because it's not very useful on it's own.
fn impl_shared_behavior(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;
    let field = get_field("SharedBehavior", "shared_behavior_data", &input.data);

    quote_spanned! { field.span()=>
        impl SharedBehavior for #name {
            fn shared_behavior_data(&self) -> &SharedBehaviorData {
                &self.shared_behavior_data
            }
        }
    }
}

/// Derive the impl for `Element`.
///
/// Also implements `SharedBehavior`.
#[proc_macro_derive(Element)]
pub fn element_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let expanded = impl_element(&input);
    proc_macro::TokenStream::from(expanded)
}

/// Implements the `Element` and `SharedBehavior` traits.
fn impl_element(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;
    let shared_behavior_impl = impl_shared_behavior(&input);

    quote! {
        #shared_behavior_impl

        impl Element for #name {}
    }
}

/// Derive the impl for `GreaterElement`.
#[proc_macro_derive(GreaterElement)]
pub fn greater_element_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let expanded = impl_greater_element(&input);
    proc_macro::TokenStream::from(expanded)
}

/// Implements the `GreaterElement`, `Element` and `SharedBehavior` traits.
fn impl_greater_element(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;
    let field = get_field("HasContent", "content_data", &input.data);
    let ty = get_generics_of_field(&field);

    quote! {
        impl GreaterElement#ty for #name {}
    }
}

fn get_generics_of_field(field: &Field) -> &syn::AngleBracketedGenericArguments {
    match &field.ty {
        Type::Path(ty) => match ty.path.segments.last().unwrap().value().arguments {
            PathArguments::AngleBracketed(ref ty_args) => ty_args,
            _ => panic!(),
        },
        _ => panic!(),
    }
}

/// Derive the impl for `HasContent`.
#[proc_macro_derive(HasContent)]
pub fn has_content_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let expanded = impl_has_content(&input);
    proc_macro::TokenStream::from(expanded)
}

/// Implements the `HasContent` trait.
fn impl_has_content(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;
    let field = get_field("HasContent", "content_data", &input.data);
    let ty = get_generics_of_field(&field);

    quote_spanned! { field.span()=>
        impl HasContent#ty for #name {
            fn content_data(&self) -> &ContentData#ty {
                &self.content_data
            }
        }
    }
}

/// Derive the impl for `HasAffiliatedKeywords`.
#[proc_macro_derive(HasAffiliatedKeywords)]
pub fn has_affiliated_keywords_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let expanded = impl_affiliated_keywords(&input);
    proc_macro::TokenStream::from(expanded)
}

/// Implements the `HasAffiliatedKeywords` trait.
fn impl_affiliated_keywords(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;
    let field = get_field(
        "HasAffiliatedKeywords",
        "affiliated_keywords_data",
        &input.data,
    );

    quote_spanned! { field.span()=>
        impl HasAffiliatedKeywords for #name {
            fn affiliated_keywords_data(&self) -> &AffiliatedKeywordsData {
                &self.affiliated_keywords_data
            }
        }
    }
}

#[proc_macro_derive(Object)]
pub fn object_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let expanded = impl_object(&input);
    proc_macro::TokenStream::from(expanded)
}

fn impl_object(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;
    let shared_behavior_impl = impl_shared_behavior(&input);

    quote! {
        #shared_behavior_impl

        impl Object for #name {}
    }
}

/// Derives the implementation of `AsRawString` for enums.
///
/// The enums require a variant called `RawString(String)`.
#[proc_macro_derive(AsRawString)]
pub fn as_raw_string_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    let expanded = impl_as_raw_string(name);
    proc_macro::TokenStream::from(expanded)
}

fn impl_as_raw_string(name: &Ident) -> TokenStream {
    quote! {
        impl AsRawString for #name {
            fn as_raw_string(&self) -> Option<&str> {
                match self {
                    #name::RawString(s) => Some(&s),
                    _ => None,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
