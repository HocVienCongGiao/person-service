use crate::entities::title::{Position as PositionEntity, Title as TitleEntity};
use crate::usecases::person_usecase_shared_models::vow_progress::PersonUsecaseSharedVowProgress;
use crate::usecases::ToEntity;
use uuid::Uuid;

pub struct PersonUsecaseSharedPosition {
    pub title: Option<PersonUsecaseSharedTitle>,
    pub period: Option<PersonUsecaseSharedVowProgress>,
    pub parish: Option<Uuid>,
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

impl ToEntity<TitleEntity> for PersonUsecaseSharedTitle {
    fn to_entity(self) -> TitleEntity {
        match self {
            PersonUsecaseSharedTitle::Priest => TitleEntity::Priest,
            PersonUsecaseSharedTitle::Monk => TitleEntity::Monk,
            PersonUsecaseSharedTitle::Nun => TitleEntity::Nun,
        }
    }
}

impl ToEntity<PositionEntity> for PersonUsecaseSharedPosition {
    fn to_entity(self) -> PositionEntity {
        PositionEntity {
            title: self.title.map(|t| t.to_entity()),
            period: self.period.map(|p| p.to_entity()),
            parish: self.parish,
        }
    }
}
