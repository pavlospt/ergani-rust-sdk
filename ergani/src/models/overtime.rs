use crate::internal::utils::{format_date, format_time, get_ergani_overtime_cancellation};
use crate::models::types::overtime_justification_type::OvertimeJustificationType;
use crate::models::weekly_work_days::WeeklyWorkDays;
use chrono::{DateTime, NaiveDate, Utc};
use serde::ser::{Serialize, SerializeStruct, Serializer};

/// Represents an overtime entry for an employee
/// * - `employee_tax_identification_number` - The employee's tax identification number
/// * - `employee_social_security_number` - The employee's social security number
/// * - `employee_last_name` - The last name of the employee
/// * - `employee_first_name` - The first name of the employee
/// * - `overtime_date` - The date of the overtime
/// * - `overtime_start_time` - The start time of the overtime period
/// * - `overtime_end_time` - The end time of the overtime period
/// * - `overtime_cancellation` - Indicates if the overtime was cancelled or not
/// * - `employee_profession_code` - The profession code of the employee
/// * - `overtime_justification` - The justification for the overtime
/// * - `weekly_workdays_number` - The number of the employee's working days in a week
/// * - `asee_approval` - The ASEE aproval
#[derive(Clone)]
pub struct Overtime {
    pub employee_tax_identification_number: String,
    pub employee_social_security_number: String,
    pub employee_last_name: String,
    pub employee_first_name: String,
    pub overtime_date: NaiveDate,
    pub overtime_start_time: DateTime<Utc>,
    pub overtime_end_time: DateTime<Utc>,
    pub overtime_cancellation: bool,
    pub employee_profession_code: String,
    pub overtime_justification: OvertimeJustificationType,
    pub weekly_workdays_number: WeeklyWorkDays,
    pub asee_approval: Option<String>,
}

impl Serialize for Overtime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let overtime_cancellation = get_ergani_overtime_cancellation(self.overtime_cancellation);
        let mut overtime = serializer.serialize_struct("Overtime", 11)?;
        overtime.serialize_field("f_afm", &self.employee_tax_identification_number)?;
        overtime.serialize_field("f_amka", &self.employee_social_security_number)?;
        overtime.serialize_field("f_eponymo", &self.employee_last_name)?;
        overtime.serialize_field("f_onoma", &self.employee_first_name)?;
        overtime.serialize_field("f_date", &format_date(Some(&self.overtime_date)))?;
        overtime.serialize_field("f_from", &format_time(&self.overtime_start_time))?;
        overtime.serialize_field("f_to", &format_time(&self.overtime_end_time))?;
        overtime.serialize_field("f_cancellation", &overtime_cancellation)?;
        overtime.serialize_field("f_step", &self.employee_profession_code)?;
        overtime.serialize_field("f_reason", &self.overtime_justification.value())?;
        overtime.serialize_field("f_weekdates", &self.weekly_workdays_number.to_string())?;
        overtime.serialize_field("f_asee", &self.asee_approval)?;
        overtime.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::load_fixture_as_text;

    #[test]
    fn test_serialize_overtime() {
        let date_time_start_text = "2014-11-28T12:00:09Z";
        let date_time_end_text = "2014-11-29T12:00:09Z";

        let dt_start = date_time_start_text.parse::<DateTime<Utc>>().unwrap();
        let dt_end = date_time_end_text.parse::<DateTime<Utc>>().unwrap();

        let overtime = Overtime {
            employee_tax_identification_number: "123456789".to_string(),
            employee_social_security_number: "12345678901".to_string(),
            employee_last_name: "ΠΑΠΑΔΟΠΟΥΛΟΣ".to_string(),
            employee_first_name: "ΓΕΩΡΓΙΟΣ".to_string(),
            overtime_date: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
            overtime_start_time: dt_start,
            overtime_end_time: dt_end,
            overtime_cancellation: false,
            employee_profession_code: "1234".to_string(),
            overtime_justification:
                OvertimeJustificationType::AccidentPreventionOrDamageRestoration,
            weekly_workdays_number: WeeklyWorkDays::Five,
            asee_approval: Some("123456789".to_string()),
        };

        let serialized = serde_json::to_string(&overtime).unwrap();
        let expected = load_fixture_as_text("overtime_fixture.json");
        assert_eq!(serialized, expected);
    }
}
