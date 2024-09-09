use std::collections::HashMap;

use nidrs_extern::{datasets::MetaKey, meta::Meta};
use utoipa::openapi::{
    self,
    path::Parameter,
    request_body::{RequestBody, RequestBodyBuilder},
    ContentBuilder, Ref,
};

#[derive(Clone)]
pub enum ParamDto {
    None,
    Parameters(Vec<Parameter>),
    RequestBodies((&'static str, utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>)),
}

#[derive(Clone)]
pub enum ParamDtoType {
    Parameter(utoipa::openapi::path::ParameterIn),
    RequestBody,
}

#[derive(Clone)]
pub enum ParamType {
    Parameter(Parameter),
    RequestBody(RequestBody, (&'static str, utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>)),
}

#[derive(Clone, Default)]
pub struct RouterParams(pub Vec<ParamType>);

impl std::fmt::Debug for RouterParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("RouterParams").finish()
    }
}

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
    fn to_param_dto(dto_type: ParamDtoType) -> ParamDto {
        ParamDto::None
    }
}

impl<T: ToParamDto> ToRouterParamsByType for axum::extract::Path<T> {
    fn to_router_parameters() -> RouterParams {
        let t = T::to_param_dto(ParamDtoType::Parameter(utoipa::openapi::path::ParameterIn::Path));
        if let ParamDto::Parameters(mut parameters) = t {
            RouterParams(parameters.drain(..).map(ParamType::Parameter).collect())
        } else {
            RouterParams(vec![])
        }
    }
}

impl<T: ToParamDto> ToRouterParamsByType for axum::extract::Query<T> {
    fn to_router_parameters() -> RouterParams {
        let t = T::to_param_dto(ParamDtoType::Parameter(utoipa::openapi::path::ParameterIn::Query));
        if let ParamDto::Parameters(mut parameters) = t {
            RouterParams(parameters.drain(..).map(ParamType::Parameter).collect())
        } else {
            RouterParams(vec![])
        }
    }
}

impl<K, V> ToParamDto for HashMap<K, V> {}

impl<T: ToParamDto> ToRouterParamsByType for axum::extract::Json<T> {
    fn to_router_parameters() -> RouterParams {
        // let scheme = T::schema();
        // let ref_scheme = Ref::new(format!("#/components/schemas/{}", scheme.0));
        // RouterParams(vec![ParamType::RequestBody(
        //     RequestBodyBuilder::new().content("application/json", ContentBuilder::new().schema(ref_scheme).build()).build(),
        //     scheme,
        // )])

        let t = T::to_param_dto(ParamDtoType::RequestBody);
        if let ParamDto::RequestBodies(scheme) = t {
            let ref_scheme = Ref::new(format!("#/components/schemas/{}", scheme.0));
            RouterParams(vec![ParamType::RequestBody(
                RequestBodyBuilder::new().content("application/json", ContentBuilder::new().schema(ref_scheme).build()).build(),
                scheme,
            )])
        } else {
            RouterParams(vec![])
        }
    }
}

// impl<T: ToParamDto> ToRouterParamsByType for axum::extract::Form<T> {
//     fn to_router_parameters() -> RouterParams {
//         // let scheme = T::schema();
//         // let ref_scheme = Ref::new(format!("#/components/schemas/{}", scheme.0));
//         // RouterParams(vec![ParamType::RequestBody(
//         //     RequestBodyBuilder::new().content("application/x-www-form-urlencoded", ContentBuilder::new().schema(ref_scheme).build()).build(),
//         //     scheme,
//         // )])
//         RouterParams(T::to_param_dto())
//     }
// }

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
