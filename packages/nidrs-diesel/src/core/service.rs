use nidrs::on_module_destroy;
use nidrs_macro::{injectable, on_module_init};

#[injectable()]
pub struct DieselService {}

impl DieselService {
    #[on_module_init()]
    pub fn on_module_init(&self) {
        // let options = self.options.extract();
        // println!("DieselService initialized");
    }

    #[on_module_destroy()]
    pub fn on_module_destroy(&self) {
        // println!("DieselService destroyed");
    }
}
