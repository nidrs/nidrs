use std::{any::Any, collections::HashMap, sync::Mutex};

use nidrs_extern::datasets::{self, get_meta_key, get_meta_key_by_ref, MetaKey};
use once_cell::sync::Lazy;
use proc_macro::{token_stream, TokenStream};
use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::{ext, parse::Parse, punctuated::Punctuated, spanned::Spanned, Expr, ExprCall, PatPath};

use crate::{
    app_parse::{get_current_app_path, parse_main_macro_args},
    current_module,
};

static CMETA_STACK: Lazy<Mutex<Option<CMeta>>> = Lazy::new(|| Mutex::new(None));
// static CMETA_CURRENT: Lazy<Mutex<Option<CMeta>>> = Lazy::new(|| Mutex::new(None));

#[derive(Debug, Clone)]
pub struct MetaData {
    expr: String,
    value: Box<CMetaValue>,
    key: CMetaKey,
}

impl MetaData {
    pub fn key(&self) -> String {
        if let CMetaKey::String(k) = self.key.clone() {
            return k;
        } else {
            eprintln!("[cmeta.MetaData.key] unknown");
            return "".to_string();
        }
    }
    pub fn value(&self) -> CMetaValue {
        *self.value.clone()
    }
}

impl From<Expr> for MetaData {
    fn from(expr_value: Expr) -> Self {
        let expr = expr_value.to_token_stream().to_string();
        let mut key = CMetaKey::None;
        let mut value = CMetaValue::None;

        match expr_value {
            Expr::Path(path) => {
                let paths = path.path.segments.iter().map(|s| s.ident.to_string()).collect::<Vec<String>>();
                let k = paths.join("::").replace(" ", "");
                key = CMetaKey::String(k);
            }
            Expr::Call(call_path) => {
                if let Expr::Path(path) = *call_path.func.clone() {
                    let paths = path.path.segments.iter().map(|s| s.ident.to_string()).collect::<Vec<String>>();
                    let k1 = paths[paths.len() - 1].clone();
                    if k1 == "from" {
                        let k = paths[paths.len() - 2].clone();
                        key = CMetaKey::String(k);
                    } else {
                        key = CMetaKey::String(k1);
                    }
                    if let Some(args) = call_path.args.first() {
                        value = CMetaValue::from(args.clone());
                    } else {
                        value = CMetaValue::None;
                    }
                    // println!("v: {}", v.to_token_stream().to_string());
                }
            }
            _ => (),
        };

        MetaData { expr, value: Box::new(value), key }
    }
}

impl Into<Expr> for MetaData {
    fn into(self) -> Expr {
        syn::parse_str(&self.expr).expect(&format!("[cmeta.MetaData.into] parse expr error, expr:`{}`", &self.expr))
    }
}

#[derive(Debug, Clone)]
pub enum CMetaLevel {
    Global(String),
    Module(String),
    Service(String),
    Handler(String),
}

#[derive(Debug, Clone)]
pub enum CMetaKey {
    String(String),
    None,
}

#[derive(Debug, Clone)]
pub enum CMetaValue {
    String(String),
    Bool(bool),
    Int(i64),
    Float(f64),
    Array(Vec<CMetaValue>),
    Object(HashMap<String, CMetaValue>),
    // Datasets(Datasets),
    MetaData(MetaData),
    None,
}

impl From<Expr> for CMetaValue {
    fn from(exp: Expr) -> Self {
        match &exp {
            Expr::Lit(lit) => {
                let lit = lit.lit.clone();
                if let syn::Lit::Str(s) = lit {
                    CMetaValue::String(s.value())
                } else if let syn::Lit::Bool(b) = lit {
                    CMetaValue::Bool(b.value)
                } else if let syn::Lit::Int(i) = lit {
                    CMetaValue::Int(i.base10_parse().unwrap())
                } else if let syn::Lit::Float(f) = lit {
                    CMetaValue::Float(f.base10_parse().unwrap())
                } else {
                    CMetaValue::String(lit.to_token_stream().to_string())
                }
            }
            Expr::Closure(closure) => CMetaValue::None,
            Expr::Array(array) => {
                let mut arr = Vec::new();
                for item in array.elems.iter() {
                    arr.push(CMetaValue::from(item.clone()));
                }
                CMetaValue::Array(arr)
            }
            _ => CMetaValue::MetaData(exp.into()),
        }
    }
}

impl Into<Expr> for CMetaValue {
    fn into(self) -> Expr {
        match self {
            CMetaValue::String(s) => syn::parse_str(&format!("\"{}\"", s)).expect("[cmeta.CMetaValue.into] parse string error"),
            CMetaValue::Bool(b) => syn::parse_str(&format!("{}", b)).expect("[cmeta.CMetaValue.into] parse bool error"),
            CMetaValue::Int(i) => syn::parse_str(&format!("{}", i)).expect("[cmeta.CMetaValue.into] parse int error"),
            CMetaValue::Float(f) => syn::parse_str(&format!("{}", f)).expect("[cmeta.CMetaValue.into] parse float error"),
            CMetaValue::Array(arr) => {
                let mut items: Vec<Expr> = Vec::new();
                for item in arr.iter() {
                    items.push(item.clone().into());
                }
                syn::parse_str(&format!("[{}]", items.iter().map(|i| i.to_token_stream().to_string()).collect::<Vec<String>>().join(", ")))
                    .expect("[cmeta.CMetaValue.into] parse array error")
            }
            CMetaValue::Object(obj) => {
                let mut items = Vec::new();
                for (k, v) in obj.iter() {
                    let k = syn::Ident::new(&k, proc_macro2::Span::call_site());
                    let v: Expr = v.clone().into();
                    items.push(quote::quote! {
                        #k: #v
                    });
                }
                syn::parse_str(&format!("{{ {} }}", items.iter().map(|i| i.to_token_stream().to_string()).collect::<Vec<String>>().join(", ")))
                    .expect("[cmeta.CMetaValue.into] parse object error")
            }
            CMetaValue::MetaData(meta) => meta.into(),
            CMetaValue::None => syn::parse_str("").expect("[cmeta.CMetaValue.into] parse none error"),
        }
    }
}

impl From<CMetaValue> for String {
    fn from(v: CMetaValue) -> String {
        if let CMetaValue::String(s) = v {
            return s;
        }
        panic!("Invalid type");
    }
}

#[derive(Debug)]
pub struct CMeta {
    data: HashMap<String, CMetaValue>,
    extends: Option<Box<CMeta>>,
}

impl CMeta {
    pub fn new() -> CMeta {
        return CMeta { data: HashMap::new(), extends: None };
    }

    pub fn collect(mut cmeta: CMeta) {
        println!("//  CMETA: {:?}", cmeta.keys());
        let mut current = CMETA_STACK.lock().unwrap();
        if let Some(mut current) = current.as_mut() {
            current.merge(cmeta);
        } else {
            *current = Some(cmeta);
        }
    }

    pub fn get_level() -> Option<CMetaLevel> {
        let stack = CMETA_STACK.lock().unwrap();
        if let Some(cmeta) = stack.as_ref() {
            return cmeta.level();
        }
        return None;
    }

    pub fn get_deep() -> usize {
        let stack = CMETA_STACK.lock().unwrap();
        if let Some(cmeta) = stack.as_ref() {
            return cmeta.deep();
        }
        return 0;
    }

    pub fn get_stack<K: Into<String>>(key: K) -> Option<CMetaValue> {
        let stack = CMETA_STACK.lock().unwrap();
        if let Some(cmeta) = stack.as_ref() {
            return cmeta.get(key).cloned();
        }
        return None;
    }

    pub fn get_stack_data<K: Into<String>, R>(key: K) -> Option<R>
    where
        R: From<CMetaValue>,
    {
        if let Some(CMetaValue::MetaData(v)) = CMeta::get_stack(key) {
            Some(v.value().into())
        } else {
            // panic!("[route_derive] {} RouterMethod not found", route_fn_name);
            None
        }
    }

    pub fn push(level: CMetaLevel) {
        println!("// >>Push: {:?} -- [{:?}]", level, CMeta::get_stack("module"));

        let mut cmeta = CMeta::new();
        cmeta.set(
            match level {
                CMetaLevel::Global(_) => "global",
                CMetaLevel::Module(_) => "module",
                CMetaLevel::Service(_) => "service",
                CMetaLevel::Handler(_) => "handler",
            },
            match level {
                CMetaLevel::Global(name) => CMetaValue::String(name),
                CMetaLevel::Module(name) => CMetaValue::String(name),
                CMetaLevel::Service(name) => CMetaValue::String(name),
                CMetaLevel::Handler(name) => CMetaValue::String(name),
            },
        );
        let mut stack = CMETA_STACK.lock().unwrap();
        let mut opt_cm: Option<CMeta> = stack.take();
        if let Some(mut cm) = opt_cm {
            cmeta.extends(cm);
        }
        *stack = Some(cmeta);
    }

    pub fn pop() {
        let mut opt_cm: Option<CMeta> = CMETA_STACK.lock().unwrap().take();
        if let Some(mut cm) = opt_cm {
            println!("// << Pop: {:?} {:?}\n", cm.level(), cm.keys());
            let mut tail = cm.tail();
            if let Some(mut t) = tail {
                *CMETA_STACK.lock().unwrap() = Some(*t);
            }
        }
    }

    pub fn build_tokens() -> TokenStream2 {
        let stack = CMETA_STACK.lock().unwrap();
        if let Some(cmeta) = stack.as_ref() {
            let tokens = cmeta.to_tokens();

            return quote::quote! {
                let mut meta = nidrs::InnerMeta::new();
                #tokens
                meta
            };
        }
        return quote::quote! {};
    }

    pub fn merge(&mut self, cmeta: CMeta) {
        for (k, v) in cmeta.data.iter() {
            self.data.insert(k.clone(), v.clone());
        }
    }

    pub fn extends(&mut self, cmeta: CMeta) {
        self.extends = Some(Box::new(cmeta));
    }

    pub fn tail(&mut self) -> Option<Box<CMeta>> {
        return self.extends.take();
    }

    pub fn set<K: Into<String>, V: Into<CMetaValue>>(&mut self, key: K, value: V) {
        self.data.insert(key.into(), value.into());
    }

    pub fn set_data<V: Into<CMetaValue>>(&mut self, value: V) {
        let cmv = value.into();
        // if let CMetaValue::Datasets(datasets) = &cmv {
        //     let k = datasets.get_meta_key();
        //     self.data.insert(k, cmv);
        // } else
        if let CMetaValue::MetaData(v) = &cmv {
            self.data.insert(v.key(), cmv);
        } else {
            panic!("[cmeta.CMeta.set_data] unknown");
        }
    }

    pub fn get<K: Into<String>>(&self, key: K) -> Option<&CMetaValue> {
        let key = key.into();
        let c = self.data.get(&key);

        if let Some(c) = c {
            return Some(c);
        }

        if let Some(extends) = &self.extends {
            return extends.get(&key);
        }

        return None;
    }

    pub fn keys(&self) -> Vec<String> {
        let mut ret_keys = self.data.keys().map(|k| k.clone()).collect::<Vec<String>>();

        if let Some(extends) = &self.extends {
            let res = extends.keys();
            for k in res.iter() {
                if !ret_keys.contains(k) {
                    ret_keys.push(k.clone());
                }
            }
        }

        ret_keys
    }

    pub fn level(&self) -> Option<CMetaLevel> {
        if let Some(CMetaValue::String(name)) = self.get("handler") {
            return Some(CMetaLevel::Handler(name.clone()));
        } else if let Some(CMetaValue::String(name)) = self.get("service") {
            return Some(CMetaLevel::Service(name.clone()));
        } else if let Some(CMetaValue::String(name)) = self.get("module") {
            return Some(CMetaLevel::Module(name.clone()));
        } else if let Some(CMetaValue::String(name)) = self.get("global") {
            return Some(CMetaLevel::Global(name.clone()));
        }
        return None;
    }

    pub fn deep(&self) -> usize {
        let mut deep = 1;
        if let Some(extends) = &self.extends {
            deep += extends.deep();
        }
        return deep;
    }

    pub fn to_tokens(&self) -> TokenStream2 {
        let mut items = Vec::new();
        let keys = self.keys();
        // println!("to_tokens: {:?}", keys);
        for k in keys.iter() {
            let v = self.get(k).expect("[cmeta.CMeta.to_tokens] get value error");
            let tokens: Expr = v.clone().into();
            if let CMetaValue::MetaData(_) = &v {
                items.push(quote::quote! {
                    meta.set_data(#tokens);
                });
            } else {
                items.push(quote::quote! {
                    meta.set(#k, #tokens);
                });
            }
        }
        let cmeta = quote::quote! {
            #(#items)*
        };
        cmeta
    }
}

impl Parse for CMeta {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let items: Punctuated<Expr, syn::Token![,]> = Punctuated::parse_terminated(input)?;
        let mut cmeta = CMeta::new();
        for item in items.iter() {
            if let syn::Expr::Assign(assign) = item {
                if let syn::Expr::Path(path) = *assign.left.clone() {
                    let k = path.path.segments.first().expect("[cmeta.CMeta.parse] path.segments.first").ident.to_string();
                    let v = *assign.right.clone();
                    cmeta.set(k, v);
                }
            } else if let syn::Expr::Path(_) = item {
                cmeta.set_data(item.clone());
            } else if let syn::Expr::Call(_) = item {
                cmeta.set_data(item.clone());
            } else {
                return Err(syn::Error::new(item.span(), "unknown"));
            }
        }
        Ok(cmeta)
    }
}

pub fn init_app_meta() {
    CMeta::push(CMetaLevel::Global("app".to_string()));
    let cmeta = CMeta::new();
    if let Some(app_path) = get_current_app_path() {
        // println!("// init_app_meta: {:?} {:?}", app_path, app_path.exists());
        if app_path.exists() {
            let app = std::fs::read_to_string(app_path.clone()).expect(&format!("[init_app_meta.read_to_string] read {app_path:?} file error"));
            let app_ast = syn::parse_file(&app).expect(&format!("[init_app_meta.parse_file] parse {app:?} file error"));
            for item in app_ast.items.iter() {
                if let syn::Item::Fn(item_fn) = item {
                    if let Some(args) = parse_main_macro_args(item_fn) {
                        // println!("item: {:?}", item);
                        // println!("args: {:?}", args);
                    }
                }
            }
        }
    }
    CMeta::collect(cmeta)
}

pub fn init_module_meta() {
    let mod_opts = current_module::get();
    if let Some(mod_opts) = mod_opts {
        CMeta::push(CMetaLevel::Module(mod_opts.name));
    }
}
