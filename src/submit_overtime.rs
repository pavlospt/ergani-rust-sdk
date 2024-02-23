use chrono::{DateTime, NaiveDate, Utc};
use ergani::client::{ErganiClient, SubmissionResponse};
use ergani::models::company::company_overtime_builder::CompanyOvertimeBuilder;
use ergani::models::overtime_builder::OvertimeBuilder;
use ergani::models::types::overtime_justification_type::OvertimeJustificationType;
use ergani::models::weekly_work_days::WeeklyWorkDays;

pub(crate) async fn submit_overtime(
    ergani_client: &ErganiClient,
) -> anyhow::Result<Vec<SubmissionResponse>> {
    let company_overtimes = vec![
        CompanyOvertimeBuilder::builder()
            .set_business_branch_number(12)
            .set_sepe_service_code("10".to_string())
            .set_business_primary_activity_code("100".to_string())
            .set_business_branch_activity_code("101".to_string())
            .set_kallikratis_municipal_code("100".to_string())
            .set_legal_representative_tax_identification_number("0123456789".to_string())
            .set_employee_overtimes(vec![OvertimeBuilder::builder()
                .set_employee_tax_identification_number("0123456789".to_string())
                .set_employee_social_security_number("0123456789".to_string())
                .set_employee_last_name("Last".to_string())
                .set_employee_first_name("First".to_string())
                .set_overtime_date(NaiveDate::from_ymd_opt(2024, 3, 1).unwrap())
                .set_overtime_start_time("2024-03-01T12:00:00Z".parse::<DateTime<Utc>>().unwrap())
                .set_overtime_end_time("2024-03-01T20:00:00Z".parse::<DateTime<Utc>>().unwrap())
                .set_overtime_cancellation(false)
                .set_employee_profession_code("".to_string())
                .set_overtime_justification(
                    OvertimeJustificationType::AccidentPreventionOrDamageRestoration,
                )
                .set_weekly_workdays_number(WeeklyWorkDays::Five)
                .set_asee_approval(Some("ΑΣΕΕ".to_string()))
                .build()
                .unwrap()])
            .set_related_protocol_id(Some("Αρ. Πρωτ. Σχετ.".to_string()))
            .set_related_protocol_date(Some(NaiveDate::from_ymd_opt(2024, 3, 1).unwrap()))
            .set_employer_organization(Some("Εργοδότης".to_string()))
            .set_business_secondary_activity_code_1(Some("ΚΑΔ 1".to_string()))
            .set_business_secondary_activity_code_2(Some("ΚΑΔ 2".to_string()))
            .set_business_secondary_activity_code_3(Some("ΚΑΔ 3".to_string()))
            .set_business_secondary_activity_code_4(Some("ΚΑΔ 4".to_string()))
            .set_comments(Some("Σχόλια".to_string()))
            .build(),
    ];

    let response = ergani_client.submit_overtime(company_overtimes).await?;

    Ok(response)
}
