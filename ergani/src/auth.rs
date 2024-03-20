use crate::api_error::{APIError, ErganiError};
use crate::endpoint::AUTHENTICATION_ENDPOINT;
use anyhow::{bail, Result};
use reqwest::header;
use reqwest::header::HeaderMap;
use serde_json::json;

/// Authentication handler for the Ergani API
pub struct ErganiAuthentication {
    pub access_token: String,
}

#[derive(serde::Deserialize)]
struct AuthenticationResponse {
    #[serde(rename = "accessToken")]
    access_token: String,
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

            Ok(ErganiAuthentication {
                access_token: authentication_response.access_token,
            })
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
