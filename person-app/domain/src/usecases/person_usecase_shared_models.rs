use chrono::NaiveDate;
use uuid::Uuid;

pub(crate) trait WithPolity<T> {
    fn with_polity(
        self,
        name: Option<String>,
        location_name: Option<String>,
        location_address: Option<String>,
        location_email: Option<String>,
    ) -> T;
}

pub(crate) trait WithChristianName<T> {
    fn with_christian_name(self, name: Option<String>) -> T;
}

pub(crate) trait WithPersonId<T> {
    fn with_person_id(self, person_id: Option<Uuid>) -> T;
}

#[derive(PartialEq, Clone)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub enum PersonUsecaseSharedTitle {
    Priest,
    Monk,
    Nun,
}

impl std::str::FromStr for PersonUsecaseSharedTitle {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "PRIEST" => std::result::Result::Ok(PersonUsecaseSharedTitle::Priest),
            "MONK" => std::result::Result::Ok(PersonUsecaseSharedTitle::Monk),
            "NUN" => std::result::Result::Ok(PersonUsecaseSharedTitle::Nun),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

#[derive(PartialEq, Clone)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub enum PersonUsecaseSharedNationality {
    Vietnamese,
    Chinese,
    British,
    American,
    French,
}

impl std::str::FromStr for PersonUsecaseSharedNationality {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "VIETNAMESE" => std::result::Result::Ok(PersonUsecaseSharedNationality::Vietnamese),
            "CHINESE" => std::result::Result::Ok(PersonUsecaseSharedNationality::Chinese),
            "BRITISH" => std::result::Result::Ok(PersonUsecaseSharedNationality::British),
            "AMERICAN" => std::result::Result::Ok(PersonUsecaseSharedNationality::American),
            "FRECH" => std::result::Result::Ok(PersonUsecaseSharedNationality::French),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

pub struct PersonUsecaseSharedIdNumber {
    pub id_number: Option<String>,
    pub code: Option<PersonUsecaseSharedIdNumberProvider>,
    pub date_of_issue: Option<NaiveDate>,
    pub place_of_issue: Option<String>,
}

#[derive(PartialEq, Clone)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub enum PersonUsecaseSharedIdNumberProvider {
    NationalId,
    Passport,
}

impl std::str::FromStr for PersonUsecaseSharedIdNumberProvider {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "NATIONAL_ID" => {
                std::result::Result::Ok(PersonUsecaseSharedIdNumberProvider::NationalId)
            }
            "PASSPORT" => std::result::Result::Ok(PersonUsecaseSharedIdNumberProvider::Passport),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

#[derive(PartialEq, Clone)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub enum PersonUsecaseSharedVowProgress {
    SolemnVow,
    SimpleVow,
    Novice,
    Preparation,
}

impl std::str::FromStr for PersonUsecaseSharedVowProgress {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "SOLEMN_VOW" => std::result::Result::Ok(PersonUsecaseSharedVowProgress::SolemnVow),
            "SIMPLE_VOW" => std::result::Result::Ok(PersonUsecaseSharedVowProgress::SimpleVow),
            "NOVICE" => std::result::Result::Ok(PersonUsecaseSharedVowProgress::Novice),
            "PREPARATION" => std::result::Result::Ok(PersonUsecaseSharedVowProgress::Preparation),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

pub struct PersonUsecaseSharedLanguage {
    pub language: String,
    pub level: PersonUsecaseSharedLanguageLevel,
}

#[derive(PartialEq, Clone)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub enum PersonUsecaseSharedLanguageLevel {
    Beginner,
    Intermediate,
    Advanced,
}

impl std::fmt::Display for PersonUsecaseSharedLanguageLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            PersonUsecaseSharedLanguageLevel::Beginner => write!(f, "BEGINNER"),
            PersonUsecaseSharedLanguageLevel::Intermediate => write!(f, "INTERMEDIATE"),
            PersonUsecaseSharedLanguageLevel::Advanced => write!(f, "ADVANCED"),
        }
    }
}

// Educational Stage
pub struct PersonUsecaseSharedEducationalStage {
    pub educational_level: Option<PersonUsecaseSharedEducationalLevel>,
    pub school_name: String,
    pub major: Option<String>,
    pub graduate_year: Option<f64>,
}

#[derive(PartialEq, Clone)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub enum PersonUsecaseSharedEducationalLevel {
    ElementarySchool,
    MiddleSchool,
    HighSchool,
    Bachelor,
    Master,
    Doctor,
    Other,
}

impl std::str::FromStr for PersonUsecaseSharedEducationalLevel {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "ELEMENTARY_SCHOOL" => {
                std::result::Result::Ok(PersonUsecaseSharedEducationalLevel::ElementarySchool)
            }
            "MIDDLE_SCHOOL" => {
                std::result::Result::Ok(PersonUsecaseSharedEducationalLevel::MiddleSchool)
            }
            "HIGH_SCHOOL" => {
                std::result::Result::Ok(PersonUsecaseSharedEducationalLevel::HighSchool)
            }
            "BACHELOR" => std::result::Result::Ok(PersonUsecaseSharedEducationalLevel::Bachelor),
            "MASTER" => std::result::Result::Ok(PersonUsecaseSharedEducationalLevel::Master),
            "DOCTOR" => std::result::Result::Ok(PersonUsecaseSharedEducationalLevel::Doctor),
            "OTHER" => std::result::Result::Ok(PersonUsecaseSharedEducationalLevel::Other),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

pub struct QueryPersonUsecaseOutput {
    pub id: Uuid,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub place_of_birth: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    // todo()!
    // pub personal_id_numbers: Option<Vec<>>,
}
