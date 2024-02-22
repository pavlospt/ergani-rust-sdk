#![allow(unused)]

use crate::api_error::APIError;
use crate::auth::ErganiAuthentication;
use crate::endpoint::{
    SUBMIT_DAILY_SCHEDULE_ENDPOINT, SUBMIT_OVERTIME_ENDPOINT, SUBMIT_WEEKLY_SCHEDULE_ENDPOINT,
    SUBMIT_WORK_CARD_ENDPOINT,
};
use crate::internal::deserializers::deserialize_datetime;
use crate::models::company::company_daily_schedule::CompanyDailySchedule;
use crate::models::company::company_overtime::CompanyOvertime;
use crate::models::company::company_weekly_schedule::CompanyWeeklySchedule;
use crate::models::company::company_work_card::CompanyWorkCard;
use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use reqwest::{Method, Response, StatusCode};
use serde::Deserialize;
use serde_json::{json, Value};

const POST_METHOD: &str = "POST";

pub struct ErganiClient {
    authentication: ErganiAuthentication,
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

impl ErganiClient {
    /// A client for interacting with the Ergani API
    /// * - `username` - The username for authentication with Ergani
    /// * - `password` - The password for authentication with Ergani
    /// * - `base_url` - The base URL of the Ergani API. Defaults to <https://trialeservices.yeka.gr/WebServicesAPI/api>.
    pub async fn new(
        username: String,
        password: String,
        base_url: Option<String>,
    ) -> Result<ErganiClient> {
        let ergani_authentication = ErganiAuthentication::new(username, password, base_url).await;

        if ergani_authentication.is_err() {
            bail!(ergani_authentication.err().unwrap())
        }

        Ok(ErganiClient {
            authentication: ergani_authentication.unwrap(),
        })
    }

    /// Submits work card records (check-in, check-out) for employees to the Ergani API
    ///
    /// # Arguments:
    /// * - `company_work_cards[Vec<CompanyWorkCard>]` - A Vec of CompanyWorkCard instances to be submitted
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
    ) -> Result<Vec<SubmissionResponse>> {
        let params = serde_json::to_value(company_work_cards)?;
        let request_payload = json!({
            "Cards": {
                "Card": params
            }
        });

        let response = self
            ._request(POST_METHOD, SUBMIT_WORK_CARD_ENDPOINT, request_payload)
            .await?;

        self._extract_submission_result(response).await
    }

    /// Submits overtime records for employees to the Ergani API
    ///
    /// # Arguments:
    /// * - `company_overtimes[Vec<CompanyOvertime>]` - A Vec of CompanyOvertime instances to be submitted
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
    ) -> Result<Vec<SubmissionResponse>> {
        let params = serde_json::to_value(company_overtimes)?;
        let request_payload = json!({
            "Overtimes": {
                "Overtime": params
            }
        });

        let response = self
            ._request(POST_METHOD, SUBMIT_OVERTIME_ENDPOINT, request_payload)
            .await?;

        self._extract_submission_result(response).await
    }

    /// Submits schedule records that are updated on a daily basis for employees to the Ergani API
    ///
    /// # Arguments:
    /// * - `company_daily_schedules[Vec<CompanyDailySchedule>]` - A Vec of CompanyDailySchedule instances to be submitted
    ///
    /// # Returns:
    /// * - `[Vec<SubmissionResponse>]` - A list of SubmissionResponse that were parsed from the API response
    ///
    /// # Errors:
    /// * - `[APIError::General]` - An error occurred while communicating with the Ergani API
    /// * - `[APIError::AuthenticationError]` - Raised if there is an authentication error with the Ergani API
    pub async fn submit_daily_schedule(
        &self,
        company_daily_schedules: Vec<CompanyDailySchedule>,
    ) -> Result<Vec<SubmissionResponse>> {
        let params = serde_json::to_value(company_daily_schedules)?;
        let request_payload = json!({
            "WTOS": {
                "WTO": params
            }
        });

        let response = self
            ._request(POST_METHOD, SUBMIT_DAILY_SCHEDULE_ENDPOINT, request_payload)
            .await?;

        self._extract_submission_result(response).await
    }

    /// Submits weekly schedule records for employees to the Ergani API
    ///
    /// # Arguments:
    /// * - `company_weekly_schedules[Vec<CompanyWeeklySchedule>]` - A Vec of CompanyWeeklySchedule instances to be submitted
    ///
    /// # Returns:
    /// * - `[Vec<SubmissionResponse>]` - A list of SubmissionResponse that were parsed from the API response
    ///
    /// # Errors:
    /// * - `[APIError::General]` - An error occurred while communicating with the Ergani API
    /// * - `[APIError::AuthenticationError]` - Raised if there is an authentication error with the Ergani API
    pub async fn submit_weekly_schedule(
        &self,
        company_weekly_schedules: Vec<CompanyWeeklySchedule>,
    ) -> Result<Vec<SubmissionResponse>> {
        let params = serde_json::to_value(company_weekly_schedules)?;
        let request_payload = json!({
            "WTOS": {
                "WTO": params
            }
        });

        let response = self
            ._request(
                POST_METHOD,
                SUBMIT_WEEKLY_SCHEDULE_ENDPOINT,
                request_payload,
            )
            .await?;

        self._extract_submission_result(response).await
    }

    /// Sends a request to the specified endpoint using the given HTTP method and payload
    /// # Arguments
    /// * - `method` - The HTTP method to use for the request (e.g., 'GET', 'POST')
    /// * - `endpoint` - The API endpoint to which the request should be sent to
    /// * - `payload` - The JSON value to be sent as the request payload
    ///
    /// # Returns
    /// * - `Option[reqwest::Response]`: The response object from the reqwest library.
    ///                                  Returns None for 204 No Content responses.
    /// # Errors
    /// * - `ApiError` - errors may occur for network-related errors
    async fn _request(
        &self,
        method: &str,
        endpoint: &str,
        params: Value,
    ) -> Result<Option<Response>> {
        let client = reqwest::Client::builder()
            .default_headers(self.authentication.auth_headers())
            .build()?;

        let url = format!("{}/{}", self.authentication.base_url(), endpoint);

        let response: Response = client
            .request(Method::from_bytes(method.as_bytes()).unwrap(), url)
            .json(&params)
            .send()
            .await?;

        self._handle_response(response).await
    }

    /// Handles the HTTP response, raising exceptions for error status codes and returning the response for successful ones
    /// # Arguments
    /// * - `response` - The response object to handle
    ///
    /// # Returns:
    /// * - `Result<Option<reqwest::Response>>` - The original response object for successful requests or None for 204 No Content responses
    ///
    /// # Errors:
    /// * - `[APIError::General]` - An error occurred while communicating with the Ergani API
    /// * - `[APIError::AuthenticationError]` - Raised if there is an authentication error with the Ergani API
    async fn _handle_response(&self, response: Response) -> Result<Option<Response>> {
        let status = response.status();

        if status == StatusCode::UNAUTHORIZED {
            let response_error = response.error_for_status().unwrap_err();
            bail!(APIError::AuthenticationFailed(response_error))
        }

        if status == StatusCode::NO_CONTENT {
            return Ok(None);
        }

        if status.is_success() {
            Ok(Some(response))
        } else {
            let error = response.error_for_status().unwrap_err();
            bail!(APIError::General(error))
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
    ) -> Result<Vec<SubmissionResponse>> {
        if response.is_none() {
            return Ok(vec![]);
        }

        let response: Vec<SubmissionResponse> = response.unwrap().json().await?;

        Ok(response)
    }
}
