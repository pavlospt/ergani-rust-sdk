#![allow(unused)]

use crate::api_error::{APIError, ErganiError};
use crate::auth::ErganiAuthentication;
use crate::endpoint::{
    DAILY_SCHEDULE_ENDPOINT, EMPLOYEE_ANNOUNCEMENT_ENDPOINT, LOOKUP_SUBMISSIONS_ENDPOINT,
    OVERTIME_ENDPOINT, TRIAL_API_ENDPOINT, WEEKLY_SCHEDULE_ENDPOINT, WORK_CARD_ENDPOINT,
};
use crate::internal::deserializers::deserialize_datetime;
use crate::models::company::company_daily_schedule::CompanyDailySchedule;
use crate::models::company::company_overtime::CompanyOvertime;
use crate::models::company::company_weekly_schedule::CompanyWeeklySchedule;
use crate::models::company::company_work_card::CompanyWorkCard;
use crate::models::employee::employee_announcement::{self, EmployeeAnnouncement};
use crate::responses::day_schedule_response::DayScheduleResponseRoot;
use crate::responses::lookup_response::{LookupResponse, LookupRoot};
use crate::responses::overtime_response::OvertimeResponseRoot;
use crate::responses::week_schedule_response::WeekScheduleResponseRoot;
use crate::responses::work_card_response::WorkCardResponseRoot;
use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use reqwest::header::HeaderValue;
use reqwest::{Method, RequestBuilder, Response, StatusCode};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::{json, Value};

pub struct ErganiClient {
    authentication: ErganiAuthentication,
    base_url: String,
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
        let ergani_base_url = base_url.map_or(TRIAL_API_ENDPOINT.to_string(), |url| url);
        let ergani_authentication =
            ErganiAuthentication::new(username, password, ergani_base_url.clone()).await;
        if ergani_authentication.is_err() {
            bail!(ergani_authentication.err().unwrap())
        }

        Ok(ErganiClient {
            authentication: ergani_authentication.unwrap(),
            base_url: ergani_base_url,
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
            ._request(Method::POST, WORK_CARD_ENDPOINT, Some(request_payload))
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
            ._request(Method::POST, OVERTIME_ENDPOINT, Some(request_payload))
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
            ._request(Method::POST, DAILY_SCHEDULE_ENDPOINT, Some(request_payload))
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
                Method::POST,
                WEEKLY_SCHEDULE_ENDPOINT,
                Some(request_payload),
            )
            .await?;

        self._extract_submission_result(response).await
    }

    pub async fn submit_employee_announcements(
        &self,
        employee_announcements: Vec<EmployeeAnnouncement>,
    ) -> Result<Vec<SubmissionResponse>> {
        let params = serde_json::to_value(employee_announcements)?;
        let request_payload = json!({
            "AnaggeliesE3": {
                "AnaggeliaE3": params
            }
        });
        let response = self
            ._request(
                Method::POST,
                EMPLOYEE_ANNOUNCEMENT_ENDPOINT,
                Some(request_payload),
            )
            .await?;

        dbg!(&response);

        self._extract_submission_result(response).await
    }

    pub async fn fetch_submissions(&self) -> Result<LookupRoot> {
        let response = self
            ._request(Method::GET, LOOKUP_SUBMISSIONS_ENDPOINT, None)
            .await?;

        self._extract_fetch_result(response).await
    }

    pub async fn fetch_weekly_schedule(&self) -> Result<WeekScheduleResponseRoot> {
        let response = self
            ._request(Method::GET, WEEKLY_SCHEDULE_ENDPOINT, None)
            .await?;

        self._extract_fetch_result(response).await
    }

    pub async fn fetch_daily_schedule(&self) -> Result<DayScheduleResponseRoot> {
        let response = self
            ._request(Method::GET, DAILY_SCHEDULE_ENDPOINT, None)
            .await?;

        self._extract_fetch_result(response).await
    }

    pub async fn fetch_work_cards(&self) -> Result<WorkCardResponseRoot> {
        let response = self._request(Method::GET, WORK_CARD_ENDPOINT, None).await?;

        self._extract_fetch_result(response).await
    }

    pub async fn fetch_overtimes(&self) -> Result<OvertimeResponseRoot> {
        let response = self._request(Method::GET, OVERTIME_ENDPOINT, None).await?;

        self._extract_fetch_result(response).await
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
        method: Method,
        endpoint: &str,
        body: Option<Value>,
    ) -> Result<Option<Response>> {
        let request_headers = self.authentication.auth_headers();
        let client = reqwest::Client::builder()
            .default_headers(request_headers)
            .user_agent("Ergani Rust Client")
            .build()?;

        let url = format!("{}{}", self.base_url, endpoint);

        let mut request_builder = client.request(method, url);

        if let Some(body) = body {
            request_builder = request_builder.json(&body);
        }

        let response = request_builder.send().await?;

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
    /// * - `[APIError::NotFound]` - Raised if the requested resource was not found
    /// * - `[APIError::AuthenticationError]` - Raised if there is an authentication error with the Ergani API
    async fn _handle_response(&self, response: Response) -> Result<Option<Response>> {
        let status = response.status();

        if status == StatusCode::UNAUTHORIZED {
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
            return Ok(None);
        }

        if status.is_success() {
            Ok(Some(response))
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
    ) -> Result<T> {
        let response: T = response.unwrap().json().await?;

        Ok(response)
    }
}
