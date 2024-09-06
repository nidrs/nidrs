use std::collections::HashMap;

use axum::extract::Query;
use nidrs::Inject;
use nidrs_macro::{controller, get};

use crate::AppResult;

use super::service::AppService;

#[controller("/app")]
pub struct AppController {
    app_service: Inject<AppService>,
}

impl AppController {
    #[get("/hello")]
    pub async fn get_hello_world(&self, Query(q): Query<HashMap<String, String>>) -> AppResult<String> {
        // println!("Query {:?}", q);
        // println!("Meta {:?}", meta.get::<&str>("role"));

        // fn_test()?;
        Ok(self.app_service.get_hello_world2())
    }
}
