#![allow(dead_code)]

use crate::models::company::company_weekly_schedule::CompanyWeeklySchedule;
use crate::models::employee::employee_weekly_schedule::EmployeeWeeklySchedule;
use chrono::NaiveDate;

#[derive(Default)]
pub struct CompanyWeeklyScheduleBuilder {
    pub(crate) business_branch_number: i64,
    pub(crate) start_date: NaiveDate,
    pub(crate) end_date: NaiveDate,
    pub(crate) employee_schedules: Vec<EmployeeWeeklySchedule>,
    pub(crate) related_protocol_id: Option<String>,
    pub(crate) related_protocol_date: Option<NaiveDate>,
    pub(crate) comments: Option<String>,
}

impl CompanyWeeklyScheduleBuilder {
    pub fn builder() -> CompanyWeeklyScheduleBuilder {
        CompanyWeeklyScheduleBuilder::default()
    }

    pub fn build(self) -> CompanyWeeklySchedule {
        CompanyWeeklySchedule {
            business_branch_number: self.business_branch_number,
            start_date: self.start_date,
            end_date: self.end_date,
            employee_schedules: self.employee_schedules,
            related_protocol_id: self.related_protocol_id,
            related_protocol_date: self.related_protocol_date,
            comments: self.comments,
        }
    }

    pub fn set_business_branch_number(mut self, business_branch_number: i64) -> Self {
        self.business_branch_number = business_branch_number;
        self
    }
    pub fn set_start_date(mut self, start_date: NaiveDate) -> Self {
        self.start_date = start_date;
        self
    }
    pub fn set_end_date(mut self, end_date: NaiveDate) -> Self {
        self.end_date = end_date;
        self
    }
    pub fn set_employee_schedules(
        mut self,
        employee_schedules: Vec<EmployeeWeeklySchedule>,
    ) -> Self {
        self.employee_schedules = employee_schedules;
        self
    }
    pub fn set_related_protocol_id(
        mut self,
        related_protocol_id: Option<impl Into<String>>,
    ) -> Self {
        self.related_protocol_id = related_protocol_id.map(|s| s.into());
        self
    }
    pub fn set_related_protocol_date(mut self, related_protocol_date: Option<NaiveDate>) -> Self {
        self.related_protocol_date = related_protocol_date;
        self
    }
    pub fn set_comments(mut self, comments: Option<impl Into<String>>) -> Self {
        self.comments = comments.map(|s| s.into());
        self
    }
}
