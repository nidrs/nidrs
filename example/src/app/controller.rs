#![allow(warnings, unused)]
use std::{collections::HashSet, rc::Rc};

use axum::extract::State;
use nestrs_macro::{controller, get};

use crate::AppState;

use super::{service, Ctx};
use std::sync::Arc;

#[controller("/app")]
#[derive(Clone, Debug, Default)]
pub struct AppController {
    pub app_service: Arc<service::AppService>,
}

impl AppController {
    #[get("/hello-world")]
    pub async fn get_hello_world(&self, State(state): State<AppState>) -> String {
        self.app_service.get_hello_world()
    }
}

impl AppController {
    pub fn register(self, router: axum::Router<AppState>) -> Arc<Self> {
        let that = Arc::new(self);
        let cloned_that = Arc::clone(&that); // Clone the Arc before using it in the closure
        router.merge(axum::Router::new().route(
            "/app/hello-world",
            axum::routing::get(move |state| {
                let cloned_that = Arc::clone(&cloned_that); // Clone the Arc again inside the closure
                async move {
                    cloned_that.get_hello_world(state).await
                }
            }),
        ));
        that
    }
}

impl nestrs::Controller for AppController {}
