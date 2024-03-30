use nidrs_macro::injectable;

#[injectable()]
#[derive(Clone, Debug)]
pub struct ConfService {
  pub log_level: String,
}

impl Default for ConfService {
  fn default() -> Self {
    ConfService {
      log_level: "info".to_string(),
    }
  }
}