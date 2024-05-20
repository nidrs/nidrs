use nidrs::macros::injectable;

#[injectable()]
pub struct LogService {}

impl LogService {
    pub fn log(&self, msg: &str) {
        println!("[Log] {}", msg);
    }
}
