use app::AppModule;
use single_hello::Creator;

mod app;


fn main() {
    let app = single_hello::NidrsFactory::<AppModule>::create();
    app.listen(8080);
}

