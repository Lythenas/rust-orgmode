extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro2::TokenStream;
use syn::{DeriveInput, Data, Fields, Type};

fn impl_shared_behavior(input: &DeriveInput) -> TokenStream {
    check_shared_behavior(&input.data);

    let name = &input.ident;

    quote! {
        impl SharedBehavior for #name {
            fn shared_behavior_data(&self) -> &SharedBehaviorData {
                &self.shared_behavior_data
            }
        }
    }
}

fn check_shared_behavior(data: &Data) {
    let data = match data {
        Data::Struct(data) => data,
        _ => panic!("SharedBehavior can only be derived for a struct."),
    };
    let fields = match &data.fields {
        Fields::Named(fields) => &fields.named,
        _ => panic!("SharedBehavior can only be derived for a struct with named fields."),
    };

    let field = fields.iter().find(|ref field| field.ident.as_ref().unwrap() == "shared_behavior_data");
    let field = match field {
        None => panic!("SharedBehavior can only be derived for structs that have a field called \"shared_behavior_data\"."),
        Some(field) => field,
    };

    let ty = match &field.ty {
        Type::Path(ref ty) => ty,
        _ => panic!(),
    };
}

#[proc_macro_derive(Element)]
pub fn element_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let expanded = impl_element(&input);
    proc_macro::TokenStream::from(expanded)
}

fn impl_element(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;
    let shared_behavior_impl = impl_shared_behavior(&input);

    quote! {
        #shared_behavior_impl

        impl Element for #name {}
    }
}

#[proc_macro_derive(ContainsObjects)]
pub fn contains_objects_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let expanded = impl_contains_objects(&input);
    proc_macro::TokenStream::from(expanded)
}

fn impl_contains_objects(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;

    quote! {
        impl ContainsObjects for #name {
            fn content_data(&self) -> &ContentData {
                &self.content_data
            }
        }
    }
}

#[proc_macro_derive(GreaterElement)]
pub fn greater_element_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let expanded = impl_greater_element(&input);
    proc_macro::TokenStream::from(expanded)
}

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

#[proc_macro_derive(HasAffiliatedKeywords)]
pub fn has_affiliated_keywords_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let expanded = impl_affiliated_keywords(&input);
    proc_macro::TokenStream::from(expanded)
}

fn impl_affiliated_keywords(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;

    quote! {
        impl HasAffiliatedKeywords for #name {
            fn affiliated_keywords_data(&self) -> &AffiliatedKeywordsData {
                &self.affiliated_keywords_data
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
