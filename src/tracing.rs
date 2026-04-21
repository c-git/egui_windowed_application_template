use anyhow::{Context as _, bail};
use std::{fs::create_dir_all, path::PathBuf};
use tracing::{Subscriber, subscriber::set_global_default};
use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, Registry, fmt::MakeWriter, layer::SubscriberExt as _};

#[cfg(not(target_arch = "wasm32"))]
/// Returns a guard for the subscriber if successful
///
/// # Errors
/// May fail for various reasons like invalid path to save to or not able to setup the writer
pub fn init_native() -> anyhow::Result<tracing_appender::non_blocking::WorkerGuard> {
    let (writer, path, guard) = setup_tracing_writer("egui-template-pwa")?;
    let subscriber = get_subscriber("egui-template-pwa".into(), "zbus=warn,info", writer);

    #[expect(clippy::print_stdout)]
    match init_subscriber(subscriber) {
        Ok(()) => {
            println!(
                "Traces being written to: {}",
                path.canonicalize()
                    .context("trace file canonicalization failed")?
                    .display()
            );
            Ok(guard)
        }
        Err(e) => {
            bail!("Failed to start tracing to file. Error: {e}");
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub fn init_wasm() {
    // print pretty errors in wasm https://github.com/rustwasm/console_error_panic_hook
    // This is not needed for tracing_wasm to work, but it is a common tool for
    // getting proper error line numbers for panics.
    console_error_panic_hook::set_once();

    let config = tracing_wasm::WASMLayerConfigBuilder::new()
        .set_max_level(tracing::Level::DEBUG)
        .build();
    tracing_wasm::set_as_global_default_with_config(config);
}

/// Compose multiple layers into a `tracing`'s subscriber.
///
/// For details acceptable Filter Directives see <https://docs.rs/tracing-subscriber/0.3.19/tracing_subscriber/filter/struct.EnvFilter.html#directives>
///
/// # Implementation Notes
///
/// We are using `impl Subscriber` as return type to avoid having to spell out
/// the actual type of the returned subscriber, which is indeed quite complex.
fn get_subscriber<Sink, S>(
    name: String,
    default_env_filter_directive: S,
    sink: Sink,
) -> impl Subscriber + Sync + Send
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
    S: AsRef<str>,
{
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(default_env_filter_directive));
    let formatting_layer = BunyanFormattingLayer::new(name, sink);
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

/// Register a subscriber as global default to process span data.
///
/// It should only be called once!
fn init_subscriber(subscriber: impl Subscriber + Sync + Send) -> anyhow::Result<()> {
    LogTracer::init().context("Failed to set logger")?;
    set_global_default(subscriber).context("Failed to set subscriber")?;
    Ok(())
}

/// Returns a handle to the file created and the file path
fn setup_tracing_writer(app_name: &str) -> anyhow::Result<(NonBlocking, PathBuf, WorkerGuard)> {
    // Create logging folder
    let log_folder = PathBuf::from("traces").join(app_name);
    create_dir_all(&log_folder).context("Failed to create logging folder")?;

    // Start non blocking logger wrapping a rolling logger
    let file_appender = tracing_appender::rolling::hourly(&log_folder, app_name);
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    Ok((non_blocking, log_folder, guard))
}
