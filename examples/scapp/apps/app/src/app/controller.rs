use std::collections::HashMap;

use macros::user;
use nidrs::externs::axum::{extract::Query, response::AppendHeaders, Json};
use nidrs::macros::{controller, get, post};
use nidrs::{Inject, Meta};

use crate::AppResult;

use super::{dto::Status, service::AppService};

#[controller()]
pub struct AppController {
    app_service: Inject<AppService>,
}

impl AppController {
    #[user]
    #[get("/hello")]
    pub async fn get_hello_world(
        &self,
        meta: Meta,
        Query(q): Query<HashMap<String, String>>,
    ) -> AppResult<(AppendHeaders<[(String, String); 2]>, Status)> {
        println!("Query {:?}", q);
        println!("Meta {:?}", meta.get_data::<datasets::role::Role>());

        Ok((
            AppendHeaders([
                ("X-Custom-Header".to_string(), "hello".to_string()),
                ("X-Custom-Header".to_string(), "world".to_string()),
            ]),
            Status {
                db: "ok".to_string(),
                redis: "ok".to_string(),
            },
        ))
    }
}
