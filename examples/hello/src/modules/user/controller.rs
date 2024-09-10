use std::collections::HashMap;

use nidrs::macros::{controller, get, meta};
use nidrs::openapi::api;
use nidrs::{externs::axum::extract::Query, post};
use nidrs::{AppResult, Inject};
use nidrs_extern::axum::extract::Path;
use nidrs_extern::axum::Json;

use super::dto::FilterDto;
use super::{dto::CreateUserDto, dto::UserByIdDto, service::UserService};

#[controller("/user")]
pub struct UserController {
    user_service: Inject<UserService>,
}

impl UserController {
    #[api]
    #[get("/")]
    pub async fn get_all(&self, Query(q): Query<HashMap<String, String>>) -> AppResult<String> {
        println!("Query {:?}", q);
        // self.log_service.log("hello");
        Ok(self.user_service.extract().get_hello_world2())
    }

    #[api]
    #[get("/:id")]
    pub async fn get_one(&self, id: Path<UserByIdDto>, query: Query<FilterDto>) -> AppResult<String> {
        Ok(format!("get one! id: {}", id.id))
    }

    #[api]
    #[post("/")]
    pub async fn create_user(&self, query: Query<FilterDto>, dto: Json<CreateUserDto>) -> AppResult<String> {
        Ok(self.user_service.extract().get_hello_world2())
    }
}
