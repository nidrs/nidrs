#![feature(proc_macro_span)]
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn user(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);
    return TokenStream::from(quote! {
        #[nidrs::meta(datasets::role::Role::User)]
        #input
    });
}
