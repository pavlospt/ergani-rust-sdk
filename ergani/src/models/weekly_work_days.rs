use std::fmt::Display;

#[allow(dead_code)]
#[non_exhaustive]
#[derive(Clone, Default)]
pub enum WeeklyWorkDays {
    #[default]
    Five,
    Six,
}

impl Display for WeeklyWorkDays {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WeeklyWorkDays::Five => write!(f, "5"),
            WeeklyWorkDays::Six => write!(f, "6"),
        }
    }
}
