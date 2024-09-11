use std::collections::HashMap;

use nidrs_extern::{datasets::MetaKey, meta::Meta};
use utoipa::openapi::{
    path::Parameter,
    request_body::{RequestBody, RequestBodyBuilder},
    ContentBuilder, Ref,
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

#[derive(Clone)]
pub enum ParamDto {
    None,
    ParamList(Vec<Parameter>),
    BodySchema((&'static str, utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>)),
}

#[derive(Clone)]
pub enum ParamDtoIn {
    Param(utoipa::openapi::path::ParameterIn),
    Body,
}

#[derive(Clone)]
pub enum ParamType {
    Param(Parameter),
    Body(RequestBody, Option<(&'static str, utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>)>),
}

impl std::fmt::Debug for ParamType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Param(arg0) => f.debug_tuple("Parameter").finish(),
            Self::Body(arg0, arg1) => f.debug_tuple("Body").finish(),
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct RouterParams(pub Vec<ParamType>);

impl RouterParams {
    pub fn value(&self) -> &Vec<ParamType> {
        &self.0
    }
    pub fn merge(mut self, other: RouterParams) -> Self {
        self.0.extend(other.0);
        self
    }
    pub fn merge_type<T: ToRouterParamsByType>(mut self) -> Self {
        let other = T::to_router_parameters();
        self.0.extend(other.0);
        self
    }
}

impl MetaKey for RouterParams {
    fn meta_key() -> String {
        "RouterParams".to_string()
    }
}

pub trait ToRouterParamsByType {
    fn to_router_parameters() -> RouterParams {
        RouterParams(vec![])
    }
}

pub trait ToParamDto {
    fn to_param_dto(dto_type: ParamDtoIn) -> ParamDto {
        ParamDto::None
    }
}

impl<T: ToParamDto> ToRouterParamsByType for axum::extract::Path<T> {
    fn to_router_parameters() -> RouterParams {
        let t = T::to_param_dto(ParamDtoIn::Param(utoipa::openapi::path::ParameterIn::Path));
        if let ParamDto::ParamList(mut parameters) = t {
            RouterParams(parameters.drain(..).map(ParamType::Param).collect())
        } else {
            RouterParams(vec![])
        }
    }
}

impl<T: ToParamDto> ToRouterParamsByType for axum::extract::Query<T> {
    fn to_router_parameters() -> RouterParams {
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
    fn to_router_parameters() -> RouterParams {
        let t = T::to_param_dto(ParamDtoIn::Body);
        if let ParamDto::BodySchema(schema) = t {
            let ref_scheme = Ref::new(format!("#/components/schemas/{}", schema.0));
            RouterParams(vec![ParamType::Body(
                RequestBodyBuilder::new().content("application/json", ContentBuilder::new().schema(ref_scheme).build()).build(),
                Some(schema),
            )])
        } else {
            RouterParams(vec![])
        }
    }
}

impl<T: ToParamDto> ToRouterParamsByType for axum::extract::Form<T> {
    fn to_router_parameters() -> RouterParams {
        let t = T::to_param_dto(ParamDtoIn::Body);
        if let ParamDto::BodySchema(schema) = t {
            let ref_scheme = Ref::new(format!("#/components/schemas/{}", schema.0));
            RouterParams(vec![ParamType::Body(
                RequestBodyBuilder::new().content("application/x-www-form-urlencoded", ContentBuilder::new().schema(ref_scheme).build()).build(),
                Some(schema),
            )])
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

impl ToRouterParamsByType for String {
    fn to_router_parameters() -> RouterParams {
        RouterParams(vec![ParamType::Body(RequestBodyBuilder::new().content("text", ContentBuilder::new().build()).build(), None)])
    }
}

impl<T: ToRouterParamsByType, E> ToRouterParamsByType for Result<T, E> {
    fn to_router_parameters() -> RouterParams {
        T::to_router_parameters()
    }
}
