use rustls::TLSError;
use std::io::Error as IoError;

#[derive(thiserror::Error, Debug)]
/// Kubernetes object validation error.
pub enum ValidationError {
    #[error("Unsupported manifest Kind {0}")]
    UnsupportedKind(String),
    #[error("Received empty object")]
    EmptyObject,
    #[error("Manifest was expected to have containers declared")]
    NoContainers,
    #[error("Field has invalid type, was expecting {0}")]
    InvalidType(String),
}

#[derive(thiserror::Error, Debug)]
pub enum StartupError {
    #[error("Error reading tls certificate file: {0}")]
    CertificateFile(IoError),
    #[error("Error reading tls key file: {0}")]
    CertificateKeyFile(IoError),
    #[error("Error parsing certificate file")]
    CertificateParsing,
    #[error("Error parsing key file")]
    KeyParsing,
    #[error("Unknown TLS Error: {0}")]
    GenericTLS(TLSError),
    #[error("Error binding to listen address: {0}")]
    Bind(IoError),
    #[error("Unknown IO Error: {0}")]
    GenericIO(IoError),
}
