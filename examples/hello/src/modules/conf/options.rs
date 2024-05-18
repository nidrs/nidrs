use nidrs_macro::injectable;

#[injectable()]
#[derive(Debug)]
pub struct ConfOptions {
    pub log_level: String,
}
