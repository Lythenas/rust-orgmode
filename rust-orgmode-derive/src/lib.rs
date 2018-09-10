extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro2::{TokenStream, Span};
use syn::{DeriveInput, Data, Fields, Field, Type, TypePath};
use syn::spanned::Spanned;

/// Searched for the given field on the data.
///
/// Panics if `data` is not `Data::Struct` or there is no field named `field_name` in the struct.
/// Panic messages will include the `trait_name`.
fn get_field<'a>(trait_name: &str, field_name: &str, data: &'a Data) -> &'a Field {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    fields.named.iter()
                        .find(|ref field| field.ident.as_ref().unwrap() == field_name)
                        .expect(&format!("{} needs a field named \"{}\".", trait_name, field_name))
                },
                _ => panic!("{} can only be derived on a struct with named fields.", trait_name)
            }
        },
        _ => panic!("{} can only be derived on a struct.", trait_name)
    }
}

/// Implements [`SharedBehavior`].
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

/// Derive the impl for [`Element`].
///
/// Also implements [`SharedBehavior`].
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

/// Derive the impl for [`ContainsObjects`].
#[proc_macro_derive(ContainsObjects)]
pub fn contains_objects_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let expanded = impl_contains_objects(&input);
    proc_macro::TokenStream::from(expanded)
}

/// Implements the `ContainsObjects` trait.
fn impl_contains_objects(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;
    let field = get_field("ContainsObjects", "content_data", &input.data);

    quote_spanned! { field.span()=>
        impl ContainsObjects for #name {
            fn content_data(&self) -> &ContentData {
                &self.content_data
            }
        }
    }
}

/// Derive the impl for [`GreaterElement`].
///
/// Also implements [`Element`] and [`SharedBehavior`].
#[proc_macro_derive(GreaterElement)]
pub fn greater_element_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let expanded = impl_greater_element(&input);
    proc_macro::TokenStream::from(expanded)
}

/// Implements the `GreaterElement`, `Element` and `SharedBehavior` traits.
fn impl_greater_element(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;
    let element_impl = impl_element(&input);
    let contains_objects_impl = impl_contains_objects(&input);

    quote! {
        #element_impl
        #contains_objects_impl

        impl GreaterElement for #name {}
    }
}

/// Derive the impl for [`HasAffiliatedKeywords`].
#[proc_macro_derive(HasAffiliatedKeywords)]
pub fn has_affiliated_keywords_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let expanded = impl_affiliated_keywords(&input);
    proc_macro::TokenStream::from(expanded)
}

/// Implements the `HasAffiliatedKeywords` trait.
fn impl_affiliated_keywords(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;
    let field = get_field("HasAffiliatedKeywords", "affiliated_keywords_data", &input.data);

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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
