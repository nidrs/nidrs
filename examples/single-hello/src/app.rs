use nidrs::{Module, ModuleCtx};

pub struct AppModule;

impl Module for AppModule {
    fn init(self, mut ctx: ModuleCtx) -> ModuleCtx {
        if !ctx.register_module("AppModule", Box::new(self)) {
            return ctx;
        }

        ctx
    }

    fn destroy(&self, ctx: &ModuleCtx) {}
}
