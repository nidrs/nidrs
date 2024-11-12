use single_hello::{Controller, Creator, Module, ModuleCtx, Svc};
use axum::Router;

pub struct AppModule;

impl Creator for AppModule {
    fn create() -> Self {
        Self
    }
}

impl Module for AppModule {
    fn init(&self, ctx: ModuleCtx) -> ModuleCtx {
        ctx.register_controller::<AppController>("crate::app::AppModule");
        ctx
    }

    fn destroy(&self, ctx: &ModuleCtx) {}
}


#[derive(Clone, Debug)]
pub struct AppController {

}

impl Creator for AppController {
    fn create() -> Self {
        Self {}
    }
}
impl Svc for AppController {
    fn register(&self, ctx: &ModuleCtx) {
        self.register_router_hello(ctx);
    }
}
impl Controller for AppController {}

impl AppController {
    pub fn hello(&self) {
        println!("hello");
    }

    pub fn register_router_hello(&self, ctx: &ModuleCtx) {
        let that = ctx.get_svc::<Self>();
        ctx.register_router(Router::new().route("/hello", axum::routing::get( || async move { 
            that.hello();
            "hello"
        })));
    }

}
