use std::sync::{Arc, Mutex};

use nidrs::Inject;
use nidrs::macros::injectable;

use crate::app::service::AppService;

#[injectable()]
pub struct UserService {
    app_service: Inject<AppService>,
    count: Arc<Mutex<i32>>,
}

impl UserService {
    pub fn get_hello_world(&self) -> String {
        self.app_service.extract().get_hello_world2()
    }

    pub fn get_hello_world2(&self) -> String {
        let mut count = self.count.lock().unwrap();
        *count += 1;
        format!("Hello, World! {}", count)
    }
}
