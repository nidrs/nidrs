use nidrs_extern::{
    axum::{self, response::IntoResponse as _},
    datasets::{self, RouterBodyScheme},
    meta::{ImplMeta, Meta},
    tokio::signal,
    tower::Service as _,
    utoipa::{
        self,
        openapi::{
            path::{OperationBuilder, PathItemBuilder},
            request_body::RequestBodyBuilder,
            Components, ContentBuilder, Info, OpenApiBuilder, PathsBuilder,
        },
    },
    utoipa_rapidoc::RapiDoc,
    utoipa_redoc::{Redoc, Servable},
    utoipa_swagger_ui::SwaggerUi,
};
use nidrs_extern::{datasets::RouterPath, tokio};
use std::{
    any::Any,
    collections::HashMap,
    sync::{Arc, RwLock},
    time::Duration,
};

use crate::{provider, shared::otr, template_format, AppResult, InnerMeta, Interceptor, Service};

static GLOBALS_KEY: &str = "Defaults";

pub trait Module {
    fn init(self, ctx: ModuleCtx) -> ModuleCtx;

    fn destroy(&self, ctx: &ModuleCtx);
}

pub struct DynamicModule {
    pub services: HashMap<String, Box<dyn Any>>,
    pub exports: Vec<String>,
}

impl Default for DynamicModule {
    fn default() -> Self {
        Self::new()
    }
}

impl DynamicModule {
    pub fn new() -> Self {
        DynamicModule { services: HashMap::new(), exports: Vec::new() }
    }

    pub fn provider(mut self, service: (String, Box<dyn Any>)) -> Self {
        self.services.insert(service.0, service.1);
        self
    }

    pub fn service<T: Service + 'static>(mut self, service: T) -> Self {
        let (name, service) = provider(service);
        self.services.insert(name, service);
        self
    }

    pub fn export<T: Service + 'static>(mut self, service: T) -> Self {
        let (name, service) = provider(service);
        self.services.insert(name.clone(), service);
        self.exports.push(name);
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
    pub module: Option<T>,
    pub module_ctx: ModuleCtx,
    pub router: axum::Router<StateCtx>,
    pub port: u32,
    pub rt: RwLock<Option<tokio::runtime::Runtime>>,
    pub inter_apply: Vec<Box<dyn FnOnce(axum::Router<StateCtx>) -> axum::Router<StateCtx> + 'static>>,

    pub router_hook: Box<dyn Fn(RouterWrap) -> axum::Router<StateCtx>>,
}

impl<T: Module> NidrsFactory<T> {
    pub fn create(module: T) -> Self {
        let router: axum::Router<StateCtx> = axum::Router::new().route("/", axum::routing::get(|| async move { "Hello, Nidrs!" }));
        let module_ctx = ModuleCtx::new(ModuleDefaults { default_version: "v1", default_prefix: "" });
        NidrsFactory {
            rt: RwLock::new(None),
            router,
            module: Some(module),
            module_ctx,
            port: 3000,
            router_hook: Box::new(|r| r.router),
            inter_apply: vec![],
        }
    }

    pub fn default_prefix(mut self, prefix: &'static str) -> Self {
        self.module_ctx.defaults.default_prefix = prefix;
        self
    }

    pub fn default_version(mut self, v: &'static str) -> Self {
        self.module_ctx.defaults.default_version = v;
        self
    }

    pub fn default_uses<I: Interceptor + 'static + Sync + Send>(mut self, inter: I) -> Self {
        let service_name = I::__meta().get_data::<datasets::ServiceName>().unwrap().value().clone();
        let interceptor = Arc::new(inter);
        self.module_ctx.register_interceptor(GLOBALS_KEY, &service_name, Box::new(interceptor.clone()));

        self.inter_apply.push(Box::new(move |router| {
            router.layer(axum::middleware::from_fn({
                move |req: axum::extract::Request, next: axum::middleware::Next| {
                    let inter = std::sync::Arc::clone(&interceptor);
                    async move {
                        let res = inter.intercept(req, next).await;
                        if let Ok(res) = res {
                            Ok(res.into_response())
                        } else {
                            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
                        }
                    }
                }
            }))
        }));
        self
    }

    pub fn default_router_hook(mut self, hook: impl Fn(RouterWrap) -> axum::Router<StateCtx> + 'static) -> Self {
        self.router_hook = Box::new(hook);
        self
    }

    pub fn listen(mut self, port: u32) -> Self {
        self.port = port;
        let module = self.module.take().unwrap();

        self.module_ctx = module.init(self.module_ctx);
        // println!("ModuleCtx Imports: {:?}", &module_ctx.imports);
        // println!("ModuleCtx Exports: {:?}", &module_ctx.exports);
        // println!("ModuleCtx Deps: {:?}", &module_ctx.deps);
        // println!("ModuleCtx Services: {:?}", &module_ctx.services.keys());
        // println!("ModuleCtx Globals: {:?}", &module_ctx.globals);

        // for item in self.module_ctx.interceptors.iter() {
        //     let service_name = item.0;
        //     let service = item.1;
        //     if service_name.starts_with(GLOBALS_KEY) {
        //         let service = self.module_ctx.get_interceptor(current_module_name, service_name)
        //     }
        // }

        let mut sub_router = axum::Router::new();
        for router in self.module_ctx.routers.iter() {
            sub_router = sub_router.merge((self.router_hook)(router.clone()));
        }

        let mut paths = PathsBuilder::new().build();

        for router in self.module_ctx.routers.iter() {
            let path = router.meta.get_data::<datasets::RouterFullPath>().unwrap().value();
            let method = router.meta.get_data::<datasets::RouterMethod>().unwrap().value();
            let path_type = match method.as_str() {
                "post" => utoipa::openapi::PathItemType::Post,
                "put" => utoipa::openapi::PathItemType::Put,
                "delete" => utoipa::openapi::PathItemType::Delete,
                "patch" => utoipa::openapi::PathItemType::Patch,
                "options" => utoipa::openapi::PathItemType::Options,
                "head" => utoipa::openapi::PathItemType::Head,
                "trace" => utoipa::openapi::PathItemType::Trace,
                "connect" => utoipa::openapi::PathItemType::Connect,
                _ => utoipa::openapi::PathItemType::Get,
            };
            let content = if let Some(router_scheme) = router.meta.get_data::<RouterBodyScheme>() {
                ContentBuilder::new().schema(router_scheme.value().1.clone()).build()
            } else {
                ContentBuilder::new().build()
            };
            let path_item = PathItemBuilder::new()
                .operation(
                    path_type,
                    OperationBuilder::new()
                        .description(path.into())
                        .request_body(Some(RequestBodyBuilder::new().content("application/json", content).build()))
                        .build(),
                )
                .build();
            paths.paths.insert(path.into(), path_item);
        }

        let api = OpenApiBuilder::new()
            .info(Info::new("Nidrs API", self.module_ctx.defaults.default_version))
            .paths(paths)
            .components(Some(Components::new()))
            .build();

        self.router = self
            .router
            .merge(sub_router)
            .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone()))
            .merge(Redoc::with_url("/redoc", api.clone()))
            .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"));

        while let Some(apply) = self.inter_apply.pop() {
            self.router = apply(self.router);
        }
        nidrs_macro::log!("Swagger UI on {}", format!("http://0.0.0.0:{}/swagger-ui", self.port));
        nidrs_macro::log!("Rapidoc UI on {}", format!("http://0.0.0.0:{}/rapidoc", self.port));
        nidrs_macro::log!("Redoc UI on {}", format!("http://0.0.0.0:{}/redoc", self.port));

        self
    }

    pub fn block(mut self) {
        // listen...
        let server = || async {
            let tcp = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", self.port)).await?;
            let addr = tcp.local_addr()?;
            nidrs_macro::log!("Listening on {}", addr);

            axum::serve(tcp, self.router.with_state(StateCtx {})).await?;

            AppResult::Ok(())
        };

        self.rt = RwLock::new(Some(
            tokio::runtime::Builder::new_multi_thread()
                // .worker_threads(4) // 设置工作线程数量
                .enable_all() // 启用所有运行时功能
                .build()
                .unwrap(),
        ));

        if let Some(rt) = &*self.rt.write().unwrap() {
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
        }

        self.module_ctx.destroy();
        nidrs_macro::log!("Process is exiting now.");
        let rt = self.rt.write().unwrap().take();
        if let Some(rt) = rt {
            rt.shutdown_timeout(Duration::from_secs(1));
        }
    }

    pub fn destroy(&self) {
        self.module_ctx.destroy();
        nidrs_macro::log!("Process is exiting now.");
        let rt = self.rt.write().unwrap().take();
        if let Some(rt) = rt {
            rt.shutdown_timeout(Duration::from_secs(1));
        }
    }
}

#[derive(Debug, Clone)]
pub struct StateCtx {}

pub struct ModuleCtx {
    pub defaults: ModuleDefaults,
    pub modules: HashMap<String, Box<dyn Module>>,
    pub services: HashMap<String, Box<dyn Any>>,
    pub controllers: HashMap<String, Box<dyn Any>>,
    pub routers: Vec<RouterWrap>,
    pub interceptors: HashMap<String, Box<dyn Any>>,

    pub imports: HashMap<String, Vec<String>>,
    pub exports: HashMap<String, Vec<String>>,
    pub deps: HashMap<String, Vec<String>>,
    pub globals: HashMap<String, String>,
}

impl ModuleCtx {
    pub fn new(defaults: ModuleDefaults) -> Self {
        ModuleCtx {
            defaults,
            modules: HashMap::new(),      // {"UserModule": Box<UserModule>}
            services: HashMap::new(),     // {"UserModule::UserService": Arc<UserService>}
            controllers: HashMap::new(),  // {"UserModule::UserController": Arc<UserController>}
            routers: Vec::new(),          // vec![axum::Router::new().route("/", axum::routing::get(|| async move { "Hello, Nidrs!" }))],
            interceptors: HashMap::new(), // {"UserModule::UserService": Arc<UserService>}
            imports: HashMap::new(),      // {"UserModule": ["AppModule"]}
            exports: HashMap::new(),      // {"UserModule": ["UserService"]}
            deps: HashMap::new(),         // {"UserService": ["UserModule"]}
            globals: HashMap::new(),      // {"UserService": "UserModule::UserService"}
        }
    }

    pub fn destroy(&self) {
        for (_, module) in self.modules.iter() {
            module.destroy(self);
        }
    }

    pub fn get_controller<R: 'static>(&self, current_module_name: &str, service_name: &str) -> Arc<R> {
        let svc_mods = self.deps.get(service_name).unwrap_or_else(|| panic!("[nidrs] not deps {} {}", current_module_name, service_name)); // ["UserModule"];
        let imp_mods =
            self.imports.get(current_module_name).unwrap_or_else(|| panic!("[nidrs] not import {}::{}", current_module_name, service_name)); // ["UserModule"];
        let intersection_mods = svc_mods.iter().filter(|&m| imp_mods.contains(m)).cloned().collect::<Vec<_>>();
        let first_mod = intersection_mods.first().unwrap_or(&current_module_name.to_string()).clone();
        let svc_key = format!("{}::{}", first_mod, service_name);

        let svc = self.controllers.get(&svc_key).unwrap();
        let svc = svc.downcast_ref::<std::sync::Arc<R>>().unwrap();

        svc.clone()
    }

    pub fn register_controller(&mut self, current_module_name: &str, service_name: &str, controller: Box<dyn Any>) -> bool {
        let svc_key = current_module_name.to_string() + "::" + service_name;
        if !self.controllers.contains_key(svc_key.as_str()) {
            self.controllers.insert(svc_key.clone(), controller);
            self.deps.entry(service_name.to_string()).or_default().push(current_module_name.to_string());
            // self.exports.entry(current_module_name.to_string()).or_default().push(service_name.to_string());
            nidrs_macro::log!("Registering controller {}.", svc_key);
            return true;
        }
        false
    }

    pub fn get_interceptor<R: 'static>(&self, current_module_name: &str, service_name: &str) -> Arc<R> {
        let current_module_name = GLOBALS_KEY;
        // let svc_mods = self.deps.get(service_name).expect(format!("[nidrs] not deps {} {}", current_module_name, service_name).as_str()); // ["UserModule"];
        // println!("svc_mods: {:?}", (&self.imports, &self.exports, &svc_mods));
        // let imp_mods = self.imports.get(current_module_name).expect(format!("[nidrs] not import {}::{}", current_module_name, service_name).as_str()); // ["UserModule"];
        // let intersection_mods = svc_mods
        //     .iter()
        //     .filter(|&m| imp_mods.contains(m))
        //     .map(|m| m.clone())
        //     .collect::<Vec<_>>();
        // let first_mod = intersection_mods.get(0).unwrap_or(&current_module_name.to_string()).clone();
        // let svc_key = format!("{}::{}", first_mod, service_name);
        let svc_key = format!("{}::{}", current_module_name, service_name);

        let svc =
            self.interceptors.get(&svc_key).unwrap_or_else(|| panic!("[nidrs] not inject {}::{} {}", current_module_name, service_name, svc_key));
        let svc = svc.downcast_ref::<std::sync::Arc<R>>().unwrap();

        svc.clone()
    }

    pub fn register_interceptor(&mut self, current_module_name: &str, service_name: &str, interceptor: Box<dyn Any>) -> bool {
        let current_module_name = GLOBALS_KEY;
        let svc_key = current_module_name.to_string() + "::" + service_name;
        if !self.interceptors.contains_key(svc_key.as_str()) {
            self.interceptors.insert(svc_key.clone(), interceptor);
            self.deps.entry(service_name.to_string()).or_default().push(current_module_name.to_string());
            // self.exports.entry(current_module_name.to_string()).or_default().push(service_name.to_string());
            nidrs_macro::log!("Registering interceptor {}.", svc_key);
            return true;
        }
        false
    }

    pub fn get_service<R: 'static>(&self, current_module_name: &str, service_name: &str) -> Arc<R> {
        let svc_mods = self.deps.get(service_name).unwrap_or_else(|| panic!("[nidrs] not deps {} {}", current_module_name, service_name)); // ["UserModule"];
        let imp_mods =
            self.imports.get(current_module_name).unwrap_or_else(|| panic!("[nidrs] not import {}::{}", current_module_name, service_name)); // ["UserModule"];
        let intersection_mods = svc_mods.iter().filter(|&m| imp_mods.contains(m)).cloned().collect::<Vec<_>>();
        let first_mod = intersection_mods.first().unwrap_or(&current_module_name.to_string()).clone();
        if first_mod != current_module_name && !self.exports.get(&first_mod).unwrap().contains(&service_name.to_string()) {
            nidrs_macro::elog!("[{}] {} is not exported by {}", current_module_name, service_name, first_mod);
            // panic!("exit");
        }

        let svc_key = format!("{}::{}", first_mod, service_name);

        let svc_key = if self.services.contains_key(&svc_key) {
            svc_key
        } else {
            let mod_name = self
                .globals
                .get(service_name)
                .unwrap_or_else(|| panic!("[nidrs] {}::{} inject {} error", current_module_name, service_name, svc_key))
                .to_string();
            format!("{}::{}", mod_name, service_name)
        };

        let svc = self.services.get(&svc_key).unwrap_or_else(|| panic!("[nidrs] {}::{} inject {} error", current_module_name, service_name, svc_key));
        let svc =
            svc.downcast_ref::<std::sync::Arc<R>>().unwrap_or_else(|| panic!("[nidrs] not downcast_ref {} {}", current_module_name, service_name));

        svc.clone()
    }

    pub fn register_service<T: Into<String>>(&mut self, current_module_name: &str, service_name: T, service: Box<dyn Any>) -> bool {
        let service_name = service_name.into();
        let svc_key = current_module_name.to_string() + "::" + &service_name;
        if !self.services.contains_key(svc_key.as_str()) {
            self.services.insert(svc_key.clone(), service);
            self.deps.entry(service_name.to_string()).or_default().push(current_module_name.to_string());
            // self.exports.entry(current_module_name.to_string()).or_default().push(service_name.to_string());

            nidrs_macro::log!("Registering service {}.", svc_key);
            return true;
        } else {
            nidrs_macro::elog!("Service {} already exists.", svc_key);
        }
        false
    }

    pub fn append_exports<T: Into<String>>(&mut self, current_module_name: &str, service_names: Vec<T>, is_global: bool) -> bool {
        let mut success = true;
        for service_name in service_names {
            let service_name = service_name.into();
            let svc_key = current_module_name.to_string() + "::" + &service_name;
            if !self.exports.contains_key(current_module_name) {
                self.exports.insert(current_module_name.to_string(), vec![service_name.to_string()]);
            } else {
                let exports = self.exports.get_mut(current_module_name).unwrap();
                if !exports.contains(&service_name.to_string()) {
                    exports.push(service_name.to_string());
                } else {
                    nidrs_macro::elog!("Service {} already exported.", svc_key);
                    success = false;
                }
            }
            if is_global {
                self.globals.insert(service_name.to_string(), current_module_name.to_string());
            }
        }
        success
    }

    pub fn get_router_full(&self, meta: &InnerMeta) -> AppResult<String> {
        let controller_path = otr(meta.get_data::<datasets::ControllerPath>(), "meta not nidrs::datasets::ControllerPath value")?.value();
        let router_path = otr(meta.get_data::<datasets::RouterPath>(), "meta not nidrs::datasets::RouterPath value")?.value();
        let version = *meta.get::<&str>("version").unwrap_or(&self.defaults.default_version);
        let disable_default_prefix = meta.get_data::<datasets::DisableDefaultPrefix>().unwrap_or(&datasets::DisableDefaultPrefix(false)).value();
        let full_path = if disable_default_prefix {
            format!("{}{}", controller_path, router_path)
        } else {
            template_format(&format!("{}{}{}", self.defaults.default_prefix, controller_path, router_path), [("version", version)])
        };

        Ok(full_path)
    }
}

#[derive(Debug, Clone)]
pub struct RouterWrap {
    pub router: axum::Router<StateCtx>,
    pub meta: Meta,
}

impl RouterWrap {
    pub fn new(router: axum::Router<StateCtx>, meta: Meta) -> Self {
        RouterWrap { router, meta }
    }
}

impl RouterWrap {
    pub fn match_router_path(&self, matcher: &str) -> bool {
        let glob = nidrs_extern::globset::Glob::new(matcher);
        match glob {
            Ok(glob) => {
                let path: &str = self.meta.get_data::<RouterPath>().unwrap().value();
                glob.compile_matcher().is_match(path)
            }
            Err(err) => {
                nidrs_macro::elog!("Error: {:?}", err);
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{sync::Arc, vec};

    #[test]
    fn test_nidrs_factory() {
        use std::any::Any;

        trait Controller: Any {
            fn handle_request(&self);
            // 定义一个方法，用于将 `&self` 转换为 `&dyn Any`
            fn as_any(&self) -> &dyn Any;
        }

        struct ConcreteService {
            pub name: String,
        };

        impl Controller for ConcreteService {
            fn handle_request(&self) {
                println!("Handling request...");
            }

            fn as_any(&self) -> &dyn Any {
                self
            }
        }

        fn main() {
            let service: Arc<dyn Controller> = Arc::new(ConcreteService { name: "hello".to_string() });

            service.handle_request();

            let service_ref: &dyn Controller = service.as_ref();
            let service_any: &dyn Any = service_ref.as_any();

            if let Some(concrete) = service_any.downcast_ref::<ConcreteService>() {
                concrete.handle_request();
            } else {
                println!("Not a ConcreteService instance.");
            }

            let mut t = vec!["str", "st2"];

            t.drain(..).for_each(|x| println!("{}", x));
        }
        main();
    }
}
