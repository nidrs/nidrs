pub use nidrs_macro::*;

pub mod result;
pub use result::*;

pub mod service;
pub use service::*;

pub mod interceptor;
pub use interceptor::*;

pub mod controller;
pub use controller::*;

pub mod module;
pub use module::*;

pub mod shared;

pub use nidrs_extern::datasets;
pub use nidrs_extern::meta::*;

pub use nidrs_extern::router;
pub use nidrs_extern::router::*;

pub use nidrs_extern as externs;
pub use nidrs_macro as macros;

#[cfg(feature = "valid")]
pub use nidrs_valid as valid;

#[cfg(feature = "openapi")]
pub use nidrs_openapi as openapi;
