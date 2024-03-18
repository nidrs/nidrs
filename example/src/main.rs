use nestrs::Module;


mod app;

fn main() {
    println!("Hello, world!");

    let app_state = AppState{};

    nestrs::NestFactory::create(app::AppModule, Box::new(app_state)).listen(3000);
}


pub struct AppState{}