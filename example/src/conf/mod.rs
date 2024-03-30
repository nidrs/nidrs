pub mod service;

use nidrs_macro::module;

use service::ConfService;

#[module({
  services = [ConfService];
  exports = [ConfService];
})]
#[derive(Clone, Debug, Default)]
pub struct ConfModule;