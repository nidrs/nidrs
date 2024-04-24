use nidrs::{on_module_destroy, Inject};
use nidrs_macro::{injectable, on_module_init};

use super::options::ConfOptionsProvider;

#[injectable()]
#[derive(Debug, Default)]
pub struct ConfService {
    pub options: Inject<ConfOptionsProvider>,
    pub log_level: String,
}

impl ConfService {
    #[on_module_init()]
    pub fn on_module_init(&self) {
        println!("ConfService initialized with log_level: {:?}", self.options.options);
    }

    #[on_module_destroy()]
    pub fn on_module_destroy(&self) {
        println!("ConfService destroyed");
    }
}
