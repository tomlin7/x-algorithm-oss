use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn receive_stats(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
