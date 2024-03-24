use std::sync::Arc;

use nestrs_macro::get_route_meta;

mod app;
mod user;

fn main() {
    println!("Hello, world!");

    get_route_meta!(println!("Hello, Nestrs!"));

    let app_state = AppState{};

    let app = nestrs::NestFactory::create(app::AppModule, app_state).listen::<AppError>(3000);
    let _ = tokio::runtime::Runtime::new().unwrap().block_on(app);
}



#[derive(Clone, Debug, Default)]
pub struct AppState{}

pub enum AppError {
    
}


impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        // Convert std::io::Error to AppError here
        // Example: AppError::new(error.to_string())
        unimplemented!()
    }
}