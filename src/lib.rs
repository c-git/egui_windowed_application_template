#![warn(unused_crate_dependencies)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]
#![cfg_attr(doc, doc = include_str!("../README.md"))]
#![warn(clippy::all, rust_2018_idioms)]

#[cfg(target_arch = "wasm32")]
mod used_in_main {
    use wasm_bindgen_futures as _;
    use web_sys as _;
}

mod app;
pub use app::TemplateApp;
