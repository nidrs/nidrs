use nidrs::Inject;
use nidrs_macro::{injectable, on_module_init};

use super::options::ConfOptions;

#[injectable()]
#[derive(Clone, Debug, Default)]
pub struct ConfService {
  // pub options: Inject<ConfOptions>,
  pub log_level: String,
}

impl ConfService {
  #[on_module_init()]
  pub fn on_module_init(&self){
    
  }
}