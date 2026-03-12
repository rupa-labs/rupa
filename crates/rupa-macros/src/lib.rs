//! # Rupa Macros 🛠️
//!
//! Procedural macros for the Rupa Framework. This crate provides 
//! the **Composites** for automating boilerplate in components, forms, 
//! and reactive state management.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

/// Attribute macro to mark a struct as a Rupa Component.
///
/// Currently acts as a marker, but will be expanded to automate 
/// the `Component` trait implementation and reactive dependency tracking.
#[proc_macro_attribute]
pub fn component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Derive macro for the `Form` trait in `rupa-forms`.
///
/// Automatically generates the `fields()` mapping for a struct, 
/// allowing it to be used with the Rupa Form Validation engine.
///
/// # Panics
///
/// Panics if applied to a non-struct type.
#[proc_macro_derive(Form)]
pub fn derive_form(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let fields_map = match input.data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let recurse = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        let name_str = name.as_ref().unwrap().to_string();
                        quote! {
                            map.insert(#name_str.to_string(), std::sync::Arc::new(self.#name.clone()) as std::sync::Arc<dyn rupa_forms::Validatable>);
                        }
                    });
                    quote! {
                        let mut map = std::collections::HashMap::new();
                        #(#recurse)*
                        map
                    }
                }
                _ => quote! { std::collections::HashMap::new() },
            }
        }
        _ => panic!("Form derive macro only supports structs with named fields"),
    };

    let expanded = quote! {
        impl rupa_forms::Form for #name {
            fn fields(&self) -> std::collections::HashMap<String, std::sync::Arc<dyn rupa_forms::Validatable>> {
                #fields_map
            }
        }
    };

    TokenStream::from(expanded)
}
