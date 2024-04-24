use nidrs_macro::injectable;

#[injectable()]
#[derive(Debug, Default)]
pub struct ConfOptions {
    pub log_level: String,
}
