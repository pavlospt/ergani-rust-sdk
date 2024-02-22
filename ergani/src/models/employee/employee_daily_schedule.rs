use crate::internal::utils::format_date;
use crate::models::work_day_details::WorkDayDetails;
use chrono::NaiveDate;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde::Serialize as TypeSerialize;

/// Represents a daily schedule entry for an employee
/// * - `employee_tax_identification_number` - The employee's tax identification number
/// * - `employee_last_name` - The employee's last name
/// * - `employee_first_name` - The employee's first name
/// * - `schedule_date` - The date of the schedule
/// * - `workday_details` - A list of workday detail entries for the employee
#[derive(Clone)]
pub struct EmployeeDailySchedule {
    pub employee_tax_identification_number: String,
    pub employee_last_name: String,
    pub employee_first_name: String,
    pub schedule_date: NaiveDate,
    pub workday_details: Vec<WorkDayDetails>,
}

#[derive(TypeSerialize)]
struct EmployeeDailySchedules {
    #[serde(rename = "ErgazomenosWTOAnalytics")]
    workday_details: Vec<WorkDayDetails>,
}

impl Serialize for EmployeeDailySchedule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let formatted_schedule_date = format_date(Some(&self.schedule_date));
        let employee_daily_schedules = EmployeeDailySchedules {
            workday_details: self.workday_details.clone(),
        };

        let mut employee_daily_schedule =
            serializer.serialize_struct("EmployeeDailySchedule", 5)?;
        employee_daily_schedule
            .serialize_field("f_afm", &self.employee_tax_identification_number)?;
        employee_daily_schedule.serialize_field("f_eponymo", &self.employee_last_name)?;
        employee_daily_schedule.serialize_field("f_onoma", &self.employee_first_name)?;
        employee_daily_schedule.serialize_field("f_date", &formatted_schedule_date)?;
        employee_daily_schedule
            .serialize_field("ErgazomenosAnalytics", &employee_daily_schedules)?;
        employee_daily_schedule.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::types::schedule_work_type::ScheduleWorkType;
    use crate::tests::load_fixture_as_text;
    use chrono::{DateTime, Utc};

    #[test]
    fn test_serialize_employee_daily_schedule() {
        let workday_details = vec![
            WorkDayDetails {
                work_type: ScheduleWorkType::WorkFromHome,
                start_time: "2014-11-28T12:00:00Z".parse::<DateTime<Utc>>().unwrap(),
                end_time: "2014-11-28T16:00:00Z".parse::<DateTime<Utc>>().unwrap(),
            },
            WorkDayDetails {
                work_type: ScheduleWorkType::WorkFromOffice,
                start_time: "2014-11-28T08:00:00Z".parse::<DateTime<Utc>>().unwrap(),
                end_time: "2014-11-28T16:00:00Z".parse::<DateTime<Utc>>().unwrap(),
            },
        ];
        let employee_daily_schedule = EmployeeDailySchedule {
            employee_tax_identification_number: "123456789".to_string(),
            employee_last_name: "ΕργαζόμενοςΕπώνυμο".to_string(),
            employee_first_name: "ΕργαζόμενοςΌνομα".to_string(),
            schedule_date: "2014-11-28".parse::<NaiveDate>().unwrap(),
            workday_details,
        };

        let serialized_employee_daily_schedule =
            serde_json::to_string(&employee_daily_schedule).unwrap();
        let expected_serialized_employee_daily_schedule =
            load_fixture_as_text("employee_daily_schedule_fixture.json");
        assert_eq!(
            serialized_employee_daily_schedule,
            expected_serialized_employee_daily_schedule
        );
    }
}
