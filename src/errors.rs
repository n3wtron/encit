use std::io;
use std::io::Error;
use std::string::FromUtf8Error;

use base64::DecodeError;
use config::ConfigError;
use hex::FromHexError;
use josekit::JoseError;
use openssl::error::ErrorStack;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EncItError {
    #[error("Configuration not found: {0}")]
    ConfigurationNotFound(String),
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    #[error("Identity not found: {0}")]
    IdentityNotFound(String),
    #[error("Friend not found: {0}")]
    FriendNotFound(String),
    #[error("IO Error: {0}")]
    IoError(String),
    #[error("Decode Error: {0}")]
    DecodeError(String),
    #[error("Encode Error: {0}")]
    EncodeError(String),
    #[error("Key Error: {0}")]
    SSLError(String),
    #[error("JWT Error: {0}")]
    JWTError(String),
    #[error("No message in payload claims")]
    EmptyMessage(),
    #[error("There is already a friend with that name")]
    FriendAlreadyExist(),
    #[error("There is already an identity with that name")]
    IdentityAlreadyExist(),
    #[error("Invalid command: {0}")]
    InvalidCommand(String),
}

impl From<ConfigError> for EncItError {
    fn from(cfg_error: ConfigError) -> Self {
        EncItError::ConfigurationError(cfg_error.to_string())
    }
}

impl From<io::Error> for EncItError {
    fn from(io_err: Error) -> Self {
        EncItError::IoError(io_err.to_string())
    }
}

impl From<base64::DecodeError> for EncItError {
    fn from(dec_error: DecodeError) -> Self {
        EncItError::DecodeError(dec_error.to_string())
    }
}

impl From<ErrorStack> for EncItError {
    fn from(error_stack: ErrorStack) -> Self {
        EncItError::SSLError(error_stack.to_string())
    }
}

impl From<FromUtf8Error> for EncItError {
    fn from(utf8_error: FromUtf8Error) -> Self {
        EncItError::EncodeError(utf8_error.to_string())
    }
}

impl From<JoseError> for EncItError {
    fn from(jwt_error: JoseError) -> Self {
        EncItError::JWTError(format!("{}", jwt_error))
    }
}

impl From<FromHexError> for EncItError {
    fn from(e: FromHexError) -> Self {
        EncItError::EncodeError(e.to_string())
    }
}

impl From<serde_yaml::Error> for EncItError {
    fn from(err: serde_yaml::Error) -> Self {
        EncItError::ConfigurationError(err.to_string())
    }
}
