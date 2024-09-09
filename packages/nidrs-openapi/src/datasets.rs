use nidrs_extern::datasets::MetaKey;
use utoipa::openapi::{
    path::Parameter,
    request_body::{RequestBody, RequestBodyBuilder},
    ContentBuilder, Ref,
};

pub enum ParamType {
    Parameter(Parameter),
    RequestBody(RequestBody, (&'static str, utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>)),
}

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
}

impl MetaKey for RouterParams {
    fn meta_key() -> String {
        "RouterParams".to_string()
    }
}

pub trait ToRouterParams {
    fn to_router_parameters() -> RouterParams;
}

pub fn to_router_parameters<T: ToRouterParams>() -> RouterParams {
    T::to_router_parameters()
}

impl<T: utoipa::IntoParams> ToRouterParams for axum::extract::Path<T> {
    fn to_router_parameters() -> RouterParams {
        RouterParams(T::into_params(|| Some(utoipa::openapi::path::ParameterIn::Path)).drain(..).map(ParamType::Parameter).collect())
    }
}
impl<T: utoipa::IntoParams> ToRouterParams for axum::extract::Query<T> {
    fn to_router_parameters() -> RouterParams {
        RouterParams(T::into_params(|| Some(utoipa::openapi::path::ParameterIn::Query)).drain(..).map(ParamType::Parameter).collect())
    }
}
impl<T: utoipa::ToSchema<'static>> ToRouterParams for axum::extract::Json<T> {
    fn to_router_parameters() -> RouterParams {
        let scheme = T::schema();
        let ref_scheme = Ref::new(format!("#/components/schemas/{}", scheme.0));
        RouterParams(vec![ParamType::RequestBody(
            RequestBodyBuilder::new().content("application/json", ContentBuilder::new().schema(ref_scheme).build()).build(),
            scheme,
        )])
    }
}
