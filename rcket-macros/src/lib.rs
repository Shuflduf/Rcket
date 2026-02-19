extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(Node, attributes(token, regex))]
pub fn derive_node(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let output_type = match &input.data {
        Data::Struct(s) => {
            let types: Vec<_> = match &s.fields {
                Fields::Unnamed(f) => f.unnamed.iter().map(|f| &f.ty).collect(),
                Fields::Named(f) => f.named.iter().map(|f| &f.ty).collect(),
                Fields::Unit => vec![],
            };
            match types.len() {
                0 => quote! { () },
                1 => {
                    let ty = types[0];
                    quote! { #ty }
                }
                _ => quote! { (#(#types),*) },
            }
        }
        Data::Enum(_) => quote! { Self },
        _ => todo!(),
    };

    quote! {
        impl Node for #name {
            type Output = #output_type;
            fn parse(input: &str) -> Option<Self::Output> {
                todo!()
            }
        }
    }
    .into()
}
