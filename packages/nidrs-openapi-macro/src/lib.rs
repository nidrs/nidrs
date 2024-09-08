use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn openapi(args: TokenStream, input: TokenStream) -> TokenStream {
    input
}
