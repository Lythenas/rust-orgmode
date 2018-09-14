#![feature(proc_macro_diagnostic)]
extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro2::{Span, TokenStream};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::token::Colon;
use syn::{
    Attribute, Data, DataEnum, DataUnion, DeriveInput, Field, Fields, FieldsNamed, Generics, Ident,
    Path, Type, TypePath, Visibility,
};

struct FieldsToAdd {
    fields: Vec<Ident>,
}

impl Parse for FieldsToAdd {
    fn parse(input: ParseStream) -> Result<Self, syn::parse::Error> {
        let mut fields = Vec::new();
        loop {
            let ident = input.parse::<Ident>()?;
            fields.push(ident);

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            } else {
                break;
            }
        }
        return Ok(FieldsToAdd { fields });
    }
}

macro_rules! make_path {
    ($name:ident) => {{
        let path = quote! { $name }.into();
        match syn::parse::<Path>(path) {
            Ok(data) => data,
            Err(err) => panic!(format!("Err: {:?}", err)),
        }
    }};
}

fn error_no_struct_with_named_fields<T>(elem: &T) -> proc_macro::TokenStream
where
    T: Spanned + quote::ToTokens,
{
    elem.span()
        .unstable()
        .error("Only structs with named fields are supported.")
        .emit();
    proc_macro::TokenStream::new()
}

/// Attribute for adding fields to structs with named fields.
///
/// Can add fields used in the traits:
///
/// - `SharedBehavior`
/// - `HasAffiliatedKeywords`
/// - `ContainsObjects`
/// - `Element` (same as `SharedBehavior`)
/// - `Object` (same as `SharedBehavior`)
/// - `GreaterElement` (same as `SharedBehavior` and `ContainsObjects`)
///
/// **Note:** You need to enable the `custom_attribute` feature because we annotate all fields with
/// `#[no_getter]` so the [`getters_derive`] does not generate unneded getters.
///
/// # Usage
///
/// ```
/// #![feature(custom_attribute)]
/// use rust_orgmode_derive::add_fields_for;
///
/// # struct SharedBehaviorData;
/// # struct ContentData;
/// #
/// #[add_fields_for(GreaterElement)]
/// struct SomeStruct {}
/// ```
///
/// produces:
///
/// ```
/// # struct SharedBehaviorData;
/// # struct ContentData;
/// #
/// struct SomeStruct {
///     shared_behavior_data: SharedBehaviorData,
///     content_data: ContentData,
/// }
/// ```
///
/// [`getters_derive`]: fn.getters_derive.html
#[proc_macro_attribute]
pub fn add_fields_for(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let FieldsToAdd { fields } = parse_macro_input!(args as FieldsToAdd);
    let mut input = parse_macro_input!(input as DeriveInput);

    {
        let output_fields = match &mut input.data {
            Data::Struct(ref mut data) => match data.fields {
                Fields::Named(ref mut fields) => &mut fields.named,
                ref other => {
                    return error_no_struct_with_named_fields(other);
                }
            },
            Data::Enum(DataEnum {
                enum_token: other, ..
            }) => {
                return error_no_struct_with_named_fields(&other);
            }
            Data::Union(DataUnion {
                union_token: other, ..
            }) => {
                return error_no_struct_with_named_fields(&other);
            }
        };

        for field in fields {
            let field_name = field.to_string();
            let fields = match field_name.as_ref() {
                "SharedBehavior" | "Element" | "Object" => vec![shared_behavior_field()],
                "HasAffiliatedKeywords" => vec![has_affiliated_keywords_field()],
                "ContainsObjects" => vec![contains_objects_field()],
                "GreaterElement" => vec![shared_behavior_field(), contains_objects_field()],
                _ => {
                    field
                        .span()
                        .unstable()
                        .error(format!("`{}` is not recognized.", field_name))
                        .emit();
                    Vec::new()
                }
            };
            output_fields.extend(fields);
        }
    }

    let output = quote! { #input };
    proc_macro::TokenStream::from(output)
}

fn shared_behavior_field() -> Field {
    make_field(make_path!(SharedBehaviorData), "shared_behavior_data")
}

fn has_affiliated_keywords_field() -> Field {
    make_field(
        make_path!(AffiliatedKeywordsData),
        "affiliated_keywords_data",
    )
}
fn contains_objects_field() -> Field {
    make_field(make_path!(ContentData), "content_data")
}

/// Creates the field with the given path and name.
fn make_field(path: Path, name: &str) -> Field {
    // this does not work so we have to construct the field manually
    //let expand = quote! {
    //    #[no_getter]
    //    #name: #path
    //}.into();
    //Field::parse_named(&expand).unwrap()
    Field {
        attrs: vec![make_attribute("no_getter")],
        vis: Visibility::Inherited,
        ident: Some(Ident::new(name, Span::call_site())),
        colon_token: Some(Colon {
            spans: [Span::call_site()],
        }),
        ty: Type::Path(TypePath { qself: None, path }),
    }
}

fn make_attribute(name: &str) -> Attribute {
    use syn::token::{Bracket, Pound};
    use syn::AttrStyle;

    let name = Ident::new(name, Span::call_site());
    let path = quote! { #name }.into();
    let path = match syn::parse::<Path>(path) {
        Ok(data) => data,
        Err(err) => panic!(format!("Err: {:?}", err)),
    };

    Attribute {
        pound_token: Pound {
            spans: [Span::call_site()],
        },
        style: AttrStyle::Outer,
        bracket_token: Bracket {
            span: Span::call_site(),
        },
        path,
        tts: TokenStream::new(),
    }
}

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
                .expect(&format!(
                    "{} needs a field named \"{}\".",
                    trait_name, field_name
                )),
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

/// Derives getters for all fields except attributes marked with `#[no_getter]`.
///
/// The `add_fields_for` attribute adds this marker to all fields it generates.
///
/// This needs `#![feature(custom_attribute)]`.
///
/// # Examples
///
/// ```
/// #[macro_use]
/// extern crate rust_orgmode_derive;
///
/// #[derive(getters)]
/// pub struct Something<T> where T: Eq {
///     value: T,
/// }
///
/// fn use_something(s: Something<String>) -> bool {
///     s.value() == "something"
/// }
///
/// # fn main() {}
/// ```
#[proc_macro_derive(getters)]
pub fn getters_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let fields = match &input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => fields,
            ref other => {
                return error_no_struct_with_named_fields(other);
            }
        },
        Data::Enum(DataEnum {
            enum_token: other, ..
        }) => {
            return error_no_struct_with_named_fields(&other);
        }
        Data::Union(DataUnion {
            union_token: other, ..
        }) => {
            return error_no_struct_with_named_fields(&other);
        }
    };
    let generics = &input.generics;

    let expanded = impl_getters(name, fields, generics);
    proc_macro::TokenStream::from(expanded)
}

fn impl_getters(name: &Ident, fields: &FieldsNamed, generics: &Generics) -> TokenStream {
    let getters = fields
        .named
        .iter()
        .filter(|field| !contains_attr(field.attrs.iter(), "no_getter"))
        .map(impl_one_getter);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            #(#getters)*
        }
    }
}

fn contains_attr<'a>(mut attrs: impl Iterator<Item = &'a Attribute>, name: &str) -> bool {
    attrs.any(|attr| attr.interpret_meta().unwrap().name().to_string() == name)
}

fn impl_one_getter(field: &Field) -> TokenStream {
    let name = &field.ident;
    let ty = &field.ty;
    let span = field.span();
    let attrs = &field.attrs; // TODO maybe whitelist or blacklist specific attributes

    quote_spanned! { span=>
        #(#attrs)*
        pub fn #name(&self) -> &#ty {
            &self.#name
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
