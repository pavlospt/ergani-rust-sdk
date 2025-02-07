use chrono::{DateTime, NaiveDate, Utc};
use ergani::auth::authenticator::ErganiAuthenticationState;
use ergani::client::{ErganiClient, SubmissionResponse};
use ergani::models::company::company_weekly_schedule_builder::CompanyWeeklyScheduleBuilder;
use ergani::models::employee::employee_weekly_schedule_builder::EmployeeWeeklyScheduleBuilder;
use ergani::models::types::schedule_work_type::ScheduleWorkType;
use ergani::models::work_day_details_builder::WorkDayDetailsBuilder;
use anyhow::Result;

#[allow(dead_code)]
pub(crate) async fn submit_weekly_schedule(
    ergani_client: &ErganiClient,
    auth_state: ErganiAuthenticationState,
) -> Result<Vec<SubmissionResponse>> {
    let start_time = "2024-03-01T12:00:00Z".parse::<DateTime<Utc>>().unwrap();
    let end_time = "2024-03-01T20:00:00Z".parse::<DateTime<Utc>>().unwrap();
    let related_protocol_date = NaiveDate::from_ymd_opt(2024, 3, 1).unwrap();
    let schedule_date = NaiveDate::from_ymd_opt(2024, 3, 3).unwrap();

    let company_weekly_schedules = vec![CompanyWeeklyScheduleBuilder::builder()
        .set_business_branch_number(0)
        .set_start_date(NaiveDate::from_ymd_opt(2024, 3, 1).unwrap())
        .set_end_date(NaiveDate::from_ymd_opt(2024, 3, 2).unwrap())
        .set_employee_schedules(vec![EmployeeWeeklyScheduleBuilder::builder()
            .set_employee_tax_identification_number("123456789")
            .set_employee_last_name("Last")
            .set_employee_first_name("First")
            .set_schedule_date(schedule_date)
            .set_workday_details(vec![
                WorkDayDetailsBuilder::builder()
                    .set_work_type(ScheduleWorkType::WorkFromHome)
                    .set_start_time(start_time)
                    .set_end_time(end_time)
                    .build()?,
                WorkDayDetailsBuilder::builder()
                    .set_work_type(ScheduleWorkType::WorkFromOffice)
                    .set_start_time(start_time)
                    .set_end_time(end_time)
                    .build()?,
            ])
            .build()])
        .set_related_protocol_id(Some("1"))
        .set_related_protocol_date(Some(related_protocol_date))
        .build()];

    let response = ergani_client
        .submit_weekly_schedule(company_weekly_schedules, auth_state)
        .await?;

    Ok(response)
}
