use uuid::Uuid;

#[derive(Clone)]
pub struct EducationalStage {
    pub id: Uuid,
    pub educational_level: Option<EducationalLevel>,
    pub school_name: Option<String>,
    pub major: Option<String>,
    pub graduate_year: Option<f64>,
}

#[derive(PartialEq, Clone)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub enum EducationalLevel {
    ElementarySchool,
    MiddleSchool,
    HighSchool,
    Bachelor,
    Master,
    Doctor,
    Other,
}

impl std::fmt::Display for EducationalLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            EducationalLevel::ElementarySchool => write!(f, "ELEMENTARY_SCHOOL"),
            EducationalLevel::MiddleSchool => write!(f, "MIDDLE_SCHOOL"),
            EducationalLevel::HighSchool => write!(f, "HIGH_SCHOOL"),
            EducationalLevel::Bachelor => write!(f, "BACHELOR"),
            EducationalLevel::Master => write!(f, "MASTER"),
            EducationalLevel::Doctor => write!(f, "DOCTOR"),
            EducationalLevel::Other => write!(f, "OTHER"),
        }
    }
}

impl std::str::FromStr for EducationalLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "ELEMENTARY_SCHOOL" => Ok(EducationalLevel::ElementarySchool),
            "MIDDLE_SCHOOL" => Ok(EducationalLevel::MiddleSchool),
            "HIGH_SCHOOL" => Ok(EducationalLevel::HighSchool),
            "BACHELOR" => Ok(EducationalLevel::Bachelor),
            "MASTER" => Ok(EducationalLevel::Master),
            "DOCTOR" => Ok(EducationalLevel::Doctor),
            "OTHER" => Ok(EducationalLevel::Other),
            _ => Err(format!("Value not valid: {}", s)),
        }
    }
}
