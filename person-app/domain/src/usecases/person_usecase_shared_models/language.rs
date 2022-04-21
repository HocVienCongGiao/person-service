use crate::entities::language::{Language as LanguageEntity, LanguageLevel as LanguageLevelEntity};
use crate::usecases::ToEntity;

pub struct PersonUsecaseSharedLanguage {
    pub language: String,
    pub level: PersonUsecaseSharedLanguageLevel,
}

impl ToEntity<LanguageEntity> for PersonUsecaseSharedLanguage {
    fn to_entity(self) -> LanguageEntity {
        LanguageEntity {
            language: Some(self.language),
            level: Some(self.level.to_entity()),
        }
    }
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

impl ToEntity<LanguageLevelEntity> for PersonUsecaseSharedLanguageLevel {
    fn to_entity(self) -> LanguageLevelEntity {
        match self {
            PersonUsecaseSharedLanguageLevel::Beginner => LanguageLevelEntity::Beginner,
            PersonUsecaseSharedLanguageLevel::Intermediate => LanguageLevelEntity::Intermediate,
            PersonUsecaseSharedLanguageLevel::Advanced => LanguageLevelEntity::Advanced,
        }
    }
}
