use serde::Deserialize;
use std::fmt::{Display, Formatter};

#[derive(Deserialize, Debug)]
pub struct ErganiError {
    pub(crate) message: String,
}

impl ErganiError {
    pub fn message(&self) -> &str {
        &self.message.trim()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum APIError {
    /// Raised when an API request fails due to an authentication error
    AuthenticationFailed(#[source] reqwest::Error, ErganiError),
    /// Raised when an API request fails due to an unknown error
    General(#[source] reqwest::Error, ErganiError),
    /// Raised when an API request fails due to a 404 error
    NotFound(#[source] reqwest::Error, ErganiError),
}

impl Display for APIError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            APIError::AuthenticationFailed(_, ergani_error) => {
                write!(f, "Authentication failed: {}", ergani_error.message())
            }
            APIError::General(_, ergani_error) => {
                write!(f, "{}", ergani_error.message())
            }
            APIError::NotFound(_, ergani_error) => {
                write!(f, "Resource not found: {}", ergani_error.message())
            }
        }
    }
}
