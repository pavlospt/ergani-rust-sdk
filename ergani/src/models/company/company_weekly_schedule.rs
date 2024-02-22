use crate::models::employee::employee_weekly_schedule::EmployeeWeeklySchedule;
use chrono::NaiveDate;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde::Serialize as TypeSerialize;

/// Represents weekly schedule entries that are issued on a single business branch
/// * - `business_branch_number` - The number identifying the business branch
/// * - `start_date` - The start date of the weekly schedule
/// * - `end_date` - The end date of the weekly schedule
/// * - `employee_schedules` - A list of weekly schedules for employees
/// * - `related_protocol_id` - The ID of the related protocol
/// * - `related_protocol_date` - The date of the related protocol
/// * - `comments` - Additional comments regarding the weekly schedule entries
pub struct CompanyWeeklySchedule {
    pub business_branch_number: i64,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub employee_schedules: Vec<EmployeeWeeklySchedule>,
    pub related_protocol_id: Option<String>,
    pub related_protocol_date: Option<NaiveDate>,
    pub comments: Option<String>,
}

#[derive(TypeSerialize)]
struct EmployeeWeeklySchedules {
    #[serde(rename = "ErgazomenoiWTO")]
    employee_schedules: Vec<EmployeeWeeklySchedule>,
}

impl Serialize for CompanyWeeklySchedule {
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
            Some(date) => date.to_string(),
            None => "".to_string(),
        };

        let comments = match &self.comments {
            Some(comments) => comments.to_string(),
            None => "".to_string(),
        };

        let from_date = self.start_date.to_string();
        let to_date = self.end_date.to_string();

        let employee_weekly_schedules = EmployeeWeeklySchedules {
            employee_schedules: self.employee_schedules.clone(),
        };

        let mut company_weekly_schedule =
            serializer.serialize_struct("CompanyWeeklySchedule", 6)?;
        company_weekly_schedule
            .serialize_field("f_aa_pararthmatos", &self.business_branch_number)?;
        company_weekly_schedule.serialize_field("f_rel_protocol", &related_protocol_id)?;
        company_weekly_schedule.serialize_field("f_rel_date", &related_protocol_date)?;
        company_weekly_schedule.serialize_field("f_comments", &comments)?;
        company_weekly_schedule.serialize_field("f_from_date", &from_date)?;
        company_weekly_schedule.serialize_field("f_to_date", &to_date)?;
        company_weekly_schedule.serialize_field("Ergazomenoi", &employee_weekly_schedules)?;
        company_weekly_schedule.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::types::schedule_work_type::ScheduleWorkType;
    use crate::models::work_day_details::WorkDayDetails;
    use crate::tests::load_fixture_as_text;
    use chrono::{DateTime, Utc};

    #[test]
    fn test_serialize_company_weekly_schedule() {
        let employee_schedules = vec![
            EmployeeWeeklySchedule {
                employee_tax_identification_number: "123456789".to_string(),
                employee_last_name: "Doe".to_string(),
                employee_first_name: "John".to_string(),
                schedule_date: "2014-11-28".parse::<NaiveDate>().unwrap(),
                workday_details: vec![
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
                ],
            },
            EmployeeWeeklySchedule {
                employee_tax_identification_number: "987654321".to_string(),
                employee_last_name: "Doe".to_string(),
                employee_first_name: "Jane".to_string(),
                schedule_date: "2014-11-28".parse::<NaiveDate>().unwrap(),
                workday_details: vec![
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
                ],
            },
        ];

        let company_weekly_schedule = CompanyWeeklySchedule {
            business_branch_number: 1,
            start_date: "2014-11-28".parse::<NaiveDate>().unwrap(),
            end_date: "2014-11-28".parse::<NaiveDate>().unwrap(),
            employee_schedules,
            related_protocol_id: Some("123456".to_string()),
            related_protocol_date: Some("2014-11-28".parse::<NaiveDate>().unwrap()),
            comments: Some("Some comments".to_string()),
        };

        let serialized = serde_json::to_string(&company_weekly_schedule).unwrap();
        let expected = load_fixture_as_text("company_weekly_schedule_fixture.json");
        assert_eq!(serialized, expected);
    }
}
