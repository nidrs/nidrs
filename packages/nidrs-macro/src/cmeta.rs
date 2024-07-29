use std::{any::Any, collections::HashMap, sync::Mutex};

use nidrs_extern::datasets::{self, get_meta_key, get_meta_key_by_ref, MetaKey};
use once_cell::sync::Lazy;
use proc_macro::{token_stream, TokenStream};
use quote::ToTokens;
use syn::{parse::Parse, punctuated::Punctuated, Expr, ExprCall, PatPath};

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
                    let k = paths[paths.len() - 2].clone();
                    let v = call_path.args.first().expect("[cmeta.MetaData.from] call_path.args.first").clone();
                    key = CMetaKey::String(k);
                    // println!("v: {}", v.to_token_stream().to_string());
                    value = CMetaValue::from(v);
                }
            }
            _ => todo!(),
        };

        MetaData { expr, value: Box::new(value), key }
    }
}

impl Into<Expr> for MetaData {
    fn into(self) -> Expr {
        syn::parse_str(&self.expr).expect(&format!("[cmeta.MetaData.into] parse expr error, expr:`{}`", &self.expr))
    }
}

// #[derive(Debug, Clone)]
// pub enum Datasets {
//     DisableDefaultPrefix(datasets::DisableDefaultPrefix),
//     Global(datasets::Global),
//     ServiceName(datasets::ServiceName),
//     ServiceType(datasets::ServiceType),
//     ModuleName(datasets::ModuleName),
//     ControllerPath(datasets::ControllerPath),
//     RouterPath(datasets::RouterPath),
//     RouterMethod(datasets::RouterMethod),
//     RouterName(datasets::RouterName),
//     RouterFullPath(datasets::RouterFullPath),
//     // RouterBodyScheme(datasets::RouterBodyScheme),
// }

// impl Datasets {
//     fn get_meta_key(&self) -> String {
//         match self {
//             Datasets::DisableDefaultPrefix(v) => get_meta_key_by_ref(v),
//             Datasets::Global(v) => get_meta_key_by_ref(v),
//             Datasets::ServiceName(v) => get_meta_key_by_ref(v),
//             Datasets::ServiceType(v) => get_meta_key_by_ref(v),
//             Datasets::ModuleName(v) => get_meta_key_by_ref(v),
//             Datasets::ControllerPath(v) => get_meta_key_by_ref(v),
//             Datasets::RouterPath(v) => get_meta_key_by_ref(v),
//             Datasets::RouterMethod(v) => get_meta_key_by_ref(v),
//             Datasets::RouterName(v) => get_meta_key_by_ref(v),
//             Datasets::RouterFullPath(v) => get_meta_key_by_ref(v),
//             // Datasets::RouterBodyScheme(v) => get_meta_key_by_ref(v),
//             Datasets::ServiceType(v) => get_meta_key_by_ref(v),
//         }
//     }
// }

// impl TryFrom<ExprCall> for Datasets {
//     type Error = String;

//     fn try_from(value: ExprCall) -> Result<Self, Self::Error> {
//         let call_str = value.func.to_token_stream().to_string().replace(" ", "");
//         let call_value = value.args.first();

//         if let None = call_value {
//             return Err(call_str);
//         }

//         let call_value: Expr = call_value.unwrap().to_owned();

//         if call_str.contains("DisableDefaultPrefix") {
//             let v = CMetaValue::from(call_value);
//             if let CMetaValue::Bool(v) = v {
//                 return Ok(Datasets::DisableDefaultPrefix(datasets::DisableDefaultPrefix(v)));
//             }
//         } else if call_str.starts_with("Global") {
//             let v = CMetaValue::from(call_value);
//             if let CMetaValue::Bool(v) = v {
//                 return Ok(Datasets::Global(datasets::Global(v)));
//             }
//         } else if call_str.starts_with("ServiceName") {
//             let v = CMetaValue::from(call_value);
//             if let CMetaValue::String(v) = v {
//                 return Ok(Datasets::ServiceName(datasets::ServiceName(v)));
//             }
//         } else if call_str.starts_with("ServiceType") {
//             let v = CMetaValue::from(call_value);
//             if let CMetaValue::String(v) = v {
//                 return Ok(Datasets::ServiceType(datasets::ServiceType::from(v.as_str())));
//             }
//         } else if call_str.starts_with("ModuleName") {
//             let v = CMetaValue::from(call_value);
//             if let CMetaValue::String(v) = v {
//                 return Ok(Datasets::ModuleName(datasets::ModuleName(v)));
//             }
//         } else if call_str.starts_with("ControllerPath") {
//             let v = CMetaValue::from(call_value);
//             if let CMetaValue::String(v) = v {
//                 return Ok(Datasets::ControllerPath(datasets::ControllerPath(v)));
//             }
//         } else if call_str.starts_with("RouterPath") {
//             let v = CMetaValue::from(call_value);
//             if let CMetaValue::String(v) = v {
//                 return Ok(Datasets::RouterPath(datasets::RouterPath(v)));
//             }
//         } else if call_str.starts_with("RouterMethod") {
//             let v = CMetaValue::from(call_value);
//             if let CMetaValue::String(v) = v {
//                 return Ok(Datasets::RouterMethod(datasets::RouterMethod(v)));
//             }
//         } else if call_str.starts_with("RouterName") {
//             let v = CMetaValue::from(call_value);
//             if let CMetaValue::String(v) = v {
//                 return Ok(Datasets::RouterName(datasets::RouterName(v)));
//             }
//         } else if call_str.starts_with("RouterFullPath") {
//             let v = CMetaValue::from(call_value);
//             if let CMetaValue::String(v) = v {
//                 return Ok(Datasets::RouterFullPath(datasets::RouterFullPath(v)));
//             }
//         }

//         return Err(call_str);
//     }
// }

#[derive(Debug, Clone)]
pub enum CMetaLevel {
    Global,
    Module,
    Service,
    Handler,
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
        let mut current = CMETA_STACK.lock().unwrap();
        if let Some(mut current) = current.as_mut() {
            current.merge(cmeta);
        } else {
            *current = Some(cmeta);
        }
        println!("CMETA: {:?}", current);
    }

    pub fn level() -> Option<CMetaLevel> {
        let stack = CMETA_STACK.lock().unwrap();
        if let Some(cm) = &*stack {
            if cm.data.contains_key("level") {
                if let CMetaValue::String(level) = &cm.data["level"] {
                    match level.as_str() {
                        "global" => return Some(CMetaLevel::Global),
                        "module" => return Some(CMetaLevel::Module),
                        "service" => return Some(CMetaLevel::Service),
                        "handler" => return Some(CMetaLevel::Handler),
                        _ => return None,
                    }
                }
            }
        }
        return None;
    }

    pub fn push(level: CMetaLevel) {
        let mut cmeta = CMeta::new();
        cmeta.set(
            "level",
            match level {
                CMetaLevel::Global => CMetaValue::String("global".into()),
                CMetaLevel::Module => CMetaValue::String("module".into()),
                CMetaLevel::Service => CMetaValue::String("service".into()),
                CMetaLevel::Handler => CMetaValue::String("handler".into()),
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
            let mut tail = cm.tail();
            if let Some(mut t) = tail {
                *CMETA_STACK.lock().unwrap() = Some(*t);
            }
        }
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
}

impl Parse for CMeta {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let items: Punctuated<Expr, syn::Token![,]> = Punctuated::parse_terminated(input)?;
        let mut cmeta = CMeta::new();
        items.iter().for_each(|item| {
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
                println!("metaArgs {:?}", item);
                panic!("Invalid argument");
            }
        });
        Ok(cmeta)
    }
}
