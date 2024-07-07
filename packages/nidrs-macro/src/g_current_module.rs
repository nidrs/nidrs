use std::{path, sync::Mutex};

use proc_macro::Span;
use quote::ToTokens;

use crate::{DefaultUsesOptions, ModuleOptions};

static CURRENT_MODULE: Mutex<Option<CurrentModule>> = Mutex::new(None);

#[derive(Clone, Debug)]
struct CurrentModule {
    pub name: String,
    pub imports: Vec<String>,
    pub controllers: Vec<String>,
    pub services: Vec<String>,
    pub exports: Vec<String>,
    pub interceptors: Vec<String>,

    pub default_uses: Vec<String>,
}

impl From<ModuleOptions> for CurrentModule {
    fn from(args: ModuleOptions) -> Self {
        CurrentModule {
            name: "".to_string(),
            imports: args.imports.iter().map(|x| x.to_string()).collect(),
            controllers: args.controllers.iter().map(|x| x.to_string()).collect(),
            services: args.services.iter().map(|x| x.to_string()).collect(),
            exports: args.exports.iter().map(|x| x.to_string()).collect(),
            interceptors: args.interceptors.iter().map(|x| x.to_string()).collect(),
            default_uses: vec![],
        }
    }
}

impl Into<ModuleOptions> for CurrentModule {
    fn into(self) -> ModuleOptions {
        ModuleOptions {
            imports: self.imports.iter().map(|x| syn::parse_str(x).unwrap()).collect(),
            controllers: self.controllers.iter().map(|x| syn::parse_str(x).unwrap()).collect(),
            services: self.services.iter().map(|x| syn::parse_str(x).unwrap()).collect(),
            exports: self.exports.iter().map(|x| syn::parse_str(x).unwrap()).collect(),
            interceptors: self.interceptors.iter().map(|x| syn::parse_str(x).unwrap()).collect(),
        }
    }
}

pub fn set<T: Into<CurrentModule>>(args: T) {
    let mut current_module = CURRENT_MODULE.lock().unwrap();
    *current_module = Some(CurrentModule::from(args.into()));
}

pub fn get() -> Option<CurrentModule> {
    let current_module = CURRENT_MODULE.lock().expect("[10010] lock error");
    current_module.clone().map(|x| x)
}

pub fn begin_mod() {
    if let Some(_) = get() {
        return;
    }
    let call_site = Span::call_site();
    let binding = call_site.source_file().path();
    let call_site_str = binding.to_string_lossy();
    let call_site_line = call_site.start().line();
    let path_buf = path::PathBuf::from(call_site_str.to_string());
    let path_buf = path_buf.parent().expect("[10000] parent error").to_path_buf().join("mod.rs");
    if path_buf.is_file() {
        let mod_content = std::fs::read_to_string(&path_buf).expect("[10001] read file error");
        // println!("// post {:?}", path_buf);
        // println!("// mod.rs {:?}", mod_content);
        let content_ast = syn::parse_file(&mod_content).expect("[10002] parse file error");
        for item in content_ast.items {
            if let syn::Item::Struct(item_module) = item {
                // println!("// mod {:#?}", item_module);
                let mut default_uses = vec![];
                for attr in item_module.attrs.iter() {
                    let attr_path = attr.meta.path();
                    let attr_path = attr_path.segments.iter().map(|seg| seg.ident.to_string()).collect::<Vec<String>>();
                    if attr_path.contains(&"module".to_string()) {
                        let module_args = attr.meta.to_token_stream();
                        let mut module_args: CurrentModule = syn::parse2::<ModuleOptions>(module_args).unwrap().into();
                        module_args.name = item_module.ident.to_string();
                        module_args.default_uses = default_uses.clone();
                        // println!("begin mod {:#?}", module_args);

                        set(module_args);
                    } else if attr_path.contains(&"default_uses".to_string()) {
                        let default_uses_tokens = attr.meta.to_token_stream();
                        let default_uses_args = syn::parse2::<DefaultUsesOptions>(default_uses_tokens).unwrap();
                        default_uses_args.args.iter().for_each(|arg| {
                            default_uses.push(arg.to_string());
                        });
                    }
                }
            }
        }
    }
}

pub fn end_mod() {
    let mut current_module = CURRENT_MODULE.lock().unwrap();
    // println!("end mod {}", current_module.as_ref().unwrap().name);
    *current_module = None;
}
