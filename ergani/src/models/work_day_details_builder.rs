#![allow(dead_code)]

use crate::models::types::schedule_work_type::ScheduleWorkType;
use crate::models::work_day_details::WorkDayDetails;
use anyhow::{bail, Result};
use chrono::{DateTime, Utc};

#[derive(Default)]
pub struct WorkDayDetailsBuilder {
    pub(crate) work_type: Option<ScheduleWorkType>,
    pub(crate) start_time: DateTime<Utc>,
    pub(crate) end_time: DateTime<Utc>,
}

impl WorkDayDetailsBuilder {
    pub fn builder() -> WorkDayDetailsBuilder {
        WorkDayDetailsBuilder::default()
    }

    pub fn build(self) -> Result<WorkDayDetails> {
        let work_type = match self.work_type {
            Some(wt) => wt,
            None => bail!("Work type is required"),
        };

        Ok(WorkDayDetails {
            start_time: self.start_time,
            end_time: self.end_time,
            work_type,
        })
    }

    pub fn set_work_type(mut self, work_type: ScheduleWorkType) -> Self {
        self.work_type = Some(work_type);
        self
    }

    pub fn set_start_time(mut self, start_time: DateTime<Utc>) -> Self {
        self.start_time = start_time;
        self
    }

    pub fn set_end_time(mut self, end_time: DateTime<Utc>) -> Self {
        self.end_time = end_time;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_work_day_details_build_fails_when_work_type_not_set() {
        let start_time = "2014-11-28T12:00:00Z";
        let end_time = "2014-11-28T16:00:00Z";
        let work_day_details = WorkDayDetailsBuilder::builder()
            .set_start_time(start_time.parse::<DateTime<Utc>>().unwrap())
            .set_end_time(end_time.parse::<DateTime<Utc>>().unwrap())
            .build();

        assert!(work_day_details.is_err());
        assert_eq!(
            work_day_details.err().unwrap().to_string(),
            "Work type is required"
        );
    }

    #[test]
    fn test_serialize_work_day_details_build_is_ok() {
        let start_time = "2014-11-28T12:00:00Z";
        let end_time = "2014-11-28T16:00:00Z";
        let work_day_details = WorkDayDetailsBuilder::builder()
            .set_work_type(ScheduleWorkType::WorkFromHome)
            .set_start_time(start_time.parse::<DateTime<Utc>>().unwrap())
            .set_end_time(end_time.parse::<DateTime<Utc>>().unwrap())
            .build();

        assert!(work_day_details.is_ok());
    }
}
