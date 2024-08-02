use proc_macro::Span;
use quote::ToTokens;
use std::path::PathBuf;
use syn::{Item, ItemFn};

pub fn get_current_app_path() -> Option<PathBuf> {
    let call_site = Span::call_site();
    let binding = call_site.source_file().path();
    let call_site_str = binding.to_string_lossy();
    let call_site_line = call_site.start().line();
    let path_buf = PathBuf::from(call_site_str.to_string());
    if let Some(parent) = path_buf.parent() {
        if let Some(parent) = parent.parent() {
            let path_buf = parent.to_path_buf().join("main.rs");
            return Some(path_buf);
        }
    }
    None
}

pub fn parse_main_macro_args(item_fn: &ItemFn) -> Option<AppArgs> {
    let mut app_args = AppArgs {};
    let mut is_main = false;
    for attr in &item_fn.attrs {
        if attr.to_token_stream().to_string().contains("main") {
            is_main = true;
        }
    }
    if is_main {
        return Some(app_args);
    }
    None
}

#[derive(Debug)]
pub struct AppArgs {}
