use nidrs_extern::tokio;
use nidrs_extern::{axum, tokio::signal};
use std::{any::Any, collections::HashMap, sync::Arc};

use crate::{provider, AppResult, ControllerService, ImplMeta, InterceptorService, Service};

pub trait Module {
    fn init(self, ctx: ModuleCtx) -> ModuleCtx;

    fn destroy(&self, ctx: &ModuleCtx);
}

pub struct DynamicModule {
    pub services: HashMap<&'static str, Arc<dyn Service>>,
}

impl Default for DynamicModule {
    fn default() -> Self {
        Self::new()
    }
}

impl DynamicModule {
    pub fn new() -> Self {
        DynamicModule { services: HashMap::new() }
    }

    pub fn service(mut self, service: (&'static str, Arc<dyn Service>)) -> Self {
        self.services.insert(service.0, service.1);
        self
    }

    pub fn provider<T: Service + ImplMeta + 'static>(mut self, service: T) -> Self {
        let (name, service) = provider(service);
        self.services.insert(name, service);
        self
    }
}

impl Module for DynamicModule {
    fn init(self, ctx: ModuleCtx) -> ModuleCtx {
        ctx
    }

    fn destroy(&self, ctx: &ModuleCtx) {}
}

#[derive(Debug, Clone)]
pub struct ModuleDefaults {
    pub default_version: &'static str,
    pub default_prefix: &'static str,
}

pub struct NidrsFactory<T: Module> {
    pub defaults: ModuleDefaults,
    pub module: T,
}

impl<T: Module> NidrsFactory<T> {
    pub fn create(module: T) -> Self {
        NidrsFactory { module, defaults: ModuleDefaults { default_version: "v1", default_prefix: "" } }
    }

    pub fn default_prefix(mut self, prefix: &'static str) -> Self {
        self.defaults.default_prefix = prefix;
        self
    }

    pub fn default_version(mut self, v: &'static str) -> Self {
        self.defaults.default_version = v;
        self
    }

    pub fn listen(self, port: u32) {
        let router = axum::Router::new().route("/", axum::routing::get(|| async move { "Hello, Nidrs!" }));
        let module_ctx = ModuleCtx::new(self.defaults);
        let module_ctx = self.module.init(module_ctx);
        let routers = module_ctx.routers.clone();
        let mut sub_router = axum::Router::new();
        for router in routers.iter() {
            sub_router = sub_router.merge(router.clone());
        }
        let router = router.merge(sub_router);

        // listen...
        let server = || async {
            let tcp = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
            let addr = tcp.local_addr()?;
            nidrs_macro::log!("Listening on {}", addr);

            axum::serve(tcp, router.with_state(StateCtx {})).await?;

            AppResult::Ok(())
        };

        let rt = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(4) // 设置工作线程数量
            .enable_all() // 启用所有运行时功能
            .build()
            .unwrap();

        rt.block_on(async {
            // 使用 tokio::select 宏同时监听服务器和退出信号
            tokio::select! {
                _ = server() => {
                  nidrs_macro::elog!("Server exited unexpectedly.");
                },
                _ = signal::ctrl_c() => {
                  nidrs_macro::log!("Received Ctrl+C, shutting down...");
                }
            }
        });
        module_ctx.destroy();
        nidrs_macro::log!("Process is exiting now.");
    }

    fn destroy(&self) {}
}

#[derive(Debug, Clone)]
pub struct StateCtx {}

pub struct ModuleCtx {
    pub defaults: ModuleDefaults,
    pub modules: HashMap<String, Box<dyn Module>>,
    pub services: HashMap<String, Box<dyn Service>>,
    pub controllers: HashMap<String, Box<dyn ControllerService>>,
    pub routers: Vec<axum::Router<StateCtx>>,
    pub interceptors: HashMap<String, Box<dyn InterceptorService>>,
}

impl ModuleCtx {
    pub fn new(defaults: ModuleDefaults) -> Self {
        ModuleCtx {
            defaults,
            modules: HashMap::new(),
            services: HashMap::new(),
            controllers: HashMap::new(),
            routers: Vec::new(),
            interceptors: HashMap::new(),
        }
    }

    pub fn destroy(&self) {
        for (_, module) in self.modules.iter() {
            module.destroy(self);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, ops::Deref, sync::{Arc, Mutex}, thread::spawn};

    #[test]
    fn test_nidrs_factory() {
        use std::any::Any;

        trait Service: Any + Send + Sync{
            fn inject(&self);
            // 定义一个方法，用于将 `&self` 转换为 `&dyn Any`
            fn as_any(&self) -> &dyn Any;
        }
        
        struct TestServiceInner{
            pub name: Mutex<String>
        }

        struct TestService{
            pub inner: Arc<TestServiceInner>
        }
        impl TestService {
            fn new() -> Self {
                TestService {
                    inner: Arc::new(TestServiceInner{ name: Mutex::new("test".to_string())})
                }
            }
        }
        impl TestService {
            fn handle_request(&self) {
                println!("Handling request... {}", self.name.lock().unwrap());
            }
            
        }
        
        impl Service for TestService {
            fn inject(&self) {
                println!("Injecting service...");
                self.name.lock().unwrap().push_str("inject");
            }
            
            fn as_any(&self) -> &dyn Any {
                self
            }
        }

        impl Clone for TestService {
            fn clone(&self) -> Self {
                TestService {
                    inner: self.inner.clone()
                }
            }
        }

        impl Deref for TestService {
            type Target = TestServiceInner;
            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        fn main() {
            // let mut map = HashMap::<&str, Box<dyn Service>>::new();
            let service: Box<dyn Service> = Box::new(TestService::new());
            let test_service = service.as_any().downcast_ref::<TestService>().unwrap().clone();
            service.inject();

            spawn(move || {
                test_service.handle_request();
            }).join().unwrap();

        }
        main();
    }
}