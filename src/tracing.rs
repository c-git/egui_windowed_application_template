use anyhow::Context as _;
use tracing_subscriber::layer::SubscriberExt as _;

#[cfg(not(target_arch = "wasm32"))]
/// Returns a guard for the subscriber if successful
///
/// # Errors
/// May fail for various reasons like invalid path to save to or not able to
/// setup the writer
pub fn init_native() -> anyhow::Result<tracing_appender::non_blocking::WorkerGuard> {
    let (writer, path, guard) = setup_tracing_writer("egui-template-pwa")?;
    let subscriber = get_subscriber("egui-template-pwa".into(), "zbus=warn,info", writer);

    init_subscriber_with_path(&path, guard, subscriber)
}

/// Calls `init_subscriber` and if it succeeds it prints the path given and
/// returns the guard. This function is needed because we first need to know if
/// the init succeeded to know if we should print the path or not but
/// `init_subscriber` shouldn't be the one to do it because it has no need for
/// the path
#[cfg(not(target_arch = "wasm32"))]
fn init_subscriber_with_path(
    path: &std::path::Path,
    guard: tracing_appender::non_blocking::WorkerGuard,
    subscriber: impl tracing::Subscriber + Sync + Send,
) -> Result<tracing_appender::non_blocking::WorkerGuard, anyhow::Error> {
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
            anyhow::bail!("Failed to start tracing to file. Error: {e}");
        }
    }
}

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

#[cfg(not(target_arch = "wasm32"))]
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
) -> impl tracing::Subscriber + Sync + Send
where
    Sink: for<'a> tracing_subscriber::fmt::MakeWriter<'a> + Send + Sync + 'static,
    S: AsRef<str>,
{
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(default_env_filter_directive));
    let formatting_layer = tracing_bunyan_formatter::BunyanFormattingLayer::new(name, sink);
    tracing_subscriber::Registry::default()
        .with(env_filter)
        .with(tracing_bunyan_formatter::JsonStorageLayer)
        .with(formatting_layer)
}

#[cfg(not(target_arch = "wasm32"))]
/// Register a subscriber as global default to process span data.
///
/// It should only be called once!
fn init_subscriber(subscriber: impl tracing::Subscriber + Sync + Send) -> anyhow::Result<()> {
    use anyhow::Context as _;

    tracing_log::LogTracer::init().context("Failed to set logger")?;
    tracing::subscriber::set_global_default(subscriber).context("Failed to set subscriber")?;
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
/// Returns a handle to the file created and the file path
fn setup_tracing_writer(
    app_name: &str,
) -> anyhow::Result<(
    tracing_appender::non_blocking::NonBlocking,
    std::path::PathBuf,
    tracing_appender::non_blocking::WorkerGuard,
)> {
    // Create logging folder
    let log_folder = std::path::PathBuf::from("traces").join(app_name);
    std::fs::create_dir_all(&log_folder).context("Failed to create logging folder")?;

    // Start non blocking logger wrapping a rolling logger
    let file_appender = tracing_appender::rolling::hourly(&log_folder, app_name);
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    Ok((non_blocking, log_folder, guard))
}
