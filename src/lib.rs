#![warn(unused_crate_dependencies)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]
#![cfg_attr(doc, doc = include_str!("../README.md"))]
#![warn(clippy::all, rust_2018_idioms)]

#[cfg(target_arch = "wasm32")]
mod suppress_wasm_warnings {
    // Only used in binary and triggers unused warning
    use anyhow as _; // TODO 2: Remove once we write more code and anyhow is used
    use wasm_bindgen_futures as _;
    use web_sys as _;
}

#[cfg(test)]
mod dev_dependencies {
    use wasm_bindgen_test as _;
}

mod app;
mod data;
mod pages;
mod shortcuts;
pub mod tracing;
pub use app::TemplateApp;
pub use data::DataShared;
pub use pages::{DisplayablePage, UiPage};
