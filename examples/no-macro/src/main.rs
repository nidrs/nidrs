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
        if let Some(user_service) = container.resolve::<UserService>() {
            self.user_service.inject(user_service);
        } else {
            panic!("UserService not found in container");
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// 应用模块
struct AppModule {
    user_service: Arc<UserService>,
    app_service: Arc<AppService>,
}

impl AppModule {
    fn new() -> Self {
        Self {
            user_service: Arc::new(UserService::new("John")),
            app_service: Arc::new(AppService::default()),
        }
    }
}

impl Module for AppModule {
    fn configure(&self, container: &mut Container) {
        // 首先注册UserService，因为AppService依赖它
        container.register::<UserService>(Arc::clone(&self.user_service));
        
        // 然后注册AppService
        container.register::<AppService>(Arc::clone(&self.app_service));
        
        // 注入依赖
        for provider in &self.providers() {
            provider.inject(container);
        }
    }

    fn providers(&self) -> Vec<Arc<dyn Provider>> {
        vec![
            as_provider(&self.user_service),
            as_provider(&self.app_service),
        ]
    }
}

fn main() {
    // 创建应用
    let mut builder = AppBuilder::new();
    let app_module = AppModule::new();
    
    // 注册模块
    builder.register_module(&app_module);
    
    // 构建应用
    let app = builder.build();
    
    // 使用服务
    let app_service = app.get_service::<AppService>();
    println!("{}", app_service.greet());
}
