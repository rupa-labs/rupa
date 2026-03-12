use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_attribute]
pub fn component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

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
        _ => panic!("Form derive macro only supports structs"),
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
