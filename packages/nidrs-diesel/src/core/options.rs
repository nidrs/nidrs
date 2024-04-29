use nidrs_macro::injectable;

#[injectable()]
#[derive(Default)]
pub struct DieselOptions {
    pub log_level: String,
}
