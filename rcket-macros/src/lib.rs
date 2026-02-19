use proc_macro::TokenStream;

#[proc_macro_derive(Node, attributes(token))]
pub fn derive_node(input: TokenStream) -> TokenStream {
    todo!()
}
