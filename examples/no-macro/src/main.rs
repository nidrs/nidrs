use std::{any::Any, sync::Arc};
use no_macro::{as_provider, AppBuilder, Container, Inject, Module, Provider};


// 用户服务
#[derive(Debug, Default, Clone)]
struct UserService {
    name: String,
}

impl UserService {
    fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}

impl Provider for UserService {
    fn inject(&self, _container: &Container) {
        // 无依赖，不需要注入
        println!("UserService injected");
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// 应用服务 - 依赖于用户服务
#[derive(Debug, Default)]
struct AppService {
    user_service: Inject<UserService>,
}

impl AppService {
    fn greet(&self) -> String {
        format!("Hello, {}!", self.user_service.get_name())
    }
}

impl Provider for AppService {
    fn inject(&self, container: &Container) {
        println!("Injecting dependencies for AppService");
        self.user_service.inject(container);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// 应用模块
struct AppModule { }

impl Module for AppModule {
    fn configure(&self, container: &mut Container) {

        let mut module = container.create_module::<AppModule>();

        // 首先注册UserService，因为AppService依赖它
        module.register::<UserService>(Inject::default());
        
        // 然后注册AppService
        module.register::<AppService>(Inject::default());

        container.finalize(module);
    }

}

fn main() {
    // 创建应用
    let mut builder = AppBuilder::new();
    let app_module = AppModule{ };
    
    // 注册模块
    builder.register_module(&app_module);
    
    // 构建应用
    let app = builder.build();
    
    // 使用服务
    let app_service = app.get_service::<AppService>();
    println!("{}", app_service.greet());
}
