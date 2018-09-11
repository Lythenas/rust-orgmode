extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro2::{TokenStream, Span};
use syn::{DeriveInput, Data, Fields, Field, Type, TypePath, Visibility, Ident, Path};
use syn::spanned::Spanned;
use syn::token::Colon;
use syn::parse::{Parse, ParseStream};

struct FieldsToAdd {
    fields: Vec<String>,
}

impl Parse for FieldsToAdd {
    fn parse(input: ParseStream) -> Result<Self, syn::parse::Error> {
        let mut fields = Vec::new();
        loop {
            let ident = input.parse::<Ident>()?;
            fields.push(ident.to_string());

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            } else {
                break;
            }
        }
        return Ok(FieldsToAdd { fields })
    }
}

macro_rules! make_path {
    ($name:ident) => {
        {
            let path = quote! { $name }.into();
            parse_macro_input!(path as Path)
        }
    };
}

/// Attribute for adding fields used in the traits:
///
/// - `SharedBehavior`
/// - `HasAffiliatedKeywords`
/// - `ContainsObjects`
#[proc_macro_attribute]
pub fn add_fields_for(args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let FieldsToAdd { fields } = parse_macro_input!(args as FieldsToAdd);
    let mut input = parse_macro_input!(input as DeriveInput);

    {
        let output_fields = match &mut input.data {
            Data::Struct(ref mut data) => {
                match data.fields {
                    Fields::Named(ref mut fields) => &mut fields.named,
                    _ => panic!("Not named fields."),
                }
            },
            _ => panic!("Not a struct."),
        };

        for field in fields {
            let field = match field.as_ref() {
                "SharedBehavior" => make_field(make_path!(SharedBehaviorData), "shared_behavior_data"),
                "HasAffiliatedKeywords" => make_field(make_path!(AffiliatedKeywordsData), "affiliated_keywords_data"),
                "ContainsObjects" => make_field(make_path!(ContentData), "content_data"),
                _ => panic!(format!("{} not recognized.", field)),
            };
            output_fields.push(field);
        }
    }

    let output = quote! { #input };
    proc_macro::TokenStream::from(output)
}

/// Creates the field with the given path and name.
fn make_field(path: Path, name: &str) -> Field {
    Field {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        ident: Some(Ident::new(name, Span::call_site())),
        colon_token: Some(Colon { spans: [Span::call_site()] }),
        ty: Type::Path(TypePath {
            qself: None,
            path,
        }),
    }
}

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

/// Derive the impl for `ContainsObjects`.
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

/// Derive the impl for `GreaterElement`.
///
/// Also implements `Element` and `SharedBehavior`.
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
