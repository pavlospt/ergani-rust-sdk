use serde::Deserialize;
use std::fmt::{Display, Formatter};

#[derive(thiserror::Error, Debug)]
pub enum APIError {
    /// Raised when an API request fails due to an authentication error
    AuthenticationFailed(#[source] reqwest::Error),
    /// Raised when an API request fails due to an unknown error
    General(#[from] reqwest::Error),
}

#[derive(Deserialize)]
pub struct ErganiError {
    pub message: String,
}

impl Display for APIError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            APIError::AuthenticationFailed(_) => write!(f, "Authentication failed"),
            APIError::General(_) => write!(f, "API error from Ergani API"),
        }
    }
}
