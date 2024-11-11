use nidrs::{Module, ModuleCtx};

pub struct AppModule;

impl Module for AppModule {
    fn init(self, ctx: ModuleCtx) -> ModuleCtx {
        ctx
    }

    fn destroy(&self, ctx: &ModuleCtx) {}
}
