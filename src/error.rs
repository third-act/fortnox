use crate::api_error_code::ApiErrorCode;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Error {
    Unspecified(String),
    ParseError(String),
    SerializationError(String),
    NetworkError(String),
    ApiError(ApiErrorCode, String),
    Throttling,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = match self {
            Error::Unspecified(g) => g,
            Error::ParseError(g) => g,
            Error::SerializationError(g) => g,
            Error::NetworkError(g) => g,
            Error::ApiError(_, g) => g,
            Error::Throttling => "Throttling.",
        };
        write!(f, "{}", text)
    }
}
