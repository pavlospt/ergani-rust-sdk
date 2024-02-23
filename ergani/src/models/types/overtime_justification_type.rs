#[allow(dead_code)]
#[derive(Clone, Default)]
#[non_exhaustive]
pub enum OvertimeJustificationType {
    AccidentPreventionOrDamageRestoration,
    #[default]
    UrgentSeasonalTasks,
    ExceptionalWorkload,
    SupplementaryTasks,
    LostHoursSuddenCauses,
    LostHoursOfficialHolidays,
    LostHoursWeatherConditions,
    EmergencyClosureDay,
    NonWorkdayTasks,
}

impl OvertimeJustificationType {
    pub fn value(&self) -> &str {
        match self {
            OvertimeJustificationType::AccidentPreventionOrDamageRestoration => "001",
            OvertimeJustificationType::UrgentSeasonalTasks => "002",
            OvertimeJustificationType::ExceptionalWorkload => "003",
            OvertimeJustificationType::SupplementaryTasks => "004",
            OvertimeJustificationType::LostHoursSuddenCauses => "005",
            OvertimeJustificationType::LostHoursOfficialHolidays => "006",
            OvertimeJustificationType::LostHoursWeatherConditions => "007",
            OvertimeJustificationType::EmergencyClosureDay => "008",
            OvertimeJustificationType::NonWorkdayTasks => "009",
        }
    }
}
