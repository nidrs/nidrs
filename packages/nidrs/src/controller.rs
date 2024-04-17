use crate::{ModuleCtx, ServiceProperty};

pub trait ControllerService {
  fn inject(&self, ctx: ModuleCtx) -> ModuleCtx;
  fn property() -> ServiceProperty;
}