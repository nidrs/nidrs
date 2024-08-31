use std::{collections::HashMap, sync::Mutex};

use macro_impl::impl_args_parse;
use once_cell::sync::Lazy;
use proc_macro::TokenStream;
use quote::quote;

mod macro_impl;

static DEFS_DEC: Lazy<Mutex<HashMap<String, Vec<String>>>> = Lazy::new(|| Mutex::new(HashMap::new()));

#[proc_macro_derive(ArgsParse)]
pub fn args_parse_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_args_parse(&ast).into()
}

#[proc_macro_attribute]
pub fn declare(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut defs = DEFS_DEC.lock().unwrap();

    let input = syn::parse_macro_input!(input as syn::Item);
    let mut first = false;
    let fn_ident;
    if let syn::Item::Fn(ref item_fn) = input {
        fn_ident = item_fn.sig.ident.clone();
        let fn_name = fn_ident.to_string();
        if !defs.contains_key(&fn_name) {
            defs.insert(fn_name.clone(), vec![]);
            first = true;
        }
        defs.get_mut(&fn_name).unwrap().push(args.to_string());
    } else {
        panic!("declare attribute only support function");
    }

    let derive_tokens = if first {
        quote! {
            #input
        }
    } else {
        quote! {
            #input
        }
    };
    let expanded = quote! {
        #derive_tokens
    };
    expanded.into()
}

#[proc_macro_attribute]
pub fn proc_attribute(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::Item);
    let expended = if let syn::Item::Fn(item_fn) = input {
        let fn_ident = &item_fn.sig.ident;
        let fn_block = &item_fn.block;
        let fn_name = item_fn.sig.ident.to_string();
        let mut defs = DEFS_DEC.lock().unwrap();
        let declares = defs.remove(&fn_name).unwrap();
        // println!("args: {:#?}", declares);
        // println!("proc_attribute: {:?}", fn_name);
        // println!("block: {}", fn_block.to_token_stream().to_string());

        let args_member = declares
            .iter()
            .enumerate()
            .map(|(index, args)| {
                let f_name = format!("F{}", index + 1);
                let f_ident = syn::Ident::new(&f_name, proc_macro2::Span::call_site());
                let args = syn::parse_str::<syn::TypeTuple>(&format!("({},)", args))
                    .expect("Invalid declare attribute, please check the declare attribute arguments");

                quote! {
                    #f_ident #args,
                }
            })
            .collect::<Vec<_>>();

        // println!("args_member: {}", args_member);

        quote! {
            #[proc_macro_attribute]
            pub fn #fn_ident(args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
                use syn_args::derive::ArgsParse;
                use syn_args::ArgsParse;
                use syn_args::SynArgs;
                #[derive(Debug, ArgsParse)]
                enum Args {
                    #(#args_member)*
                }

                let args: Args = parse_macro_input!(args as SynArgs).arguments().expect("Invalid argument");

                return f(args, input);

                fn f(args: Args, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
                    #fn_block
                }
            }
        }
    } else {
        quote! {
            #input
        }
    };
    expended.into()
}
