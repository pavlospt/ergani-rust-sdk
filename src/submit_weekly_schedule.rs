use chrono::{DateTime, NaiveDate, Utc};
use ergani::client::{ErganiClient, SubmissionResponse};
use ergani::models::company::company_weekly_schedule_builder::CompanyWeeklyScheduleBuilder;
use ergani::models::employee::employee_weekly_schedule_builder::EmployeeWeeklyScheduleBuilder;
use ergani::models::types::schedule_work_type::ScheduleWorkType;
use ergani::models::work_day_details_builder::WorkDayDetailsBuilder;

pub(crate) async fn submit_weekly_schedule(
    ergani_client: &ErganiClient,
) -> anyhow::Result<Vec<SubmissionResponse>> {
    let company_weekly_schedules = vec![
        CompanyWeeklyScheduleBuilder::builder()
            .set_business_branch_number(10)
            .set_start_date(NaiveDate::from_ymd_opt(2024, 3, 1).unwrap())
            .set_end_date(NaiveDate::from_ymd_opt(2024, 3, 2).unwrap())
            .set_employee_schedules(vec![
                EmployeeWeeklyScheduleBuilder::builder()
                    .set_employee_tax_identification_number("0123456789".to_string())
                    .set_employee_last_name("Last".to_string())
                    .set_employee_first_name("First".to_string())
                    .set_schedule_date(NaiveDate::from_ymd_opt(2024, 3, 3).unwrap())
                    .set_workday_details(vec![
                        WorkDayDetailsBuilder::builder()
                            .set_work_type(ScheduleWorkType::WorkFromHome)
                            .set_start_time("2024-03-01T12:00:00Z".parse::<DateTime<Utc>>().unwrap())
                            .set_end_time("2024-03-01T20:00:00Z".parse::<DateTime<Utc>>().unwrap())
                            .build()?,
                        WorkDayDetailsBuilder::builder()
                            .set_work_type(ScheduleWorkType::WorkFromOffice)
                            .set_start_time("2024-03-02T12:00:00Z".parse::<DateTime<Utc>>().unwrap())
                            .set_end_time("2024-03-02T20:00:00Z".parse::<DateTime<Utc>>().unwrap())
                            .build()?,
                    ])
                    .build()
            ])
            .set_related_protocol_id(Some("1".to_string()))
            .set_related_protocol_date(Some(NaiveDate::from_ymd_opt(2024, 3, 1).unwrap()))
            .build()
    ];

    let response = ergani_client
        .submit_weekly_schedule(company_weekly_schedules)
        .await?;

    Ok(response)
}
