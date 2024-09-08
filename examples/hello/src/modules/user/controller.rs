use std::collections::HashMap;

use nidrs::datasets::to_router_parameters;
use nidrs::macros::{controller, get, meta};
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
    #[get("/")]
    pub async fn get_all(&self, Query(q): Query<HashMap<String, String>>) -> AppResult<String> {
        println!("Query {:?}", q);
        // self.log_service.log("hello");
        Ok(self.user_service.extract().get_hello_world2())
    }

    #[meta(nidrs::shared::block({
        to_router_parameters::<Path<UserByIdDto>>()
        .merge(to_router_parameters::<Query<FilterDto>>())
    }))]
    #[get("/:id")]
    pub async fn get_one(&self, id: Path<UserByIdDto>, query: Query<FilterDto>) -> AppResult<String> {
        Ok(format!("get one! id: {}", id.id))
    }

    #[meta(nidrs::shared::block({
        to_router_parameters::<Path<UserByIdDto>>()
        .merge(to_router_parameters::<Query<FilterDto>>())
        .merge(to_router_parameters::<Json<CreateUserDto>>())
    }))]
    #[post("/:id")]
    pub async fn create_user(&self, query: Query<FilterDto>, dto: Json<CreateUserDto>) -> AppResult<String> {
        Ok(self.user_service.extract().get_hello_world2())
    }
}
