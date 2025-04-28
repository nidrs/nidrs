use std::{any::{Any, TypeId}, collections::HashMap, sync::{Arc, OnceLock}};

// 依赖注入容器
#[derive(Default)]
pub struct Container {
    services: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}

impl Container {
    pub fn new() -> Self {
        Self { services: HashMap::new() }
    }

    // 注册单例服务
    pub fn register<T: 'static + Send + Sync>(&mut self, instance: Arc<T>) {
        println!("Registering service: {}", std::any::type_name::<T>());
        self.services.insert(TypeId::of::<T>(), instance as Arc<dyn Any + Send + Sync>);
    }

    // 解析服务
    pub fn resolve<T: 'static + Send + Sync>(&self) -> Option<Arc<T>> {
        let type_id = TypeId::of::<T>();
        println!("Resolving service: {}", std::any::type_name::<T>());
        
        if let Some(service) = self.services.get(&type_id) {
            println!("  Service found");
            match service.clone().downcast::<T>() {
                Ok(concrete) => {
                    println!("  Downcast successful");
                    Some(concrete)
                },
                Err(_) => {
                    println!("  Downcast failed");
                    None
                }
            }
        } else {
            println!("  Service not found");
            None
        }
    }

    // 解析服务或报错
    pub fn resolve_or_panic<T: 'static + Send + Sync>(&self) -> Arc<T> {
        self.resolve::<T>().unwrap_or_else(|| panic!("Service of type {} not registered", std::any::type_name::<T>()))
    }
}

// 依赖注入注射器 - 用于惰性获取依赖
#[derive(Debug, Clone)]
pub struct Inject<T: 'static + Send + Sync> {
    value: Arc<OnceLock<Arc<T>>>,
}

impl<T: 'static + Send + Sync> Default for Inject<T> {
    fn default() -> Self {
        Self { value: Arc::new(OnceLock::new()) }
    }
}

impl<T: 'static + Send + Sync> Inject<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn inject(&self, value: Arc<T>) {
        println!("Injecting {} into Inject container", std::any::type_name::<T>());
        match self.value.set(value) {
            Ok(_) => println!("  Injection successful"),
            Err(_) => println!("  Injection failed (already set)"),
        }
    }

    pub fn extract(&self) -> Arc<T> {
        self.value.get().expect("Dependency not injected").clone()
    }
}

impl<T: 'static + Send + Sync> std::ops::Deref for Inject<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        // 从Arc<T>解引用到T
        match self.value.get() {
            Some(val) => &*val,
            None => panic!("Dependency of type {} not injected", std::any::type_name::<T>()),
        }
    }
}

// 服务提供者接口
pub trait Provider: Any + Send + Sync {
    fn inject(&self, container: &Container);
    fn as_any(&self) -> &dyn Any;
}

// 模块 - 用于组织服务
pub trait Module {
    fn configure(&self, container: &mut Container);
    fn providers(&self) -> Vec<Arc<dyn Provider>>;
}

// 应用构建器
pub struct AppBuilder {
    container: Container,
}

impl AppBuilder {
    pub fn new() -> Self {
        Self { container: Container::new() }
    }

    pub fn register_module<M: Module>(&mut self, module: &M) {
        module.configure(&mut self.container);
    }

    pub fn build(self) -> Application {
        Application { container: self.container }
    }
}

// 应用
pub struct Application {
    container: Container,
}

impl Application {
    pub fn get_service<T: 'static + Send + Sync>(&self) -> Arc<T> {
        self.container.resolve_or_panic::<T>()
    }
}

// 将具体Provider转换为trait对象的辅助函数
pub fn as_provider<T: Provider + 'static>(provider: &Arc<T>) -> Arc<dyn Provider> {
    Arc::clone(provider) as Arc<dyn Provider>
}
