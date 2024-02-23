#[allow(dead_code)]
#[non_exhaustive]
#[derive(Clone)]
pub enum ScheduleWorkType {
    WorkFromOffice,
    WorkFromHome,
    RestDay,
    Absent,
}

impl ScheduleWorkType {
    pub fn value(&self) -> &str {
        match self {
            ScheduleWorkType::WorkFromOffice => "ΕΡΓ",
            ScheduleWorkType::WorkFromHome => "ΤΗΛ",
            ScheduleWorkType::RestDay => "ΑΝ",
            ScheduleWorkType::Absent => "ΜΕ",
        }
    }
}
