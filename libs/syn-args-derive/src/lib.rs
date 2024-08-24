use macro_impl::impl_args_parse;
use proc_macro::TokenStream;

mod macro_impl;

#[proc_macro_derive(ArgsParse)]
pub fn args_parse_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_args_parse(&ast).into()
}
