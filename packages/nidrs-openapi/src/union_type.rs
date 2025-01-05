use serde::Serialize;
use utoipa::ToSchema;

pub trait UToSchema {
    fn clone_box(&self) -> Box<dyn UToSchema>;
}

impl<T: ToSchema + Serialize + Clone + 'static> UToSchema for T {
    fn clone_box(&self) -> Box<dyn UToSchema> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn UToSchema> {
    fn clone(&self) -> Self {
        self.as_ref().clone_box()
    }
}
