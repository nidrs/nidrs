use nidrs::macros::{injectable, on_module_init};
use nidrs::{on_module_destroy, Inject};

use super::options::ConfOptions;

#[injectable()]
#[derive(Debug)]
pub struct ConfService {
    pub options: Inject<ConfOptions>,
    pub log_level: String,
}

impl ConfService {
    #[on_module_init()]
    pub fn on_module_init(&self) {
        let options = self.options.extract();
        println!("ConfService initialized with log_level: {:?}", options);
    }

    #[on_module_destroy()]
    pub fn on_module_destroy(&self) {
        println!("ConfService destroyed");
    }
}
