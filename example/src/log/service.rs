use nidrs_macro::injectable;

#[injectable()]
#[derive(Default)]
pub struct LogService {
}

impl LogService {

  pub fn log(&self, msg: &str) {
    println!("[Log] {}", msg);
  }
}