use crate::entities::personal_id_number::PersonalIdNumber;
use chrono::NaiveDate;
use uuid::Uuid;

#[derive(Clone)]
pub(crate) struct Person {
    pub id: Option<Uuid>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub place_of_birth: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub nationality: Option<Nationality>,
    pub race: Option<String>,
    pub personal_id_number: Option<Vec<PersonalIdNumber>>,
    pub address: Option<String>,
}

impl Person {
    pub(crate) fn is_valid(&self) -> bool {
        true
    }
}

#[derive(PartialEq, Clone)]
#[repr(C)]
pub(crate) enum Nationality {
    Vietnamese,
    Chinese,
    British,
    American,
    French,
}

impl std::fmt::Display for Nationality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Nationality::Vietnamese => write!(f, "VIETNAMESE"),
            Nationality::Chinese => write!(f, "CHINESE"),
            Nationality::British => write!(f, "BRITISH"),
            Nationality::American => write!(f, "AMERICAN"),
            Nationality::French => write!(f, "FRENCH"),
        }
    }
}

impl std::str::FromStr for Nationality {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "VIETNAMESE" => std::result::Result::Ok(Nationality::Vietnamese),
            "CHINESE" => std::result::Result::Ok(Nationality::Chinese),
            "BRITISH" => std::result::Result::Ok(Nationality::British),
            "AMERICAN" => std::result::Result::Ok(Nationality::American),
            "FRENCH" => std::result::Result::Ok(Nationality::French),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}
