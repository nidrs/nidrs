use utoipa::openapi::{
    path::Parameter,
    request_body::{RequestBody, RequestBodyBuilder},
    ContentBuilder, Ref,
};

pub trait MetaKey {
    fn meta_key() -> String;
}

pub fn get_meta_key<T: MetaKey>(_: T) -> String {
    T::meta_key()
}

pub fn get_meta_key_by_ref<T: MetaKey>(_: &T) -> String {
    T::meta_key()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisableDefaultPrefix(pub bool);

impl DisableDefaultPrefix {
    pub fn value(&self) -> bool {
        self.0
    }
}

impl MetaKey for DisableDefaultPrefix {
    fn meta_key() -> String {
        "DisableDefaultPrefix".to_string()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Global(pub bool);

impl Global {
    pub fn value(&self) -> bool {
        self.0
    }
}

impl MetaKey for Global {
    fn meta_key() -> String {
        "Global".to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceName(pub String);

impl ServiceName {
    pub fn value(&self) -> &String {
        &self.0
    }
}

impl MetaKey for ServiceName {
    fn meta_key() -> String {
        "ServiceName".to_string()
    }
}

impl From<&str> for ServiceName {
    fn from(value: &str) -> Self {
        ServiceName(value.to_string())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceType {
    Service,
    Controller,
    Interceptor,
}

impl ServiceType {
    pub fn value(&self) -> &Self {
        self
    }
}

impl MetaKey for ServiceType {
    fn meta_key() -> String {
        "ServiceType".to_string()
    }
}

impl From<&str> for ServiceType {
    fn from(value: &str) -> Self {
        match value {
            "Service" => ServiceType::Service,
            "Controller" => ServiceType::Controller,
            "Interceptor" => ServiceType::Interceptor,
            _ => panic!("Invalid service type"),
        }
    }
}

impl From<ServiceType> for &'static str {
    fn from(val: ServiceType) -> Self {
        match val {
            ServiceType::Service => "Service",
            ServiceType::Controller => "Controller",
            ServiceType::Interceptor => "Interceptor",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleName(pub String);

impl ModuleName {
    pub fn value(&self) -> &String {
        &self.0
    }
}

impl MetaKey for ModuleName {
    fn meta_key() -> String {
        "ModuleName".to_string()
    }
}

impl From<&str> for ModuleName {
    fn from(value: &str) -> Self {
        ModuleName(value.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControllerPath(pub String);

impl ControllerPath {
    pub fn value(&self) -> &String {
        &self.0
    }
}

impl MetaKey for ControllerPath {
    fn meta_key() -> String {
        "ControllerPath".to_string()
    }
}

impl From<&str> for ControllerPath {
    fn from(value: &str) -> Self {
        ControllerPath(value.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RouterPath(pub String);

impl RouterPath {
    pub fn value(&self) -> &String {
        &self.0
    }
}

impl MetaKey for RouterPath {
    fn meta_key() -> String {
        "RouterPath".to_string()
    }
}

impl From<&str> for RouterPath {
    fn from(value: &str) -> Self {
        RouterPath(value.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RouterMethod(pub String);

impl RouterMethod {
    pub fn value(&self) -> &String {
        &self.0
    }
}

impl MetaKey for RouterMethod {
    fn meta_key() -> String {
        "RouterMethod".to_string()
    }
}

impl From<&str> for RouterMethod {
    fn from(value: &str) -> Self {
        RouterMethod(value.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RouterName(pub String);

impl RouterName {
    pub fn value(&self) -> &String {
        &self.0
    }
}

impl MetaKey for RouterName {
    fn meta_key() -> String {
        "RouterName".to_string()
    }
}

impl From<&str> for RouterName {
    fn from(value: &str) -> Self {
        RouterName(value.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RouterFullPath(pub String);

impl RouterFullPath {
    pub fn value(&self) -> &String {
        &self.0
    }
}

impl MetaKey for RouterFullPath {
    fn meta_key() -> String {
        "RouterFullPath".to_string()
    }
}

impl From<&str> for RouterFullPath {
    fn from(value: &str) -> Self {
        RouterFullPath(value.to_string())
    }
}

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
