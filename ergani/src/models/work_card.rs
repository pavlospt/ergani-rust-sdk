use crate::internal::utils::format_datetime;
use crate::models::types::late_declaration_justification_type::LateDeclarationJustificationType;
use crate::models::types::work_card_movement_type::WorkCardMovementType;
use chrono::{DateTime, NaiveDate, Utc};
use serde::ser::{Serialize, SerializeStruct, Serializer};

/// Represents a work card entry for an employee
/// * - `employee_tax_identification_number` - The employee's tax identification number
/// * - `employee_last_name` - The last name of the employee
/// * - `employee_first_name` - The first name of the employee
/// * - `work_card_movement_type` - The type of work card movement
/// * - `work_card_submission_date` - The date the work card was submitted
/// * - `work_card_movement_datetime` - The exact date and time of the work card movement
/// * - `late_declaration_justification` - The justification for the late declaration of the work card movement
#[derive(Clone)]
pub struct WorkCard {
    pub employee_tax_identification_number: String,
    pub employee_last_name: String,
    pub employee_first_name: String,
    pub work_card_movement_type: WorkCardMovementType,
    pub work_card_submission_date: NaiveDate,
    pub work_card_movement_datetime: DateTime<Utc>,
    pub late_declaration_justification: Option<LateDeclarationJustificationType>,
}

impl Serialize for WorkCard {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let formatted_reference_date = &self
            .work_card_submission_date
            .format("%Y-%m-%d")
            .to_string();

        let late_declaration_justification = match &self.late_declaration_justification {
            Some(justification) => justification.value(),
            None => "",
        };

        let work_card_movement_type = self.work_card_movement_type.value();

        let work_card_movement_datetime = format_datetime(Some(&self.work_card_movement_datetime));

        let mut work_card = serializer.serialize_struct("WorkCard", 5)?;
        work_card.serialize_field("f_afm", &self.employee_tax_identification_number)?;
        work_card.serialize_field("f_eponymo", &self.employee_last_name)?;
        work_card.serialize_field("f_onoma", &self.employee_first_name)?;
        work_card.serialize_field("f_type", work_card_movement_type)?;
        work_card.serialize_field("f_reference_date", formatted_reference_date)?;
        work_card.serialize_field("f_date", &work_card_movement_datetime)?;
        work_card.serialize_field("f_aitiologia", &late_declaration_justification)?;
        work_card.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_serialize_work_card_with_late_declaration_justification() {
        let date_time = "2014-11-28T12:00:00Z";
        let dt = date_time.parse::<DateTime<Utc>>().unwrap();

        let work_card = WorkCard {
            employee_tax_identification_number: "123456789".to_string(),
            employee_last_name: "ΠΑΠΑΔΟΠΟΥΛΟΣ".to_string(),
            employee_first_name: "ΓΕΩΡΓΙΟΣ".to_string(),
            work_card_movement_type: WorkCardMovementType::Arrival,
            work_card_submission_date: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
            work_card_movement_datetime: dt,
            late_declaration_justification: Some(LateDeclarationJustificationType::PowerOutage),
        };

        let serialized_work_card = serde_json::to_string(&work_card).unwrap();
        let expected_work_card = r#"{"f_afm":"123456789","f_eponymo":"ΠΑΠΑΔΟΠΟΥΛΟΣ","f_onoma":"ΓΕΩΡΓΙΟΣ","f_type":"0","f_reference_date":"2021-01-01","f_date":"2014-11-28T12:00:00","f_aitiologia":"001"}"#;
        assert_eq!(serialized_work_card, expected_work_card);
    }

    #[test]
    fn test_serialize_work_card_without_late_declaration_justification() {
        let date_time = "2014-11-28T12:00:00Z";
        let dt = date_time.parse::<DateTime<Utc>>().unwrap();

        let work_card = WorkCard {
            employee_tax_identification_number: "123456789".to_string(),
            employee_last_name: "ΠΑΠΑΔΟΠΟΥΛΟΣ".to_string(),
            employee_first_name: "ΓΕΩΡΓΙΟΣ".to_string(),
            work_card_movement_type: WorkCardMovementType::Arrival,
            work_card_submission_date: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
            work_card_movement_datetime: dt,
            late_declaration_justification: None,
        };

        let serialized_work_card = serde_json::to_string(&work_card).unwrap();
        let expected_work_card = r#"{"f_afm":"123456789","f_eponymo":"ΠΑΠΑΔΟΠΟΥΛΟΣ","f_onoma":"ΓΕΩΡΓΙΟΣ","f_type":"0","f_reference_date":"2021-01-01","f_date":"2014-11-28T12:00:00","f_aitiologia":""}"#;
        assert_eq!(serialized_work_card, expected_work_card);
    }
}
