#![allow(warnings, unused)]
use std::{any::Any, collections::{HashMap, HashSet}, rc::Rc, sync::{Mutex, MutexGuard}};

use axum::{extract::State, Router};
use nestrs::Inject;
use nestrs_macro::{controller, get};

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
    pub async fn get_hello_world(&self, State(state): State<StateCtx>) -> String {
        let app_service = self.app_service.lock().unwrap();
        let app_service = app_service.as_ref().unwrap();
        app_service.get_hello_world2()
    }
    #[get("/hello2")]
    pub async fn get_hello_world2(&self, State(state): State<StateCtx>) -> String {
        "Hello, World2!".to_string()
    }
}


impl nestrs::Controller for AppController {
    fn inject(&self, services: &MutexGuard<HashMap<String, Box<dyn Any>>>) {
        let app_service = services.get("AppService").unwrap();
        let app_service = app_service.downcast_ref::<Arc<AppService>>().unwrap();
        self.app_service.inject(app_service.clone());
    }
}