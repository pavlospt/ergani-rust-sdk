use crate::internal::utils::format_time;
use crate::models::types::schedule_work_type::ScheduleWorkType;
use chrono::{DateTime, Utc};
use serde::ser::{Serialize, SerializeStruct, Serializer};

/// Represents details of an employee's workday
/// * - `work_type` - The type of an employee's work schedule
/// * - `start_time` - The start time of the workday
/// * - `end_time` - The end time of the workday
#[derive(Clone)]
pub struct WorkDayDetails {
    pub work_type: ScheduleWorkType,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}

impl Serialize for WorkDayDetails {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let formatted_start_time = format_time(&self.start_time);
        let formatted_end_time = format_time(&self.end_time);

        let mut work_day_details = serializer.serialize_struct("WorkDayDetails", 3)?;
        work_day_details.serialize_field("f_type", &self.work_type.value())?;
        work_day_details.serialize_field("f_from", &formatted_start_time)?;
        work_day_details.serialize_field("f_to", &formatted_end_time)?;
        work_day_details.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::internal::tests::load_fixture_as_text;
    use crate::models::types::schedule_work_type::ScheduleWorkType;

    #[test]
    fn test_serialize_work_day_details() {
        let start_time = "2014-11-28T12:00:00Z";
        let end_time = "2014-11-28T16:00:00Z";
        let work_day_details = WorkDayDetails {
            work_type: ScheduleWorkType::WorkFromHome,
            start_time: start_time.parse::<DateTime<Utc>>().unwrap(),
            end_time: end_time.parse::<DateTime<Utc>>().unwrap(),
        };

        let serialized_work_day_details = serde_json::to_string(&work_day_details).unwrap();
        let expected_serialized_work_day_details =
            load_fixture_as_text("work_day_details_fixture.json");
        assert_eq!(
            serialized_work_day_details,
            expected_serialized_work_day_details
        );
    }
}
