use nidrs_extern::{*};
use std::{any::Any, collections::HashMap, sync::MutexGuard};

pub trait ControllerService {
  fn inject(&self, services: &MutexGuard<HashMap<String, Box<dyn Any>>>);
}

