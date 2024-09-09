use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::ItemFn;

#[proc_macro_attribute]
pub fn api(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as ItemFn);

    let mut args: Vec<TokenStream2> = vec![];

    input.sig.inputs.iter().for_each(|arg| {
        if let syn::FnArg::Typed(pat) = arg {
            let tokens = pat.ty.to_token_stream();
            args.push(quote! {
                .merge_type::<#tokens>()
            })
        }
    });

    quote! {
        #[meta(nidrs::externs::shared::block({
            nidrs::openapi::RouterParams::default()
            #(#args)*
        }))]
        #input
    }
    .into()
}
