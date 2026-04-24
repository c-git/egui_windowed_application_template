#[cfg(target_arch = "wasm32")]
use anyhow::Context as _;

#[cfg(not(target_arch = "wasm32"))]
mod native_only;

#[cfg(not(target_arch = "wasm32"))]
pub use native_only::init_native;

#[cfg(target_arch = "wasm32")]
/// Sets up the global tracing subscriber and sets better panic messages
///
/// # Errors
/// Fails if a global subscriber has already been set
pub fn init_wasm() -> anyhow::Result<()> {
    // print pretty errors in wasm https://github.com/rustwasm/console_error_panic_hook
    // This is not needed for tracing_wasm to work, but it is a common tool for
    // getting proper error line numbers for panics.

    console_error_panic_hook::set_once();

    let config = tracing_wasm::WASMLayerConfigBuilder::new()
        .set_max_level(tracing::Level::DEBUG)
        .build();
    tracing::subscriber::set_global_default(
        tracing_subscriber::Registry::default().with(tracing_wasm::WASMLayer::new(config)),
    )
    .context("failed to set global subscriber")
}
