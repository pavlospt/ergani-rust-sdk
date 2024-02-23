#![allow(dead_code)]

use crate::models::types::late_declaration_justification_type::LateDeclarationJustificationType;
use crate::models::types::work_card_movement_type::WorkCardMovementType;
use crate::models::work_card::WorkCard;
use anyhow::{bail, Result};
use chrono::{DateTime, NaiveDate, Utc};

#[derive(Default)]
pub struct WorkCardBuilder {
    pub(crate) employee_tax_identification_number: String,
    pub(crate) employee_last_name: String,
    pub(crate) employee_first_name: String,
    pub(crate) work_card_movement_type: Option<WorkCardMovementType>,
    pub(crate) work_card_submission_date: NaiveDate,
    pub(crate) work_card_movement_datetime: DateTime<Utc>,
    pub(crate) late_declaration_justification: Option<LateDeclarationJustificationType>,
}

impl WorkCardBuilder {
    pub fn builder() -> WorkCardBuilder {
        WorkCardBuilder::default()
    }

    pub fn build(self) -> Result<WorkCard> {
        let work_card_movement_type = match self.work_card_movement_type {
            Some(wcmt) => wcmt,
            None => bail!("Work card movement type is required"),
        };

        Ok(WorkCard {
            employee_tax_identification_number: self.employee_tax_identification_number,
            employee_last_name: self.employee_last_name,
            employee_first_name: self.employee_first_name,
            work_card_submission_date: self.work_card_submission_date,
            work_card_movement_datetime: self.work_card_movement_datetime,
            late_declaration_justification: self.late_declaration_justification,
            work_card_movement_type,
        })
    }

    pub fn set_employee_tax_identification_number(
        mut self,
        employee_tax_identification_number: String,
    ) -> Self {
        self.employee_tax_identification_number = employee_tax_identification_number;
        self
    }

    pub fn set_employee_last_name(mut self, employee_last_name: String) -> Self {
        self.employee_last_name = employee_last_name;
        self
    }
    pub fn set_employee_first_name(mut self, employee_first_name: String) -> Self {
        self.employee_first_name = employee_first_name;
        self
    }

    pub fn set_work_card_movement_type(
        mut self,
        work_card_movement_type: WorkCardMovementType,
    ) -> Self {
        self.work_card_movement_type = Some(work_card_movement_type);
        self
    }
    pub fn set_work_card_submission_date(mut self, work_card_submission_date: NaiveDate) -> Self {
        self.work_card_submission_date = work_card_submission_date;
        self
    }
    pub fn set_work_card_movement_datetime(
        mut self,
        work_card_movement_datetime: DateTime<Utc>,
    ) -> Self {
        self.work_card_movement_datetime = work_card_movement_datetime;
        self
    }
    pub fn set_late_declaration_justification(
        mut self,
        late_declaration_justification: Option<LateDeclarationJustificationType>,
    ) -> Self {
        self.late_declaration_justification = late_declaration_justification;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::types::late_declaration_justification_type::LateDeclarationJustificationType;
    use chrono::NaiveDate;

    #[test]
    fn test_build_work_card_fails_when_work_card_movement_type_not_set() {
        let date_time = "2014-11-28T12:00:00Z";
        let dt = date_time.parse::<DateTime<Utc>>().unwrap();

        let work_card = WorkCardBuilder::builder()
            .set_employee_tax_identification_number("123456789".to_string())
            .set_employee_last_name("ΠΑΠΑΔΟΠΟΥΛΟΣ".to_string())
            .set_employee_first_name("ΓΕΩΡΓΙΟΣ".to_string())
            .set_work_card_submission_date(NaiveDate::from_ymd_opt(2014, 11, 28).unwrap())
            .set_work_card_movement_datetime(dt)
            .set_late_declaration_justification(Some(LateDeclarationJustificationType::PowerOutage))
            .build();

        assert!(work_card.is_err());
    }

    #[test]
    fn test_build_work_card_is_ok() {
        let date_time = "2014-11-28T12:00:00Z";
        let dt = date_time.parse::<DateTime<Utc>>().unwrap();

        let work_card = WorkCardBuilder::builder()
            .set_employee_tax_identification_number("123456789".to_string())
            .set_employee_last_name("ΠΑΠΑΔΟΠΟΥΛΟΣ".to_string())
            .set_employee_first_name("ΓΕΩΡΓΙΟΣ".to_string())
            .set_work_card_submission_date(NaiveDate::from_ymd_opt(2014, 11, 28).unwrap())
            .set_work_card_movement_datetime(dt)
            .set_late_declaration_justification(Some(LateDeclarationJustificationType::PowerOutage))
            .set_work_card_movement_type(WorkCardMovementType::Arrival)
            .build();

        assert!(work_card.is_ok());
    }
}
