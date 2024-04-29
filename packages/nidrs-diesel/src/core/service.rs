use nidrs::{on_module_destroy, Inject};
use nidrs_macro::{injectable, on_module_init};

use super::options::DieselOptions;

#[injectable()]
#[derive(Default)]
pub struct DieselService {
    pub options: Inject<DieselOptions>,
    pub log_level: String,
}

impl DieselService {
    #[on_module_init()]
    pub fn on_module_init(&self) {
        let options = self.options.extract();
        println!("DieselService initialized with log_level: {:?}", options.log_level);
    }

    #[on_module_destroy()]
    pub fn on_module_destroy(&self) {
        println!("DieselService destroyed");
    }
}
