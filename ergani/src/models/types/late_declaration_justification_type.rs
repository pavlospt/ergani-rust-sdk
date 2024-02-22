#[derive(Clone)]
#[allow(dead_code)]
#[non_exhaustive]
pub enum LateDeclarationJustificationType {
    PowerOutage,
    EmployerSystemsUnavailable,
    ErganiSystemsUnavailable,
}

impl LateDeclarationJustificationType {
    pub fn value(&self) -> &str {
        match self {
            LateDeclarationJustificationType::PowerOutage => "001",
            LateDeclarationJustificationType::EmployerSystemsUnavailable => "002",
            LateDeclarationJustificationType::ErganiSystemsUnavailable => "003",
        }
    }
}
