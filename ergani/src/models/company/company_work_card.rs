use crate::models::work_card::WorkCard;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde::Serialize as TypeSerialize;

/// Represents work card entries that are issued on a single business branch
/// * - `employer_tax_identification_number` - The employer's tax identification number
/// * - `business_branch_number` - The number identifying the specific business branch
/// * - `comments` - Additional comments related to the work cards
/// * - `card_details` - A list of `WorkCard` entries for the business branch
pub struct CompanyWorkCard {
    pub employer_tax_identification_number: String,
    pub business_branch_number: i64,
    pub comments: Option<String>,
    pub card_details: Vec<WorkCard>,
}

#[derive(TypeSerialize)]
struct CompanyWorkCardDetails {
    #[serde(rename = "CardDetails")]
    card_details: Vec<WorkCard>,
}

impl Serialize for CompanyWorkCard {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let sanitized_comments = match &self.comments {
            Some(comments) => comments,
            None => "",
        };

        let details = CompanyWorkCardDetails {
            card_details: self.card_details.clone(),
        };

        let mut company_work_card = serializer.serialize_struct("CompanyWorkCard", 4)?;
        company_work_card
            .serialize_field("f_afm_ergodoti", &self.employer_tax_identification_number)?;
        company_work_card.serialize_field("f_aa", &self.business_branch_number.to_string())?;
        company_work_card.serialize_field("f_comments", sanitized_comments)?;
        company_work_card.serialize_field("Details", &details)?;
        company_work_card.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::internal::tests::load_fixture_as_text;
    use crate::models::types::work_card_movement_type::WorkCardMovementType;
    use chrono::{DateTime, NaiveDate, Utc};

    #[test]
    fn test_serialize_company_work_card() {
        let date_time = "2014-11-28T12:00:00Z";
        let dt = date_time.parse::<DateTime<Utc>>().unwrap();

        let work_card = WorkCard {
            employee_tax_identification_number: "123456789".to_string(),
            employee_last_name: "ΠΑΠΑΔΟΠΟΥΛΟΣ".to_string(),
            employee_first_name: "ΓΕΩΡΓΙΟΣ".to_string(),
            work_card_submission_date: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
            work_card_movement_datetime: dt,
            late_declaration_justification: None,
            work_card_movement_type: WorkCardMovementType::Departure,
        };

        let company_work_card = CompanyWorkCard {
            employer_tax_identification_number: "987654321".to_string(),
            business_branch_number: 1,
            comments: Some("Σχόλια".to_string()),
            card_details: vec![work_card],
        };

        let serialized = serde_json::to_string(&company_work_card).unwrap();
        let expected = load_fixture_as_text("company_work_card_fixture.json");
        assert_eq!(serialized, expected);
    }
}
