use crate::entities::educational_stage::{
    EducationalLevel as EducationalLevelEntity, EducationalLevel,
    EducationalStage as EducationalStageEntity,
};
use crate::usecases::ToEntity;
use uuid::Uuid;

// Educational Stage
pub struct PersonUsecaseSharedEducationalStage {
    pub educational_level: Option<PersonUsecaseSharedEducationalLevel>,
    pub school_name: String,
    pub major: Option<String>,
    pub graduate_year: Option<f64>,
}

impl ToEntity<EducationalStageEntity> for PersonUsecaseSharedEducationalStage {
    fn to_entity(self) -> EducationalStageEntity {
        EducationalStageEntity {
            id: Uuid::new_v4(),
            educational_level: self.educational_level.map(|level| level.to_entity()),
            school_name: Some(self.school_name),
            major: self.major,
            graduate_year: self.graduate_year,
        }
    }
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
            "ELEMENTARY_SCHOOL" => Ok(PersonUsecaseSharedEducationalLevel::ElementarySchool),
            "MIDDLE_SCHOOL" => Ok(PersonUsecaseSharedEducationalLevel::MiddleSchool),
            "HIGH_SCHOOL" => Ok(PersonUsecaseSharedEducationalLevel::HighSchool),
            "BACHELOR" => Ok(PersonUsecaseSharedEducationalLevel::Bachelor),
            "MASTER" => Ok(PersonUsecaseSharedEducationalLevel::Master),
            "DOCTOR" => Ok(PersonUsecaseSharedEducationalLevel::Doctor),
            "OTHER" => Ok(PersonUsecaseSharedEducationalLevel::Other),
            _ => Err(format!("Value not valid: {}", s)),
        }
    }
}

impl ToEntity<EducationalLevelEntity> for PersonUsecaseSharedEducationalLevel {
    fn to_entity(self) -> EducationalLevelEntity {
        match self {
            PersonUsecaseSharedEducationalLevel::ElementarySchool => {
                EducationalLevel::ElementarySchool
            }
            PersonUsecaseSharedEducationalLevel::MiddleSchool => EducationalLevel::MiddleSchool,
            PersonUsecaseSharedEducationalLevel::HighSchool => EducationalLevel::HighSchool,
            PersonUsecaseSharedEducationalLevel::Bachelor => EducationalLevel::Bachelor,
            PersonUsecaseSharedEducationalLevel::Master => EducationalLevel::Master,
            PersonUsecaseSharedEducationalLevel::Doctor => EducationalLevel::Doctor,
            PersonUsecaseSharedEducationalLevel::Other => EducationalLevel::Other,
        }
    }
}
