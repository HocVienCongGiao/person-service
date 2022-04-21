#[derive(Clone)]
pub struct Language {
    pub language: Option<String>,
    pub level: Option<LanguageLevel>,
}

#[derive(PartialEq, Clone)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub enum LanguageLevel {
    Beginner,
    Intermediate,
    Advanced,
}

impl std::fmt::Display for LanguageLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            LanguageLevel::Beginner => write!(f, "BEGINNER"),
            LanguageLevel::Intermediate => write!(f, "INTERMEDIATE"),
            LanguageLevel::Advanced => write!(f, "ADVANCED"),
        }
    }
}

impl std::str::FromStr for LanguageLevel {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "BEGINNER" => Ok(LanguageLevel::Beginner),
            "INTERMEDIATE" => Ok(LanguageLevel::Intermediate),
            "ADVANCED" => Ok(LanguageLevel::Advanced),
            _ => Err(format!("Value not valid: {}", s)),
        }
    }
}
