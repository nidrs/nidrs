use nidrs::macros::injectable;

#[injectable()]
#[derive(Debug)]
pub struct ConfOptions {
    pub log_level: String,
}
