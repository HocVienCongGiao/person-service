use crate::entities::vow_progress::VowProgress as VowProgressEntity;
use crate::usecases::ToEntity;

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
            "SOLEMN_VOW" => Ok(PersonUsecaseSharedVowProgress::SolemnVow),
            "SIMPLE_VOW" => Ok(PersonUsecaseSharedVowProgress::SimpleVow),
            "NOVICE" => Ok(PersonUsecaseSharedVowProgress::Novice),
            "PREPARATION" => Ok(PersonUsecaseSharedVowProgress::Preparation),
            _ => Err(format!("Value not valid: {}", s)),
        }
    }
}

impl ToEntity<VowProgressEntity> for PersonUsecaseSharedVowProgress {
    fn to_entity(self) -> VowProgressEntity {
        match self {
            PersonUsecaseSharedVowProgress::SolemnVow => VowProgressEntity::SolemnVow,
            PersonUsecaseSharedVowProgress::SimpleVow => VowProgressEntity::SimpleVow,
            PersonUsecaseSharedVowProgress::Novice => VowProgressEntity::Novice,
            PersonUsecaseSharedVowProgress::Preparation => VowProgressEntity::Preparation,
        }
    }
}
