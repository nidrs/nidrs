mod app;
mod user;

fn main() {
    let app_state = AppState{};

    let app = nidrs::NestFactory::create(app::AppModule, app_state).listen::<AppError>(3000);
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