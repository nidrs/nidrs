mod app;
mod models;
mod modules;
mod shared;
// mod test;

pub use nidrs::AppError;
pub use nidrs::AppResult;

#[nidrs::main]
fn main() {
    let app = nidrs::NidrsFactory::create(app::AppModule);

    let app = app.default_prefix("/api/{version}");
    let app = app.default_version("v1");

    app.listen(3000).block();
}
