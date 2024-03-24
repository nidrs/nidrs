#![allow(warnings, unused)]
use std::{collections::HashSet, rc::Rc};

use axum::{extract::State, Router};
use nestrs::Inject;
use nestrs_macro::{controller, get};

use crate::AppState;

use super::{service, StateCtx};
use std::sync::Arc;

#[controller("/app")]
#[derive(Clone, Debug, Default)]
pub struct AppController {
    pub app_service: Inject<service::AppService>,
}

impl AppController {
    #[get("/hello")]
    pub async fn get_hello_world(&self, State(state): State<StateCtx>) -> String {
        self.app_service.get_hello_world()
    }
    #[get("/hello2")]
    pub async fn get_hello_world2(&self, State(state): State<StateCtx>) -> String {
        self.app_service.get_hello_world()
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
}