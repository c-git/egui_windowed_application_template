#![warn(unused_crate_dependencies)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]
#![cfg_attr(doc, doc = include_str!("../README.md"))]
#![warn(clippy::all, rust_2018_idioms)]

#[cfg(target_arch = "wasm32")]
mod suppress_wasm_warnings {
    // Only used in binary and triggers unused warning
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
pub use pages::UiPage;

/// Placeholder type for if your application needs permissions. I normally model
/// them as an enum and then have vectors or arrays to store lists of them
/// needed for various parts of the application. That said they more serve to
/// provide a good UI by not showing buttons that won't work on the client but
/// all actual validation of privileges is done on the server in all of my
/// applications. For an example where I actually used permissions you can look
/// at a chat app that I did at
/// <https://github.com/wykies/crates/tree/develop/crates/chat-app-client>
struct Permission;
