#![allow(unused)]

use core::error;
use std::sync::Arc;

use crate::api_error::{APIError, ErganiError};
use crate::auth::authenticator::{ErganiAuthenticationState, ErganiAuthenticator};
use crate::endpoint::{
    DAILY_SCHEDULE_ENDPOINT, LOOKUP_SUBMISSIONS_ENDPOINT, OVERTIME_ENDPOINT, TRIAL_API_ENDPOINT,
    WEEKLY_SCHEDULE_ENDPOINT, WORK_CARD_ENDPOINT,
};
use crate::ergani_fetch_response::ErganiFetchResponse;
use crate::internal::deserializers::deserialize_datetime;
use crate::models::company::company_daily_schedule::CompanyDailySchedule;
use crate::models::company::company_overtime::CompanyOvertime;
use crate::models::company::company_weekly_schedule::CompanyWeeklySchedule;
use crate::models::company::company_work_card::CompanyWorkCard;
use crate::responses::day_schedule_response::DayScheduleResponseRoot;
use crate::responses::lookup_response::{LookupResponse, LookupRoot};
use crate::responses::overtime_response::OvertimeResponseRoot;
use crate::responses::week_schedule_response::WeekScheduleResponseRoot;
use crate::responses::work_card_response::WorkCardResponseRoot;
use anyhow::{bail, Result};
use bon::Builder;
use chrono::{DateTime, Utc};
use reqwest::header::HeaderValue;
use reqwest::{Method, Request, RequestBuilder, Response, StatusCode};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::{json, Value};
use tokio::sync::Mutex;
use tracing::{error, info};

use crate::auth::{self, login_payload};

#[derive(Clone)]
pub struct ErganiClient {
    base_url: String,
    http_client: reqwest::Client,
}

/// Represents a submission response from the Ergani API
/// * - `submission_id` - The unique identifier of the submission
/// * - `protocol` - The protocol associated with the submission
/// * - `submission_date` - The datetime of the submission
#[derive(Deserialize, Debug)]
pub struct SubmissionResponse {
    id: String,
    protocol: String,
    #[serde(rename = "submitDate", deserialize_with = "deserialize_datetime")]
    submit_date: DateTime<Utc>,
}

#[derive(Builder)]
struct ErganiRequestPayload {
    method: Method,
    endpoint: String,
    body: Option<Value>,
}

#[derive(Builder)]
struct ErganiRequestResponse {
    response: Option<Response>,
    auth_state: ErganiAuthenticationState,
}

impl ErganiClient {
    /// A client for interacting with the Ergani API
    /// * - `base_url` - The base URL of the Ergani API. Defaults to <https://trialeservices.yeka.gr/WebServicesAPI/api>.
    pub fn init(base_url: String) -> ErganiClient {
        let client = reqwest::Client::builder()
            .user_agent("Ergani Rust Client")
            .build()
            .unwrap();

        ErganiClient {
            http_client: client,
            base_url,
        }
    }

    /// Submits work card records (check-in, check-out) for employees to the Ergani API
    ///
    /// # Arguments:
    /// * - `company_work_cards[Vec<CompanyWorkCard>]` - A Vec of CompanyWorkCard instances to be submitted
    /// * - `auth_state` - The authentication state of the Ergani API
    ///
    /// # Returns:
    /// * - `[Vec<SubmissionResponse>]` - A list of SubmissionResponse that were parsed from the API response
    ///
    /// # Errors:
    /// * - `[APIError::General]` - An error occurred while communicating with the Ergani API
    /// * - `[APIError::AuthenticationError]` - Raised if there is an authentication error with the Ergani API
    pub async fn submit_work_card(
        &self,
        company_work_cards: Vec<CompanyWorkCard>,
        auth_state: ErganiAuthenticationState,
    ) -> Result<Vec<SubmissionResponse>> {
        let params = serde_json::to_value(company_work_cards)?;

        let request_payload = json!({
            "Cards": {
                "Card": params
            }
        });

        let payload = ErganiRequestPayload::builder()
            .method(Method::POST)
            .endpoint(WORK_CARD_ENDPOINT.to_string())
            .body(request_payload)
            .build();
        let response = self._request(&payload, &auth_state).await?;

        self._extract_submission_result(response.response, auth_state)
            .await
    }

    /// Submits overtime records for employees to the Ergani API
    ///
    /// # Arguments:
    /// * - `company_overtimes[Vec<CompanyOvertime>]` - A Vec of CompanyOvertime instances to be submitted
    /// * - `auth_state` - The authentication state of the Ergani API
    ///
    /// # Returns:
    /// * - `[Vec<SubmissionResponse>]` - A list of SubmissionResponse that were parsed from the API response
    ///
    /// # Errors:
    /// * - `[APIError::General]` - An error occurred while communicating with the Ergani API
    /// * - `[APIError::AuthenticationError]` - Raised if there is an authentication error with the Ergani API
    pub async fn submit_overtime(
        &self,
        company_overtimes: Vec<CompanyOvertime>,
        auth_state: ErganiAuthenticationState,
    ) -> Result<Vec<SubmissionResponse>> {
        let params = serde_json::to_value(company_overtimes)?;
        let request_payload = json!({
            "Overtimes": {
                "Overtime": params
            }
        });

        let payload = ErganiRequestPayload::builder()
            .method(Method::POST)
            .endpoint(OVERTIME_ENDPOINT.to_string())
            .body(request_payload)
            .build();
        let response = self._request(&payload, &auth_state).await?;

        self._extract_submission_result(response.response, auth_state)
            .await
    }

    /// Submits schedule records that are updated on a daily basis for employees to the Ergani API
    ///
    /// # Arguments:
    /// * - `company_daily_schedules[Vec<CompanyDailySchedule>]` - A Vec of CompanyDailySchedule instances to be submitted
    /// * - `auth_state` - The authentication state of the Ergani API
    /// # Returns:
    /// * - `[Vec<SubmissionResponse>]` - A list of SubmissionResponse that were parsed from the API response
    ///
    /// # Errors:
    /// * - `[APIError::General]` - An error occurred while communicating with the Ergani API
    /// * - `[APIError::AuthenticationError]` - Raised if there is an authentication error with the Ergani API
    pub async fn submit_daily_schedule(
        &self,
        company_daily_schedules: Vec<CompanyDailySchedule>,
        auth_state: ErganiAuthenticationState,
    ) -> Result<Vec<SubmissionResponse>> {
        let params = serde_json::to_value(company_daily_schedules)?;
        let request_payload = json!({
            "WTOS": {
                "WTO": params
            }
        });

        let payload = ErganiRequestPayload::builder()
            .method(Method::POST)
            .endpoint(DAILY_SCHEDULE_ENDPOINT.to_string())
            .body(request_payload)
            .build();

        let response = self._request(&payload, &auth_state).await?;

        self._extract_submission_result(response.response, auth_state)
            .await
    }

    /// Submits weekly schedule records for employees to the Ergani API
    ///
    /// # Arguments:
    /// * - `company_weekly_schedules[Vec<CompanyWeeklySchedule>]` - A Vec of CompanyWeeklySchedule instances to be submitted
    /// * - `auth_state` - The authentication state of the Ergani API
    /// # Returns:
    /// * - `[Vec<SubmissionResponse>]` - A list of SubmissionResponse that were parsed from the API response
    ///
    /// # Errors:
    /// * - `[APIError::General]` - An error occurred while communicating with the Ergani API
    /// * - `[APIError::AuthenticationError]` - Raised if there is an authentication error with the Ergani API
    pub async fn submit_weekly_schedule(
        &self,
        company_weekly_schedules: Vec<CompanyWeeklySchedule>,
        auth_state: ErganiAuthenticationState,
    ) -> Result<Vec<SubmissionResponse>> {
        let params = serde_json::to_value(company_weekly_schedules)?;
        let request_payload = json!({
            "WTOS": {
                "WTO": params
            }
        });

        let payload = ErganiRequestPayload::builder()
            .method(Method::POST)
            .endpoint(WEEKLY_SCHEDULE_ENDPOINT.to_string())
            .body(request_payload)
            .build();

        let response = self._request(&payload, &auth_state).await?;

        self._extract_submission_result(response.response, auth_state)
            .await
    }

    /// Fetches the submissions from the Ergani API
    ///
    /// # Arguments:
    /// * - `auth_state` - The authentication state of the Ergani API
    ///
    /// # Returns:
    /// * - `LookupRoot` - The submissions from the Ergani API
    pub async fn fetch_submissions(
        &self,
        auth_state: ErganiAuthenticationState,
    ) -> Result<ErganiFetchResponse<LookupRoot>> {
        let payload = ErganiRequestPayload::builder()
            .method(Method::GET)
            .endpoint(LOOKUP_SUBMISSIONS_ENDPOINT.to_string())
            .maybe_body(None)
            .build();

        let response = self._request(&payload, &auth_state).await?;

        self._extract_fetch_result(response.response, auth_state)
            .await
    }

    /// Fetches the weekly schedule from the Ergani API
    ///
    /// # Arguments:
    /// * - `auth_state` - The authentication state of the Ergani API
    ///
    /// # Returns:
    /// * - `WeekScheduleResponseRoot` - The weekly schedule from the Ergani API
    pub async fn fetch_weekly_schedule(
        &self,
        auth_state: ErganiAuthenticationState,
    ) -> Result<ErganiFetchResponse<WeekScheduleResponseRoot>> {
        let payload = ErganiRequestPayload::builder()
            .method(Method::GET)
            .endpoint(WEEKLY_SCHEDULE_ENDPOINT.to_string())
            .maybe_body(None)
            .build();
        let response = self._request(&payload, &auth_state).await?;

        self._extract_fetch_result(response.response, auth_state)
            .await
    }

    /// Fetches the daily schedule from the Ergani API
    ///
    /// # Arguments:
    /// * - `auth_state` - The authentication state of the Ergani API
    ///
    /// # Returns:
    /// * - `DayScheduleResponseRoot` - The daily schedule from the Ergani API
    pub async fn fetch_daily_schedule(
        &self,
        auth_state: ErganiAuthenticationState,
    ) -> Result<ErganiFetchResponse<DayScheduleResponseRoot>> {
        let payload = ErganiRequestPayload::builder()
            .method(Method::GET)
            .endpoint(DAILY_SCHEDULE_ENDPOINT.to_string())
            .maybe_body(None)
            .build();
        let response = self._request(&payload, &auth_state).await?;

        self._extract_fetch_result(response.response, auth_state)
            .await
    }

    /// Fetches the work cards from the Ergani API
    ///
    /// # Arguments:
    /// * - `auth_state` - The authentication state of the Ergani API
    ///
    /// # Returns:
    /// * - `WorkCardResponseRoot` - The work cards from the Ergani API
    pub async fn fetch_work_cards(
        &self,
        auth_state: ErganiAuthenticationState,
    ) -> Result<ErganiFetchResponse<WorkCardResponseRoot>> {
        let payload = ErganiRequestPayload::builder()
            .method(Method::GET)
            .endpoint(WORK_CARD_ENDPOINT.to_string())
            .maybe_body(None)
            .build();
        let response = self._request(&payload, &auth_state).await?;

        self._extract_fetch_result(response.response, auth_state)
            .await
    }

    /// Fetches the overtime records from the Ergani API
    ///
    /// # Arguments:
    /// * - `auth_state` - The authentication state of the Ergani API
    ///
    /// # Returns:
    /// * - `OvertimeResponseRoot` - The overtime records from the Ergani API
    pub async fn fetch_overtimes(
        &self,
        auth_state: ErganiAuthenticationState,
    ) -> Result<ErganiFetchResponse<OvertimeResponseRoot>> {
        let payload = ErganiRequestPayload::builder()
            .method(Method::GET)
            .endpoint(OVERTIME_ENDPOINT.to_string())
            .maybe_body(None)
            .build();
        let response = self._request(&payload, &auth_state).await?;

        self._extract_fetch_result(response.response, auth_state)
            .await
    }

    /// Sends a request to the specified endpoint using the given HTTP method and payload
    ///
    /// # Arguments
    /// * - `payload` - The JSON value to be sent as the request payload
    /// * - `auth_state` - The authentication state of the Ergani API
    ///
    /// # Returns
    /// * - `ErganiRequestResponse` - The response from the Ergani API
    ///
    /// # Errors
    /// * - `ApiError` - errors may occur for network-related errors
    async fn _request(
        &self,
        payload: &ErganiRequestPayload,
        auth_state: &ErganiAuthenticationState,
    ) -> Result<ErganiRequestResponse> {
        let url = format!("{}{}", self.base_url, payload.endpoint);

        let mut request_builder = self
            .http_client
            .request(payload.method.clone(), url)
            .bearer_auth(auth_state.access_token());

        if let Some(body) = payload.body.clone() {
            request_builder = request_builder.json(&body);
        }

        let response = request_builder.send().await?;

        self._handle_response(payload, response, auth_state).await
    }

    /// Handles the HTTP response, raising exceptions for error status codes and returning the response for successful ones
    /// # Arguments
    /// * - `payload` - The payload of the request
    /// * - `response` - The response object to handle
    /// * - `auth_state` - The authentication state of the Ergani API
    ///
    /// # Returns:
    /// * - `ErganiRequestResponse` - The response from the Ergani API
    ///
    /// # Errors:
    /// * - `[APIError::General]` - An error occurred while communicating with the Ergani API
    /// * - `[APIError::NotFound]` - Raised if the requested resource was not found
    /// * - `[APIError::AuthenticationError]` - Raised if there is an authentication error with the Ergani API
    async fn _handle_response(
        &self,
        payload: &ErganiRequestPayload,
        response: Response,
        auth_state: &ErganiAuthenticationState,
    ) -> Result<ErganiRequestResponse> {
        let status = response.status();

        /// Refresh the authentication token if the response is a 401 Unauthorized
        if status == StatusCode::UNAUTHORIZED {
            info!("Refreshing authentication token");

            let ergani_authenticator = ErganiAuthenticator::builder()
                .base_url(self.base_url.clone())
                .build();

            let refreshed_auth_state = ergani_authenticator.refresh(auth_state).await;

            if refreshed_auth_state.is_err() {
                error!("Failed to refresh authentication token");
                bail!(refreshed_auth_state.err().unwrap())
            }

            let url = format!("{}{}", self.base_url, payload.endpoint);

            let refreshed_auth_state = refreshed_auth_state.unwrap();

            let mut request_builder = self
                .http_client
                .request(payload.method.clone(), url)
                .bearer_auth(refreshed_auth_state.access_token());

            if let Some(body) = payload.body.clone() {
                request_builder = request_builder.json(&body);
            }

            let response = request_builder.send().await?;

            if response.status().is_success() {
                let auth_state = ErganiAuthenticationState::builder()
                    .access_token(refreshed_auth_state.access_token().to_string())
                    .access_token_expired(refreshed_auth_state.access_token_expired())
                    .refresh_token(refreshed_auth_state.refresh_token().to_string())
                    .refresh_token_expired(refreshed_auth_state.refresh_token_expired().to_utc())
                    .build();

                /// Return the response from the Ergani API, with the new authentication state
                let request_response = ErganiRequestResponse::builder()
                    .maybe_response(Some(response))
                    .auth_state(auth_state)
                    .build();

                return Ok(request_response);
            }

            let original_error = response.error_for_status_ref().unwrap_err();
            let error_text = response.text().await?.to_string();
            let ergani_error = ErganiError {
                message: error_text,
            };
            bail!(APIError::AuthenticationFailed(original_error, ergani_error))
        }

        if status == StatusCode::NOT_FOUND {
            let original_error = response.error_for_status_ref().unwrap_err();
            let ergani_error = response.json::<ErganiError>().await?;
            bail!(APIError::NotFound(original_error, ergani_error))
        }

        if status == StatusCode::NO_CONTENT {
            let request_response = ErganiRequestResponse::builder()
                .maybe_response(None)
                .auth_state(auth_state.clone())
                .build();

            return Ok(request_response);
        }

        if status.is_success() {
            let request_response = ErganiRequestResponse::builder()
                .maybe_response(Some(response))
                .auth_state(auth_state.clone())
                .build();

            Ok(request_response)
        } else {
            let original_error = response.error_for_status_ref().unwrap_err();
            let ergani_error = response.json::<ErganiError>().await?;
            bail!(APIError::General(original_error, ergani_error))
        }
    }

    /// Extracts the submission result from the Ergani API response
    /// # Arguments:
    /// * - `response` - The response object from the Ergani API
    ///
    /// # Returns:
    ///  * - `Vec<SubmissionResponse>` - A Vec of submission responses parsed from the API response
    async fn _extract_submission_result(
        &self,
        response: Option<Response>,
        auth_state: ErganiAuthenticationState,
    ) -> Result<Vec<SubmissionResponse>> {
        if response.is_none() {
            return Ok(vec![]);
        }

        let response: Vec<SubmissionResponse> = response.unwrap().json().await?;

        Ok(response)
    }

    /// Extracts the lookup result from the Ergani API response
    /// # Arguments:
    /// * - `response` - The response object from the Ergani API
    ///
    /// # Returns:
    ///  * - `T` - A Vec of T responses parsed from the API response
    async fn _extract_fetch_result<T: DeserializeOwned>(
        &self,
        response: Option<Response>,
        auth_state: ErganiAuthenticationState,
    ) -> Result<ErganiFetchResponse<T>> {
        let response: T = response.unwrap().json().await?;

        Ok(ErganiFetchResponse::builder()
            .response(response)
            .auth_state(auth_state)
            .build())
    }
}
