use anyhow::Result;
use chrono::{NaiveDate, NaiveDateTime};

use ergani::client::{ErganiClient, SubmissionResponse};
use ergani::models::company::company_work_card_builder::CompanyWorkCardBuilder;
use ergani::models::types::late_declaration_justification_type::LateDeclarationJustificationType;
use ergani::models::types::work_card_movement_type::WorkCardMovementType;
use ergani::models::work_card_builder::WorkCardBuilder;

#[allow(dead_code)]
pub(crate) async fn submit_work_card(
    ergani_client: &ErganiClient,
) -> Result<Vec<SubmissionResponse>> {
    let work_card_movement_datetime =
        NaiveDateTime::parse_from_str("2024-03-20 10:00", "%Y-%m-%d %H:%M")
            .unwrap()
            .and_utc();

    let work_card_submission_date = NaiveDate::parse_from_str("2022-05-04", "%Y-%m-%d").unwrap();

    let work_card = vec![CompanyWorkCardBuilder::builder()
        .set_employer_tax_identification_number("123456789")
        .set_business_branch_number(0)
        .set_comments(Some("Σχόλια"))
        .set_card_details(vec![WorkCardBuilder::builder()
            .set_employee_tax_identification_number("123456789")
            .set_employee_last_name("Last")
            .set_employee_first_name("First")
            .set_work_card_movement_type(WorkCardMovementType::Arrival)
            .set_work_card_submission_date(work_card_submission_date)
            .set_work_card_movement_datetime(work_card_movement_datetime)
            .set_late_declaration_justification(Some(LateDeclarationJustificationType::PowerOutage))
            .build()?])
        .build()];

    let response = ergani_client.submit_work_card(work_card).await?;

    response.iter().for_each(|r| {
        println!("{:?}", r);
    });

    Ok(response)
}
