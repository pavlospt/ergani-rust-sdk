use crate::api_error::{APIError, ErganiError};
use crate::endpoint::{AUTHENTICATION_ENDPOINT, AUTHENTICATION_REFRESH_ENDPOINT};
use anyhow::{bail, Result};
use bon::{bon, Builder};
use chrono::{DateTime, Utc};
use serde_json::json;

use crate::auth::authentication_response::AuthenticationResponse;
use crate::auth::login_payload::LoginPayload;

/// Authentication handler for the Ergani API
#[allow(dead_code)]
#[derive(Clone, Builder)]
pub struct ErganiAuthenticationState {
    access_token: String,
    access_token_expired: i64,
    refresh_token: String,
    refresh_token_expired: DateTime<Utc>,
}

impl ErganiAuthenticationState {
    pub fn access_token(&self) -> &str {
        &self.access_token
    }

    pub fn access_token_expired(&self) -> i64 {
        self.access_token_expired
    }

    pub fn refresh_token(&self) -> &str {
        &self.refresh_token
    }

    pub fn refresh_token_expired(&self) -> &DateTime<Utc> {
        &self.refresh_token_expired
    }
}

#[derive(Clone)]
pub struct ErganiAuthenticator {
    base_url: String,
    http_client: reqwest::Client,
}

#[bon]
impl ErganiAuthenticator {
    #[builder]
    pub fn new(base_url: String) -> Self {
        ErganiAuthenticator {
            base_url,
            http_client: reqwest::Client::new(),
        }
    }

    /// Creates a new instance of the ErganiAuthenticator with the specified base URL
    ///
    /// # Arguments
    /// * `base_url` - The base URL of the Ergani API
    ///
    /// # Returns
    /// * `ErganiAuthenticationState` - The authentication state of the Ergani API
    pub async fn login(&self, login_payload: LoginPayload) -> Result<ErganiAuthenticationState> {
        let url = format!("{}{AUTHENTICATION_ENDPOINT}", self.base_url);

        let response = self
            .http_client
            .post(&url)
            .json(&json!({
                "Username": login_payload.username(),
                "Password": login_payload.password(),
                "UserType": "02"
            }))
            .send()
            .await?;

        if response.status().is_success() {
            let authentication_response: AuthenticationResponse =
                response.json::<AuthenticationResponse>().await?;

            let authentication_state = ErganiAuthenticationState::builder()
                .access_token(authentication_response.access_token())
                .access_token_expired(authentication_response.access_token_expired())
                .refresh_token(authentication_response.refresh_token())
                .refresh_token_expired(authentication_response.refresh_token_expired())
                .build();

            Ok(authentication_state)
        } else {
            let original_error = response.error_for_status_ref().unwrap_err();
            let error_text = response.text().await?.to_string();
            let ergani_error = ErganiError {
                message: error_text,
            };
            bail!(APIError::AuthenticationFailed(original_error, ergani_error))
        }
    }

    /// Refreshes the authentication token using the provided authentication state
    ///
    /// # Arguments
    /// * `auth_state` - The current authentication state containing the refresh token
    ///
    /// # Returns
    /// * `Result<ErganiAuthenticationState>` - A new authentication state with refreshed tokens
    ///
    /// # Errors
    /// * Returns `APIError::AuthenticationFailed` if the refresh request fails or returns an error response
    pub async fn refresh(
        &self,
        auth_state: &ErganiAuthenticationState,
    ) -> Result<ErganiAuthenticationState> {
        let url = format!("{}{AUTHENTICATION_REFRESH_ENDPOINT}", self.base_url);

        let response = self
            .http_client
            .post(&url)
            .json(&json!({
                "refreshToken": auth_state.refresh_token()
            }))
            .send()
            .await?;

        if response.status().is_success() {
            let authentication_response: AuthenticationResponse =
                response.json::<AuthenticationResponse>().await?;

            let authentication_state = ErganiAuthenticationState::builder()
                .access_token(authentication_response.access_token())
                .access_token_expired(authentication_response.access_token_expired())
                .refresh_token(authentication_response.refresh_token())
                .refresh_token_expired(authentication_response.refresh_token_expired())
                .build();

            Ok(authentication_state)
        } else {
            let original_error = response.error_for_status_ref().unwrap_err();
            let error_text = response.text().await?.to_string();
            let ergani_error = ErganiError {
                message: error_text,
            };
            bail!(APIError::AuthenticationFailed(original_error, ergani_error))
        }
    }
}
