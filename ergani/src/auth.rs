use crate::api_error::{APIError, ErganiError};
use crate::endpoint::AUTHENTICATION_ENDPOINT;
use anyhow::{bail, Result};
use bon::Builder;
use chrono::{DateTime, Utc};
use reqwest::header;
use reqwest::header::HeaderMap;
use serde_json::json;

/// Authentication handler for the Ergani API
#[allow(dead_code)]
#[derive(Clone, Builder)]
pub struct ErganiAuthentication {
    access_token: String,
    access_token_expired: i64,
    refresh_token: String,
    refresh_token_expired: DateTime<Utc>,
}

#[derive(serde::Deserialize)]
struct AuthenticationResponse {
    #[serde(rename = "accessToken")]
    access_token: String,
    #[serde(rename = "accessTokenExpired")]
    access_token_expired: i64,
    #[serde(rename = "refreshToken")]
    refresh_token: String,
    #[serde(rename = "refreshTokenExpired")]
    refresh_token_expired: DateTime<Utc>,
}

impl ErganiAuthentication {
    pub async fn new(
        username: String,
        password: String,
        base_url: String,
    ) -> Result<ErganiAuthentication> {
        ErganiAuthentication::authenticate(username, password, base_url).await
    }

    async fn authenticate(
        username: String,
        password: String,
        url: String,
    ) -> Result<ErganiAuthentication> {
        let client = reqwest::Client::new();
        let url = format!("{url}{AUTHENTICATION_ENDPOINT}");

        let response = client
            .post(&url)
            .json(&json!({
                "Username": username,
                "Password": password,
                "UserType": "02"
            }))
            .send()
            .await?;

        if response.status().is_success() {
            let authentication_response: AuthenticationResponse =
                response.json::<AuthenticationResponse>().await?;

            let authentication = ErganiAuthentication::builder()
                .access_token(authentication_response.access_token)
                .access_token_expired(authentication_response.access_token_expired)
                .refresh_token(authentication_response.refresh_token)
                .refresh_token_expired(authentication_response.refresh_token_expired)
                .build();

            Ok(authentication)
        } else {
            let original_error = response.error_for_status_ref().unwrap_err();
            let error_text = response.text().await?.to_string();
            let ergani_error = ErganiError {
                message: error_text,
            };
            bail!(APIError::AuthenticationFailed(original_error, ergani_error))
        }
    }

    pub fn auth_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let mut auth_value =
            header::HeaderValue::from_str(format!("Bearer {}", self.access_token).as_str())
                .unwrap();
        auth_value.set_sensitive(true);
        headers.insert(header::AUTHORIZATION, auth_value);
        headers
    }
}
