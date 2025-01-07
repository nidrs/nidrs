use std::collections::HashMap;

use nidrs_extern::{datasets::MetaKey, meta::Meta};
use utoipa::{
    openapi::{
        path::{Parameter, ParameterBuilder, ParameterIn},
        Required,
    },
    ToSchema,
};

#[derive(Debug)]
pub struct RouterIn(pub RouterParams);
impl MetaKey for RouterIn {
    fn meta_key() -> String {
        "RouterIn".to_string()
    }
}
impl RouterIn {
    pub fn value(&self) -> &RouterParams {
        &self.0
    }
}

#[derive(Debug)]
pub struct RouterOut(pub RouterParams);
impl MetaKey for RouterOut {
    fn meta_key() -> String {
        "RouterOut".to_string()
    }
}
impl RouterOut {
    pub fn value(&self) -> &RouterParams {
        &self.0
    }
}

#[derive(Debug)]
pub struct RouterSecurity(pub Vec<String>);
impl MetaKey for RouterSecurity {
    fn meta_key() -> String {
        "RouterSecurity".to_string()
    }
}
impl RouterSecurity {
    pub fn value(&self) -> &Vec<String> {
        &self.0
    }
}

#[derive(Clone)]
pub enum ParamDto {
    None,
    ParamList(Vec<Parameter>),
    BodySchema((utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>, Vec<(String, utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>)>)),
}

impl std::fmt::Debug for ParamDto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::ParamList(arg0) => {
                f.debug_tuple("ParamList").field(&arg0.iter().map(|p| serde_json::to_string(p).unwrap()).collect::<Vec<_>>()).finish()
            }
            Self::BodySchema(arg0) => f.debug_tuple("BodySchema").field(&serde_json::to_string(arg0).unwrap()).finish(),
        }
    }
}

#[derive(Clone)]
pub enum ParamDtoIn {
    Param(utoipa::openapi::path::ParameterIn),
    Body,
}

#[derive(Clone)]
pub enum ParamType {
    Param(Parameter), // 路径参数, /path/to/resource/{id} | /path/to/resource?id=123 | header
    Body(BodySchema), // 请求体参数
}

impl std::fmt::Debug for ParamType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Param(arg0) => f.debug_tuple("Param").finish(),
            Self::Body(arg0) => f.debug_tuple("Body").finish(),
        }
    }
}

#[derive(Clone)]
pub struct BodySchema {
    pub content_type: &'static str,
    pub schema:
        Option<(utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>, Vec<(String, utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>)>)>,
}

#[derive(Clone, Default, Debug)]
pub struct RouterParams(pub Vec<ParamType>);

impl RouterParams {
    pub fn value(&self) -> &Vec<ParamType> {
        &self.0
    }

    pub fn comb<T: ToRouterParamsByType>(mut self, indent: &str) -> Self {
        let other = T::to_router_parameters(indent);
        self.0.extend(other.0);
        self
    }
}

pub trait ToRouterParamsByType {
    fn to_router_parameters(indent: &str) -> RouterParams {
        RouterParams(vec![])
    }
}

pub trait ToParamDto {
    fn to_param_dto(_: ParamDtoIn) -> ParamDto {
        ParamDto::None
    }
}
fn build_path_param(indent: &str) -> Parameter {
    ParameterBuilder::new()
        .name(indent)
        .parameter_in(ParameterIn::Path)
        .required(Required::True)
        // .description(Some("路径参数".to_string()))
        .build()
}

impl ToRouterParamsByType for axum::extract::Path<String> {
    fn to_router_parameters(indent: &str) -> RouterParams {
        RouterParams(vec![ParamType::Param(build_path_param(indent))])
    }
}

impl ToRouterParamsByType for axum::extract::Path<&str> {
    fn to_router_parameters(indent: &str) -> RouterParams {
        RouterParams(vec![ParamType::Param(build_path_param(indent))])
    }
}

impl ToRouterParamsByType for axum::extract::Path<i32> {
    fn to_router_parameters(indent: &str) -> RouterParams {
        RouterParams(vec![ParamType::Param(build_path_param(indent))])
    }
}

impl ToRouterParamsByType for axum::extract::Path<i64> {
    fn to_router_parameters(indent: &str) -> RouterParams {
        RouterParams(vec![ParamType::Param(build_path_param(indent))])
    }
}

impl<T: ToParamDto> ToRouterParamsByType for axum::extract::Path<T> {
    fn to_router_parameters(_: &str) -> RouterParams {
        let t = T::to_param_dto(ParamDtoIn::Param(utoipa::openapi::path::ParameterIn::Path));
        if let ParamDto::ParamList(mut parameters) = t {
            RouterParams(parameters.drain(..).map(ParamType::Param).collect())
        } else {
            RouterParams(vec![])
        }
    }
}

impl<T: ToParamDto> ToRouterParamsByType for axum::extract::Query<T> {
    fn to_router_parameters(_: &str) -> RouterParams {
        let t = T::to_param_dto(ParamDtoIn::Param(utoipa::openapi::path::ParameterIn::Query));
        if let ParamDto::ParamList(mut parameters) = t {
            RouterParams(parameters.drain(..).map(ParamType::Param).collect())
        } else {
            RouterParams(vec![])
        }
    }
}

impl<K, V> ToParamDto for HashMap<K, V> {}

impl<T: ToParamDto> ToRouterParamsByType for axum::extract::Json<T> {
    fn to_router_parameters(_: &str) -> RouterParams {
        let t = T::to_param_dto(ParamDtoIn::Body);
        if let ParamDto::BodySchema(schema) = t {
            // let ref_scheme = Ref::new(format!("#/components/schemas/{}", schema.0));
            // RouterParams(vec![ParamType::Body(
            //     RequestBodyBuilder::new().content("application/json", ContentBuilder::new().schema(ref_scheme).build()).build(),
            //     Some(schema),
            // )])
            RouterParams(vec![ParamType::Body(BodySchema { content_type: "application/json", schema: Some(schema) })])
        } else {
            RouterParams(vec![])
        }
    }
}

impl<T: ToParamDto> ToRouterParamsByType for axum::extract::Form<T> {
    fn to_router_parameters(_: &str) -> RouterParams {
        let t = T::to_param_dto(ParamDtoIn::Body);
        if let ParamDto::BodySchema(schema) = t {
            // let ref_scheme: Ref = Ref::new(format!("#/components/schemas/{}", schema.0));
            // RouterParams(vec![ParamType::Body(
            //     RequestBodyBuilder::new().content("application/x-www-form-urlencoded", ContentBuilder::new().schema(ref_scheme).build()).build(),
            //     Some(schema),
            // )])
            RouterParams(vec![ParamType::Body(BodySchema { content_type: "application/x-www-form-urlencoded", schema: Some(schema) })])
        } else {
            RouterParams(vec![])
        }
    }
}

impl ToRouterParamsByType for Meta {}

impl<T> ToRouterParamsByType for axum::extract::Extension<T> {}

impl ToRouterParamsByType for axum::extract::Host {}

impl ToRouterParamsByType for axum::extract::MatchedPath {}

impl ToRouterParamsByType for axum::extract::NestedPath {}

impl ToRouterParamsByType for axum::extract::OriginalUri {}

impl ToRouterParamsByType for axum::extract::RawForm {}

impl ToRouterParamsByType for axum::extract::RawPathParams {}

impl ToRouterParamsByType for axum::extract::RawQuery {}

impl<T> ToRouterParamsByType for axum::extract::Request<T> {}

impl<T> ToRouterParamsByType for axum::extract::State<T> {}

impl<T> ToRouterParamsByType for axum::extract::WebSocketUpgrade<T> {}

impl<T> ToRouterParamsByType for axum::response::AppendHeaders<T> {}

impl ToRouterParamsByType for String {
    fn to_router_parameters(_: &str) -> RouterParams {
        RouterParams(vec![ParamType::Body(BodySchema { content_type: "text/plain", schema: None })])
    }
}

impl<T: ToRouterParamsByType, E> ToRouterParamsByType for Result<T, E> {
    fn to_router_parameters(indent: &str) -> RouterParams {
        T::to_router_parameters(indent)
    }
}

macro_rules! impl_for_tuples {
    // 递归结束条件：空元组
    () => {
        impl ToRouterParamsByType for () {}
    };
    // 递归实现：匹配元组类型
    ($T:ident $(, $Ts:ident)*) => {
        impl<$T: ToRouterParamsByType, $($Ts: ToRouterParamsByType),*> ToRouterParamsByType for ($T, $($Ts),*) {
            fn to_router_parameters(indent: &str) -> RouterParams {
                $T::to_router_parameters(indent).comb::<($($Ts),*)>(indent)  // 调用元组成员的 to_router_parameters 方法，然后合并
            }
        }
        impl_for_tuples!($($Ts),*);  // 递归调用宏
    };
}

impl_for_tuples!(T1, T2, T3, T4, T5);

impl<T: ToParamDto> ToParamDto for std::sync::Arc<T> {
    fn to_param_dto(param: ParamDtoIn) -> ParamDto {
        T::to_param_dto(param)
    }
}
