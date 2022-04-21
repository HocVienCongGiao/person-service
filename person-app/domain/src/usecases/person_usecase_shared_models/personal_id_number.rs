use crate::entities::personal_id_number::{PersonIdNumberProvider, PersonalIdNumber};
use crate::usecases::ToEntity;
use chrono::NaiveDate;
use core::option::Option;
use uuid::Uuid;

#[derive(Clone)]
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

impl ToEntity<PersonIdNumberProvider> for PersonUsecaseSharedIdNumberProvider {
    fn to_entity(self) -> PersonIdNumberProvider {
        match self {
            PersonUsecaseSharedIdNumberProvider::NationalId => PersonIdNumberProvider::NationalId,
            PersonUsecaseSharedIdNumberProvider::Passport => PersonIdNumberProvider::Passport,
        }
    }
}

impl ToEntity<PersonalIdNumber> for PersonUsecaseSharedIdNumber {
    fn to_entity(self) -> PersonalIdNumber {
        PersonalIdNumber {
            id: Some(Uuid::new_v4()),
            id_number: self.id_number,
            code: Some(self.code.unwrap().to_entity()),
            date_of_issue: self.date_of_issue,
            place_of_issue: self.place_of_issue,
        }
    }
}
