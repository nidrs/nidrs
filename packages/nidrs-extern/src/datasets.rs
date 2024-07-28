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

pub struct RouterBodyScheme(pub (&'static str, utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>));

impl std::fmt::Debug for RouterBodyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("RouterBodyScheme").finish()
    }
}

impl RouterBodyScheme {
    pub fn value(&self) -> &(&'static str, utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>) {
        &self.0
    }

    pub fn from_type<T: utoipa::ToSchema<'static>>() -> Self {
        RouterBodyScheme(T::schema())
    }
}

impl MetaKey for RouterBodyScheme {
    fn meta_key() -> String {
        "RouterBodyScheme".to_string()
    }
}
