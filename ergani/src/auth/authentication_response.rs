use chrono::{DateTime, Utc};

#[derive(serde::Deserialize)]
pub struct AuthenticationResponse {
    #[serde(rename = "accessToken")]
    access_token: String,
    #[serde(rename = "accessTokenExpired")]
    access_token_expired: i64,
    #[serde(rename = "refreshToken")]
    refresh_token: String,
    #[serde(rename = "refreshTokenExpired")]
    refresh_token_expired: DateTime<Utc>,
}

impl AuthenticationResponse {
    pub fn access_token(&self) -> String {
        self.access_token.to_string()
    }

    pub fn access_token_expired(&self) -> i64 {
        self.access_token_expired
    }

    pub fn refresh_token(&self) -> String {
        self.refresh_token.to_string()
    }

    pub fn refresh_token_expired(&self) -> DateTime<Utc> {
        self.refresh_token_expired
    }
}
