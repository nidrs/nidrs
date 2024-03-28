#![allow(warnings, unused)]
use std::{collections::HashSet, rc::Rc};

use axum::{extract::State, Router};
use nestrs::Inject;
use nestrs_macro::{controller, get};

use crate::AppState;

use super::{service::AppService, StateCtx};
use std::sync::Arc;

#[controller("/app")]
#[derive(Debug, Default)]
pub struct AppController {
    pub app_service: Inject<AppService>,
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
    fn register(self) -> Router<StateCtx> {
        let router = axum::Router::new();
        let that = Arc::new(self);
        let cloned_that = Arc::clone(&that); // Clone the Arc before using it in the closure
        router.merge(axum::Router::new().route(
            "/app/hello",
            axum::routing::get(move |state| {
                let cloned_that: Arc<AppController> = Arc::clone(&cloned_that); // Clone the Arc again inside the closure
                async move {
                    cloned_that.get_hello_world(state).await
                }
            }),
        ))
    }

    fn inject(&self, ctx: &nestrs::ModuleCtx) {
        let binding = ctx.services.clone();
        let binding = binding.lock().unwrap();
        let app_service = binding.get("AppService").unwrap();
        let app_service = app_service.downcast_ref::<Arc<AppService>>().unwrap();
        self.app_service.inject(app_service.clone());
    }
}