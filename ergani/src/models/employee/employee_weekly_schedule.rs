use crate::internal::utils::get_day_of_week;
use crate::models::work_day_details::WorkDayDetails;
use chrono::NaiveDate;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde::Serialize as TypeSerialize;

/// Represents a weekly schedule entry for an employee
/// * - `employee_tax_identification_number` - The employee's tax identification number
/// * - `employee_last_name` - The employee's last name
/// * - `employee_first_name` - The employee's first name
/// * - `schedule_date` - The date of the schedule
/// * - `workday_details` - A list of workday detail entries for the week
#[derive(Clone)]
pub struct EmployeeWeeklySchedule {
    pub employee_tax_identification_number: String,
    pub employee_last_name: String,
    pub employee_first_name: String,
    pub schedule_date: NaiveDate,
    pub workday_details: Vec<WorkDayDetails>,
}

#[derive(TypeSerialize)]
struct EmployeeWeeklySchedules {
    #[serde(rename = "ErgazomenosWTOAnalytics")]
    workday_details: Vec<WorkDayDetails>,
}

impl Serialize for EmployeeWeeklySchedule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let week_day = get_day_of_week(Some(self.schedule_date));

        let employee_weekly_schedules = EmployeeWeeklySchedules {
            workday_details: self.workday_details.clone(),
        };

        let mut employee_weekly_schedule =
            serializer.serialize_struct("EmployeeWeeklySchedule", 5)?;
        employee_weekly_schedule
            .serialize_field("f_afm", &self.employee_tax_identification_number)?;
        employee_weekly_schedule.serialize_field("f_eponymo", &self.employee_last_name)?;
        employee_weekly_schedule.serialize_field("f_onoma", &self.employee_first_name)?;
        employee_weekly_schedule.serialize_field("f_day", &week_day)?;
        employee_weekly_schedule
            .serialize_field("ErgazomenosAnalytics", &employee_weekly_schedules)?;
        employee_weekly_schedule.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::internal::tests::load_fixture_as_text;
    use crate::models::types::schedule_work_type::ScheduleWorkType;
    use chrono::{DateTime, Utc};

    #[test]
    fn test_serialize_employee_weekly_schedule() {
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

        let employee_weekly_schedule = EmployeeWeeklySchedule {
            employee_tax_identification_number: "123456789".to_string(),
            employee_last_name: "Παπαδόπουλος".to_string(),
            employee_first_name: "Γιάννης".to_string(),
            schedule_date: "2024-03-17".parse::<NaiveDate>().unwrap(),
            workday_details,
        };

        let serialized_employee_weekly_schedule =
            serde_json::to_string(&employee_weekly_schedule).unwrap();
        let expected_text = load_fixture_as_text("employee_weekly_schedule_fixture.json");
        assert_eq!(serialized_employee_weekly_schedule, expected_text);
    }
}
