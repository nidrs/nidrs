use nidrs_macro::injectable;

#[derive(Debug, Default)]
pub struct ConfOptions {
    pub log_level: String,
}

#[injectable()]
#[derive(Debug, Default)]
pub struct ConfOptionsProvider {
    pub options: ConfOptions,
}

impl ConfOptionsProvider {
    pub fn new(options: ConfOptions) -> Self {
        ConfOptionsProviderInner { options }.into()
    }
}
