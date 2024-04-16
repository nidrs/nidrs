use crate::ModuleCtx;

pub trait ControllerService {
  fn inject(&self, ctx: ModuleCtx) -> ModuleCtx;
}

