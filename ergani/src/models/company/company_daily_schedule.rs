use crate::internal::utils::format_date;
use crate::models::employee::employee_daily_schedule::EmployeeDailySchedule;
use chrono::NaiveDate;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde::Serialize as TypeSerialize;

/// Represents daily schedule entries that are issued on a single business branch
/// * - `business_branch_number` - The number identifying the business branch
/// * - `start_date` -The start date of the schedule
/// * - `end_date` -The end date of the schedule period
/// * - `employee_schedules` - A list of daily schedules for employees
/// * - `related_protocol_id` - The ID of the related protocol
/// * - `related_protocol_date` - The date of the related protocol
/// * - `comments` - Additional comments regarding the daily schedule entries
pub struct CompanyDailySchedule {
    pub business_branch_number: i64,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub employee_schedules: Vec<EmployeeDailySchedule>,
    pub related_protocol_id: Option<String>,
    pub related_protocol_date: Option<NaiveDate>,
    pub comments: Option<String>,
}

#[derive(TypeSerialize)]
struct EmployeeDailySchedules {
    #[serde(rename = "ErgazomenoiWTO")]
    employee_schedules: Vec<EmployeeDailySchedule>,
}

impl Serialize for CompanyDailySchedule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let related_protocol_id = match &self.related_protocol_id {
            Some(id) => id,
            None => "",
        }
        .to_string();

        let related_protocol_date = match &self.related_protocol_date {
            Some(date) => format_date(Some(date)),
            None => "".to_string(),
        };

        let comments = match &self.comments {
            Some(comments) => comments.to_string(),
            None => "".to_string(),
        };

        let from_date = match &self.start_date {
            Some(date) => format_date(Some(date)),
            None => "".to_string(),
        };

        let to_date = match &self.end_date {
            Some(date) => format_date(Some(date)),
            None => "".to_string(),
        };

        let employee_daily_schedules = EmployeeDailySchedules {
            employee_schedules: self.employee_schedules.clone(),
        };

        let mut company_daily_schedule = serializer.serialize_struct("CompanyDailySchedule", 6)?;
        company_daily_schedule
            .serialize_field("f_aa_pararthmatos", &self.business_branch_number)?;
        company_daily_schedule.serialize_field("f_rel_protocol", &related_protocol_id)?;
        company_daily_schedule.serialize_field("f_rel_date", &related_protocol_date)?;
        company_daily_schedule.serialize_field("f_comments", &comments)?;
        company_daily_schedule.serialize_field("f_from_date", &from_date)?;
        company_daily_schedule.serialize_field("f_to_date", &to_date)?;
        company_daily_schedule.serialize_field("Ergazomenoi", &employee_daily_schedules)?;
        company_daily_schedule.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::internal::tests::load_fixture_as_text;
    use crate::models::types::schedule_work_type::ScheduleWorkType;
    use crate::models::work_day_details::WorkDayDetails;
    use chrono::{DateTime, Utc};

    #[test]
    fn test_serialize_company_daily_schedule() {
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
        let company_daily_schedule = CompanyDailySchedule {
            business_branch_number: 123,
            start_date: Some("2014-11-28".parse::<NaiveDate>().unwrap()),
            end_date: Some("2014-11-28".parse::<NaiveDate>().unwrap()),
            employee_schedules: vec![employee_daily_schedule],
            related_protocol_id: Some("123".to_string()),
            related_protocol_date: Some("2014-11-28".parse::<NaiveDate>().unwrap()),
            comments: Some("Σχόλια".to_string()),
        };

        let serialized_company_daily_schedule =
            serde_json::to_string(&company_daily_schedule).unwrap();
        let expected_text = load_fixture_as_text("company_daily_schedule_fixture.json");
        assert_eq!(serialized_company_daily_schedule, expected_text);
    }
}
