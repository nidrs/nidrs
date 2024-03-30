#![allow(warnings, unused)]
use std::{any::Any, collections::{HashMap, HashSet}, rc::Rc, sync::{Mutex, MutexGuard}};

use axum::{extract::{Query, State}, Json, Router};
use nestrs::Inject;
use nestrs_macro::{controller, get, post};

use crate::AppState;

use super::{service::AppService, StateCtx};
use std::sync::Arc;

#[controller("/app")]
#[derive(Debug, Default)]
pub struct AppController {
    app_service: Inject<AppService>,
}

impl AppController {
    #[get("/hello")]
    pub async fn get_hello_world(&self, State(state): State<StateCtx>, Query(q): Query<HashMap<String, String>>) -> String {
        println!("Query {:?}", q);
        let app_service = self.app_service.lock().unwrap();
        let app_service = app_service.as_ref().unwrap();
        app_service.get_hello_world()
    }
    #[post("/hello")]
    pub async fn get_hello_world2(&self, State(state): State<StateCtx>, Query(q): Query<HashMap<String, String>>, Json(j): Json<serde_json::Value>) -> String {
        println!("Query {:?}", q);
        println!("Json {:?}", j);

        "Hello, World2!".to_string()
    }
}
