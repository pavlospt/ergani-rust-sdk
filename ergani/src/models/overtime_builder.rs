#![allow(dead_code)]

use crate::models::overtime::Overtime;
use crate::models::types::overtime_justification_type::OvertimeJustificationType;
use crate::models::weekly_work_days::WeeklyWorkDays;
use anyhow::{bail, Result};
use chrono::{DateTime, NaiveDate, Utc};

#[derive(Default)]
pub struct OvertimeBuilder {
    pub(crate) employee_tax_identification_number: String,
    pub(crate) employee_social_security_number: String,
    pub(crate) employee_last_name: String,
    pub(crate) employee_first_name: String,
    pub(crate) overtime_date: NaiveDate,
    pub(crate) overtime_start_time: DateTime<Utc>,
    pub(crate) overtime_end_time: DateTime<Utc>,
    pub(crate) overtime_cancellation: bool,
    pub(crate) employee_profession_code: String,
    pub(crate) overtime_justification: Option<OvertimeJustificationType>,
    pub(crate) weekly_workdays_number: WeeklyWorkDays,
    pub(crate) asee_approval: Option<String>,
}

impl OvertimeBuilder {
    pub fn builder() -> OvertimeBuilder {
        OvertimeBuilder::default()
    }

    pub fn build(self) -> Result<Overtime> {
        let overtime_justification = match self.overtime_justification {
            Some(ot) => ot,
            None => bail!("Overtime justification is required"),
        };

        Ok(Overtime {
            employee_tax_identification_number: self.employee_tax_identification_number,
            employee_social_security_number: self.employee_social_security_number,
            employee_last_name: self.employee_last_name,
            employee_first_name: self.employee_first_name,
            overtime_date: self.overtime_date,
            overtime_start_time: self.overtime_start_time,
            overtime_end_time: self.overtime_end_time,
            overtime_cancellation: self.overtime_cancellation,
            employee_profession_code: self.employee_profession_code,
            weekly_workdays_number: self.weekly_workdays_number,
            asee_approval: self.asee_approval,
            overtime_justification,
        })
    }

    pub fn set_employee_tax_identification_number(
        mut self,
        employee_tax_identification_number: impl Into<String>,
    ) -> Self {
        self.employee_tax_identification_number = employee_tax_identification_number.into();
        self
    }
    pub fn set_employee_social_security_number(
        mut self,
        employee_social_security_number: impl Into<String>,
    ) -> Self {
        self.employee_social_security_number = employee_social_security_number.into();
        self
    }
    pub fn set_employee_last_name(mut self, employee_last_name: impl Into<String>) -> Self {
        self.employee_last_name = employee_last_name.into();
        self
    }
    pub fn set_employee_first_name(mut self, employee_first_name: impl Into<String>) -> Self {
        self.employee_first_name = employee_first_name.into();
        self
    }
    pub fn set_overtime_date(mut self, overtime_date: NaiveDate) -> Self {
        self.overtime_date = overtime_date;
        self
    }
    pub fn set_overtime_start_time(mut self, overtime_start_time: DateTime<Utc>) -> Self {
        self.overtime_start_time = overtime_start_time;
        self
    }
    pub fn set_overtime_end_time(mut self, overtime_end_time: DateTime<Utc>) -> Self {
        self.overtime_end_time = overtime_end_time;
        self
    }
    pub fn set_overtime_cancellation(mut self, overtime_cancellation: bool) -> Self {
        self.overtime_cancellation = overtime_cancellation;
        self
    }
    pub fn set_employee_profession_code(
        mut self,
        employee_profession_code: impl Into<String>,
    ) -> Self {
        self.employee_profession_code = employee_profession_code.into();
        self
    }
    pub fn set_overtime_justification(
        mut self,
        overtime_justification: OvertimeJustificationType,
    ) -> Self {
        self.overtime_justification = Some(overtime_justification);
        self
    }
    pub fn set_weekly_workdays_number(mut self, weekly_workdays_number: WeeklyWorkDays) -> Self {
        self.weekly_workdays_number = weekly_workdays_number;
        self
    }
    pub fn set_asee_approval(mut self, asee_approval: Option<impl Into<String>>) -> Self {
        self.asee_approval = asee_approval.map(|a| a.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::types::overtime_justification_type::OvertimeJustificationType;
    use chrono::{DateTime, NaiveDate, Utc};

    #[test]
    fn test_build_overtime_fails_when_overtime_justification_not_set() {
        let date_time_start_text = "2014-11-28T12:00:09Z";
        let date_time_end_text = "2014-11-29T12:00:09Z";

        let dt_start = date_time_start_text.parse::<DateTime<Utc>>().unwrap();
        let dt_end = date_time_end_text.parse::<DateTime<Utc>>().unwrap();

        let overtime = OvertimeBuilder::builder()
            .set_employee_tax_identification_number("123456789")
            .set_employee_social_security_number("12345678901")
            .set_employee_last_name("ΠΑΠΑΔΟΠΟΥΛΟΣ")
            .set_employee_first_name("ΓΕΩΡΓΙΟΣ")
            .set_overtime_date(NaiveDate::from_ymd_opt(2021, 1, 1).unwrap())
            .set_overtime_start_time(dt_start)
            .set_overtime_end_time(dt_end)
            .set_overtime_cancellation(false)
            .set_employee_profession_code("1234")
            .set_weekly_workdays_number(WeeklyWorkDays::Five)
            .set_asee_approval(Some("123456"))
            .build();

        assert!(overtime.is_err());
    }

    #[test]
    fn test_build_overtime_is_ok() {
        let date_time_start_text = "2014-11-28T12:00:09Z";
        let date_time_end_text = "2014-11-29T12:00:09Z";

        let dt_start = date_time_start_text.parse::<DateTime<Utc>>().unwrap();
        let dt_end = date_time_end_text.parse::<DateTime<Utc>>().unwrap();

        let overtime = OvertimeBuilder::builder()
            .set_employee_tax_identification_number("123456789")
            .set_employee_social_security_number("12345678901")
            .set_employee_last_name("ΠΑΠΑΔΟΠΟΥΛΟΣ")
            .set_employee_first_name("ΓΕΩΡΓΙΟΣ")
            .set_overtime_date(NaiveDate::from_ymd_opt(2021, 1, 1).unwrap())
            .set_overtime_start_time(dt_start)
            .set_overtime_end_time(dt_end)
            .set_overtime_cancellation(false)
            .set_employee_profession_code("1234")
            .set_weekly_workdays_number(WeeklyWorkDays::Five)
            .set_asee_approval(Some("123456"))
            .set_overtime_justification(
                OvertimeJustificationType::AccidentPreventionOrDamageRestoration,
            )
            .build();

        assert!(overtime.is_ok());
    }
}
