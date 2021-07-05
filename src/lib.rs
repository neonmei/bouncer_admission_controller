pub mod config;
pub mod error;
pub mod http;
pub mod telemetry;
mod utils;
pub mod validators;

use actix_web::{dev::Server, App, HttpServer};
use tracing_actix_web::TracingLogger;

use error::StartupError;

pub fn run() -> Result<Server, StartupError> {
    telemetry::init_default();
    let bouncer_config = config::BouncerConfig::load().unwrap();

    let server = HttpServer::new(|| {
        App::new()
            .wrap(TracingLogger::default())
            .service(http::post_probes_validation)
            .service(http::get_healthcheck)
    })
    .bind_rustls(bouncer_config.listen_address, bouncer_config.tls.load()?)
    .map_err(StartupError::Bind)?
    .run();

    Ok(server)
}
