#[derive(Clone)]
#[allow(dead_code)]
#[non_exhaustive]
pub enum WorkCardMovementType {
    Arrival,
    Departure,
}

impl WorkCardMovementType {
    pub fn value(&self) -> &str {
        match self {
            WorkCardMovementType::Arrival => "0",
            WorkCardMovementType::Departure => "1",
        }
    }
}
