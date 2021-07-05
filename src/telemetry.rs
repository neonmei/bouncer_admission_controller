use tracing::{subscriber::set_global_default, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

use crate::utils::get_program_name;

/// One-time registration of a global default subscriber to process span data.
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
}

/// Instantiates a basic subscriber.
pub fn build_default(name: String, env_filter: String) -> impl Subscriber + Send + Sync {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

    let formatting_layer = BunyanFormattingLayer::new(name, std::io::stdout);

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

/// Asume defaults.
pub fn init_default() {
    let program_name = get_program_name().unwrap();
    let env_filter = match cfg!(debug_assertions) {
        true => "debug",
        false => "info",
    };

    init_subscriber(build_default(program_name, env_filter.into()))
}
