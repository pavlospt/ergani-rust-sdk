#![allow(dead_code)]

use crate::models::employee::employee_daily_schedule::EmployeeDailySchedule;
use crate::models::work_day_details::WorkDayDetails;
use chrono::NaiveDate;

#[derive(Default)]
pub struct EmployeeDailyScheduleBuilder {
    pub(crate) employee_tax_identification_number: String,
    pub(crate) employee_last_name: String,
    pub(crate) employee_first_name: String,
    pub(crate) schedule_date: NaiveDate,
    pub(crate) workday_details: Vec<WorkDayDetails>,
}

impl EmployeeDailyScheduleBuilder {
    pub fn builder() -> EmployeeDailyScheduleBuilder {
        EmployeeDailyScheduleBuilder::default()
    }

    pub fn build(self) -> EmployeeDailySchedule {
        EmployeeDailySchedule {
            employee_tax_identification_number: self.employee_tax_identification_number,
            employee_last_name: self.employee_last_name,
            employee_first_name: self.employee_first_name,
            schedule_date: self.schedule_date,
            workday_details: self.workday_details,
        }
    }

    pub fn set_employee_tax_identification_number(
        mut self,
        employee_tax_identification_number: impl Into<String>,
    ) -> Self {
        self.employee_tax_identification_number = employee_tax_identification_number.into();
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
    pub fn set_schedule_date(mut self, schedule_date: NaiveDate) -> Self {
        self.schedule_date = schedule_date;
        self
    }
    pub fn set_workday_details(mut self, workday_details: Vec<WorkDayDetails>) -> Self {
        self.workday_details = workday_details;
        self
    }
}
