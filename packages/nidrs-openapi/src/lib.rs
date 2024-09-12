use nidrs_extern::{
    router::{MetaRouter, StateCtx},
    shared::convert_path_to_openapi,
};
pub use utoipa;
use utoipa::openapi::{
    path::{OperationBuilder, PathItemBuilder},
    request_body::RequestBodyBuilder,
    Components, Info, OpenApiBuilder, PathsBuilder,
};
pub use utoipa_rapidoc;
use utoipa_rapidoc::RapiDoc;
pub use utoipa_redoc;
use utoipa_redoc::{Redoc, Servable};
pub use utoipa_scalar;
pub use utoipa_swagger_ui;
use utoipa_swagger_ui::SwaggerUi;

mod datasets;
pub use datasets::*;

pub use nidrs_openapi_macro::*;

pub fn register(routers: &Vec<MetaRouter>) -> axum::Router<StateCtx> {
    // OPENAPI IMPLEMENTATION
    let mut paths = PathsBuilder::new().build();
    let components = Components::new();

    for router in routers.iter() {
        let path = router.meta.get_data::<nidrs_extern::datasets::RouterFullPath>().unwrap().value();
        let method = router.meta.get_data::<nidrs_extern::datasets::RouterMethod>().unwrap().value();
        let router_name = router.meta.get_data::<nidrs_extern::datasets::RouterName>().unwrap().value();
        let controller_name = router.meta.get_data::<nidrs_extern::datasets::ServiceName>().unwrap().value();
        // println!("path: {}, method: {}, body: {:?}", path, method, router.meta.get_data::<datasets::RouterIn>());
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

        let opath = convert_path_to_openapi(path);
        if paths.paths.get(&opath).is_none() {
            let path_item = PathItemBuilder::new().build();
            paths.paths.insert(opath.clone(), path_item);
        }

        if let Some(path_item) = paths.paths.get_mut(&opath) {
            let mut operation = OperationBuilder::new();
            let router_in = router.meta.get_data::<datasets::RouterIn>();
            let router_out = router.meta.get_data::<datasets::RouterOut>();
            // println!("router_in: {:?}, router_out: {:?}", router_in, router_out);
            if let Some(router_in) = router_in {
                for param in router_in.value().value() {
                    match param {
                        datasets::ParamType::Param(p) => {
                            operation = operation.parameter(p.to_owned());
                        }
                        datasets::ParamType::Body(body) => {
                            if let Some(schema) = &body.schema {
                                // components.schemas.insert(schema.0.to_string(), schema.1.to_owned());
                                operation = operation.request_body(Some(
                                    RequestBodyBuilder::new()
                                        .content(body.content_type, utoipa::openapi::ContentBuilder::new().schema(schema.to_owned().1).build())
                                        .build(),
                                ));
                            }
                        }
                    }
                }
            }
            if let Some(router_out) = router_out {
                for param in router_out.value().value() {
                    if let datasets::ParamType::Body(body) = param {
                        let mut content = utoipa::openapi::ContentBuilder::new();
                        if let Some(schema) = &body.schema {
                            content = content.schema(schema.to_owned().1);
                        } else {
                            content = content.example(Some(serde_json::Value::String("String".to_string())));
                        }
                        let response = utoipa::openapi::ResponseBuilder::new().content(body.content_type, content.build()).build();
                        operation = operation.response("200", response);
                    }
                }
            }
            path_item.operations.insert(path_type.clone(), operation.build());
        }
    }

    let api = OpenApiBuilder::new().info(Info::new("Nidrs OpenAPI", "v1.0")).paths(paths).components(Some(components)).build();

    axum::Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone()))
        .merge(Redoc::with_url("/redoc", api.clone()))
        .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
}
