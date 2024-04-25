use std::{collections::HashMap, hash::Hash, sync::{Arc, Mutex}};

use once_cell::sync::Lazy;
use proc_macro::Span;
use proc_macro2::TokenStream;

pub static mut PATHS: Lazy<HashMap<String, String>> = Lazy::new(|| HashMap::new());

pub fn push_path(item_name: &str) {
  let call_site = Span::call_site();
  let binding = call_site.source_file().path();
  let call_site_str = binding.to_string_lossy();
  // let call_site_line = call_site.start().line();

  unsafe { PATHS.insert(item_name.to_string(), path_to_mod_path(&call_site_str)) };

  // println!("call_site: {:?}", paths);
}

pub fn concat_path(item_name: &str) -> String {
  let path = unsafe { PATHS.get(item_name).unwrap() };
  path.to_string() + "::" + item_name
}

pub fn gen_use_path_tokens(item_name: &str) -> TokenStream {
  let concat = concat_path(item_name);
  let path_tokens = syn::parse_str::<syn::Path>(&concat).unwrap();
  let tokens = quote::quote! {
    pub use #path_tokens;
  };
  tokens
}

pub fn gen_import_mod_tokens()-> TokenStream {
  let paths: &Lazy<HashMap<String, String>> = unsafe { &PATHS };
  // println!("paths: {:?}", paths);
  let tokens: Vec<TokenStream> = paths.iter()
    .map(|(item_name, path)| {
      let full_path = gen_use_path_tokens(item_name);
      let tokens = quote::quote! {
        #full_path
      };
      tokens
    })
    .collect::<Vec<TokenStream>>();
  quote::quote! {
    pub mod import {
      #(#tokens)*
    }
  }
}

fn path_to_mod_path(path: &str) -> String {
  let binding = path.split("src/").collect::<Vec<&str>>();
  let src_other = binding.last().unwrap();
  "crate::".to_string() + &src_other.trim()  // 移除开头的 src/
      .replace("/", "::")          // 替换所有的 / 为 ::
      .replace("\\", "::")         // 替换所有的 \ 为 ::，适用于 Windows 路径
      .trim_end_matches(".rs")     // 移除文件扩展名
      .to_string()
}