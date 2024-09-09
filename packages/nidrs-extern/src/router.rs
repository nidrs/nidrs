use crate::{datasets, meta::Meta};

#[derive(Debug, Clone)]
pub struct StateCtx {}

#[derive(Debug, Clone)]
pub struct MetaRouter {
    pub router: axum::Router<StateCtx>,
    pub meta: Meta,
}

impl MetaRouter {
    pub fn new(router: axum::Router<StateCtx>, meta: Meta) -> Self {
        MetaRouter { router, meta }
    }
}

impl MetaRouter {
    pub fn match_full_path(&self, matcher: &str) -> bool {
        let glob = globset::Glob::new(matcher);
        match glob {
            Ok(glob) => {
                let path: &str = self.meta.get_data::<datasets::RouterFullPath>().unwrap().value();
                glob.compile_matcher().is_match(path)
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                false
            }
        }
    }
}
