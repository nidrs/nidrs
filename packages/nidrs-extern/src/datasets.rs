#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisableDefaultPrefix(pub bool);

impl DisableDefaultPrefix {
    pub fn value(&self) -> bool {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Global(pub bool);

impl Global {
    pub fn value(&self) -> bool {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ServiceName(pub &'static str);

impl ServiceName {
    pub fn value(&self) -> &'static str {
        self.0
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

impl From<&str> for ServiceType {
    fn from(value: &str) -> Self {
        match value {
            "Service" => ServiceType::Service,
            "ControllerService" => ServiceType::Controller,
            "InterceptorService" => ServiceType::Interceptor,
            _ => panic!("Invalid service type"),
        }
    }
}

impl From<ServiceType> for &'static str {
    fn from(val: ServiceType) -> Self {
        match val {
            ServiceType::Service => "Service",
            ServiceType::Controller => "ControllerService",
            ServiceType::Interceptor => "InterceptorService",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModuleName(pub &'static str);

impl ModuleName {
    pub fn value(&self) -> &'static str {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ControllerPath(pub &'static str);

impl ControllerPath {
    pub fn value(&self) -> &'static str {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RouterPath(pub &'static str);

impl RouterPath {
    pub fn value(&self) -> &'static str {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RouterMethod(pub &'static str);

impl RouterMethod {
    pub fn value(&self) -> &'static str {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RouterName(pub &'static str);

impl RouterName {
    pub fn value(&self) -> &'static str {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RouterFullPath(pub String);

impl RouterFullPath {
    pub fn value(&self) -> &String {
        &self.0
    }
}

pub struct RouterBodyScheme(pub (&'static str, utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>));

impl RouterBodyScheme {
    pub fn value(&self) -> &(&'static str, utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>) {
        &self.0
    }

    pub fn from_dto_type<T: utoipa::ToSchema<'static>>() -> Self {
        RouterBodyScheme(T::schema())
    }
}
