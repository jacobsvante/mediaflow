use std::fmt::Display;

use http::StatusCode;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to authenticate: {0} - {1}")]
    AuthenticationError(u16, String),
    #[error("failed HTTP request")]
    HttpError(#[from] reqwest::Error),
    #[error("JSON serialization/deserialization error")]
    JsonError(#[from] serde_json::Error),
    #[error("API error: {0}")]
    ApiError(StatusCode, String),
    #[error("{0}")]
    UnexpectedApiResponseError(String),
}

#[derive(serde::Deserialize, Debug)]
pub struct MediaFlowResponseError {
    #[serde(with = "http_serde::status_code")]
    status: StatusCode,
    error: String,
}

impl MediaFlowResponseError {
    pub(super) fn error(self) -> String {
        self.error
    }
}

impl Display for MediaFlowResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.status, self.error)
    }
}
