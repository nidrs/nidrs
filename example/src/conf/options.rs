use nidrs_macro::injectable;


#[injectable()]
#[derive(Clone, Debug, Default)]
pub struct ConfOptions{
    pub log_level: String,
}