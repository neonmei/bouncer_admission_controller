use rustls::internal::pemfile::{certs, rsa_private_keys};
use rustls::{NoClientAuth, ServerConfig};
use std::fs::File;
use std::io::BufReader;

use std::net::SocketAddr;
use std::path::{Path, PathBuf};

use crate::error::StartupError;

fn base_dir() -> PathBuf {
    match std::env::var("BOUNCER_CONFIG_DIR") {
        Ok(dir) => dir.into(),
        Err(_) => std::env::current_dir()
            .expect("Could not get current directory")
            .join("configuration"),
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct TlsConfig {
    pub cert_path: PathBuf,
    pub key_path: PathBuf,
}

#[derive(serde::Deserialize, Debug)]
pub struct BouncerConfig {
    pub tls: TlsConfig,
    pub listen_address: SocketAddr,
}

impl TlsConfig {
    // TLS backend should be abstracted.
    pub fn load(self) -> Result<ServerConfig, StartupError> {
        let mut rustls_config = ServerConfig::new(NoClientAuth::new());

        let crt_file = File::open(&self.cert_path).map_err(StartupError::CertificateFile)?;
        let key_file = File::open(&self.key_path).map_err(StartupError::CertificateKeyFile)?;

        let crt_reader = &mut BufReader::new(crt_file);
        let key_reader = &mut BufReader::new(key_file);

        let certs = certs(crt_reader).map_err(|_| StartupError::CertificateParsing)?;
        let mut keys = rsa_private_keys(key_reader).map_err(|_| StartupError::KeyParsing)?;

        rustls_config
            .set_single_cert(certs, keys.remove(0))
            .map_err(StartupError::GenericTLS)?;

        Ok(rustls_config)
    }
}

impl BouncerConfig {
    pub fn load() -> Result<Self, config::ConfigError> {
        BouncerConfig::load_from(&base_dir().join("bouncer"))
    }

    pub fn load_from(config_path: &Path) -> Result<BouncerConfig, config::ConfigError> {
        let mut config = config::Config::default();

        // Move this once finished
        config.set_default("listen_address", "0.0.0.0:8080")?;
        config.set_default("tls.cert_path", "./certs/tls.crt")?;
        config.set_default("tls.key_path", "./certs/tls.key")?;

        config.merge(config::File::from(config_path).required(false))?;

        config.try_into()
    }
}
