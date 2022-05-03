use crate::entities::educational_stage::{
    EducationalLevel as EducationalLevelEntity, EducationalLevel,
    EducationalStage as EducationalStageEntity,
};
use crate::entities::language::{Language as LanguageEntity, LanguageLevel as LanguageLevelEntity};
use crate::entities::person::{Nationality as NationalityEntity, Person as PersonEntity};
use crate::entities::title::{Position as PositionEntity, Title as TitleEntity};
use crate::entities::vow_progress::VowProgress as VowProgressEntity;
use crate::ports::person::models::person_dbresponse::Person as PersonDbResponse;
use crate::ports::person_db_gateway::PersonDbGateway;
use crate::ports::personal_id_number::models::personal_id_number_db_response::PersonalIdNumberDbResponse;
use crate::ports::saint::saint_db_gateway::SaintDbGateway;
use crate::usecases::person_usecase_shared_models::educational_stage::{
    PersonUsecaseSharedEducationalLevel, PersonUsecaseSharedEducationalStage,
};
use crate::usecases::person_usecase_shared_models::language::{
    PersonUsecaseSharedLanguage, PersonUsecaseSharedLanguageLevel,
};
use crate::usecases::person_usecase_shared_models::nationality::PersonUsecaseSharedNationality;
use crate::usecases::person_usecase_shared_models::personal_id_number::PersonUsecaseSharedIdNumber;
use crate::usecases::person_usecase_shared_models::title::{
    PersonUsecaseSharedPosition, PersonUsecaseSharedTitle,
};
use crate::usecases::person_usecase_shared_models::vow_progress::PersonUsecaseSharedVowProgress;
use crate::usecases::{ToEntity, ToUsecaseOutput, UsecaseError};
use async_trait::async_trait;
use chrono::NaiveDate;
use uuid::Uuid;

pub struct CreatePersonUsecaseInteractor<A: PersonDbGateway, B: SaintDbGateway> {
    person_db_gateway: A,
    saint_db_gateway: B,
}

impl<A, B> CreatePersonUsecaseInteractor<A, B>
where
    A: PersonDbGateway + Sync + Send,
    B: SaintDbGateway + Sync + Send,
{
    pub fn new(person_db_gateway: A, saint_db_gateway: B) -> Self {
        CreatePersonUsecaseInteractor {
            person_db_gateway,
            saint_db_gateway,
        }
    }
}

#[async_trait]
pub trait CreatePersonUsecase {
    // InputBoundary
    async fn execute(
        &mut self,
        request: CreatePersonUsecaseInput,
    ) -> Result<CreatePersonUsecaseOutput, UsecaseError>;
}

#[async_trait]
impl<A, B> CreatePersonUsecase for CreatePersonUsecaseInteractor<A, B>
where
    A: PersonDbGateway + Sync + Send,
    B: SaintDbGateway + Sync + Send,
{
    async fn execute(
        &mut self,
        request: CreatePersonUsecaseInput,
    ) -> Result<CreatePersonUsecaseOutput, UsecaseError> {
        let person = request.to_entity();

        if !person.is_valid() {
            println!("This person is not valid");
            return Err(UsecaseError::InvalidInput);
        }

        let mut christian_name = String::new();
        if let Some(saint_ids) = person.saint_ids.clone() {
            for saint_id in saint_ids {
                let saint_db_response = (*self).saint_db_gateway.find_one_by_id(saint_id).await;
                if saint_db_response.is_none() {
                    eprintln!("Saint id ({:?}) not found", saint_id);
                    return Err(UsecaseError::ResourceNotFound);
                }
                christian_name.push_str(
                    &saint_db_response
                        .map(|saint| saint.display_name.unwrap())
                        .unwrap(),
                );
                christian_name.push(' ');
            }
        }

        println!("This person is valid");
        let usecase_output: Result<CreatePersonUsecaseOutput, UsecaseError> = (*self)
            .person_db_gateway
            .insert(person.to_mutation_db_request())
            .await
            .map(|response| response.to_usecase_output())
            .map_err(|err| err.to_usecase_error());

        return match usecase_output {
            Ok(mut output) => {
                println!("Create successfully");
                output.christian_name = Some(christian_name);
                Ok(output)
            }
            Err(error) => {
                println!("Create fail");
                Err(error)
            }
        };
    }
}

pub struct CreatePersonUsecaseInput {
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub saint_ids: Option<Vec<Uuid>>,
    pub date_of_birth: Option<NaiveDate>,
    pub place_of_birth: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub nationality: Option<PersonUsecaseSharedNationality>,
    pub race: Option<String>,
    pub personal_id_numbers: Option<Vec<PersonUsecaseSharedIdNumber>>,
    pub languages: Option<Vec<PersonUsecaseSharedLanguage>>,
    pub educational_stages: Option<Vec<PersonUsecaseSharedEducationalStage>>,
    pub position: Option<PersonUsecaseSharedPosition>,
}

pub struct CreatePersonUsecaseOutput {
    pub person_id: Uuid,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub address: Option<String>,
    pub christian_name: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub place_of_birth: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub personal_id_numbers: Option<Vec<PersonUsecaseSharedIdNumber>>,
    pub nationality: Option<PersonUsecaseSharedNationality>,
    pub race: Option<String>,
    pub languages: Option<Vec<PersonUsecaseSharedLanguage>>,
    pub educational_stages: Option<Vec<PersonUsecaseSharedEducationalStage>>,
    pub position: Option<PersonUsecaseSharedPosition>,
}

impl ToUsecaseOutput<PersonUsecaseSharedIdNumber> for PersonalIdNumberDbResponse {
    fn to_usecase_output(self) -> PersonUsecaseSharedIdNumber {
        PersonUsecaseSharedIdNumber {
            id_number: self.id_number,
            code: self.code.map(|code| code.parse().unwrap()),
            date_of_issue: self.date_of_issue,
            place_of_issue: self.place_of_issue,
        }
    }
}

impl ToUsecaseOutput<PersonUsecaseSharedLanguageLevel> for LanguageLevelEntity {
    fn to_usecase_output(self) -> PersonUsecaseSharedLanguageLevel {
        match self {
            LanguageLevelEntity::Beginner => PersonUsecaseSharedLanguageLevel::Beginner,
            LanguageLevelEntity::Intermediate => PersonUsecaseSharedLanguageLevel::Intermediate,
            LanguageLevelEntity::Advanced => PersonUsecaseSharedLanguageLevel::Advanced,
        }
    }
}

impl ToUsecaseOutput<PersonUsecaseSharedLanguage> for LanguageEntity {
    fn to_usecase_output(self) -> PersonUsecaseSharedLanguage {
        PersonUsecaseSharedLanguage {
            language: self.language.unwrap(),
            level: self.level.unwrap().to_usecase_output(),
        }
    }
}

impl ToUsecaseOutput<PersonUsecaseSharedTitle> for TitleEntity {
    fn to_usecase_output(self) -> PersonUsecaseSharedTitle {
        match self {
            TitleEntity::Monk => PersonUsecaseSharedTitle::Monk,
            TitleEntity::Nun => PersonUsecaseSharedTitle::Nun,
            TitleEntity::Priest => PersonUsecaseSharedTitle::Priest,
        }
    }
}

impl ToUsecaseOutput<PersonUsecaseSharedVowProgress> for VowProgressEntity {
    fn to_usecase_output(self) -> PersonUsecaseSharedVowProgress {
        match self {
            VowProgressEntity::SolemnVow => PersonUsecaseSharedVowProgress::SolemnVow,
            VowProgressEntity::SimpleVow => PersonUsecaseSharedVowProgress::SimpleVow,
            VowProgressEntity::Preparation => PersonUsecaseSharedVowProgress::Preparation,
            VowProgressEntity::Novice => PersonUsecaseSharedVowProgress::Novice,
        }
    }
}

impl ToUsecaseOutput<PersonUsecaseSharedPosition> for PositionEntity {
    fn to_usecase_output(self) -> PersonUsecaseSharedPosition {
        PersonUsecaseSharedPosition {
            title: self.title.map(|t| t.to_usecase_output()),
            period: self.period.map(|p| p.to_usecase_output()),
            parish: self.parish,
        }
    }
}

impl ToUsecaseOutput<PersonUsecaseSharedEducationalLevel> for EducationalLevelEntity {
    fn to_usecase_output(self) -> PersonUsecaseSharedEducationalLevel {
        match self {
            EducationalLevel::Other => PersonUsecaseSharedEducationalLevel::Other,
            EducationalLevel::Doctor => PersonUsecaseSharedEducationalLevel::Doctor,
            EducationalLevel::MiddleSchool => PersonUsecaseSharedEducationalLevel::MiddleSchool,
            EducationalLevel::HighSchool => PersonUsecaseSharedEducationalLevel::HighSchool,
            EducationalLevel::ElementarySchool => {
                PersonUsecaseSharedEducationalLevel::ElementarySchool
            }
            EducationalLevel::Master => PersonUsecaseSharedEducationalLevel::Master,
            EducationalLevel::Bachelor => PersonUsecaseSharedEducationalLevel::Bachelor,
        }
    }
}

impl ToUsecaseOutput<PersonUsecaseSharedEducationalStage> for EducationalStageEntity {
    fn to_usecase_output(self) -> PersonUsecaseSharedEducationalStage {
        PersonUsecaseSharedEducationalStage {
            educational_level: self
                .educational_level
                .map(|level| level.to_usecase_output()),
            school_name: self.school_name.unwrap(),
            major: self.major,
            graduate_year: self.graduate_year,
        }
    }
}

impl ToUsecaseOutput<PersonUsecaseSharedNationality> for NationalityEntity {
    fn to_usecase_output(self) -> PersonUsecaseSharedNationality {
        match self {
            NationalityEntity::American => PersonUsecaseSharedNationality::American,
            NationalityEntity::British => PersonUsecaseSharedNationality::British,
            NationalityEntity::Chinese => PersonUsecaseSharedNationality::Chinese,
            NationalityEntity::French => PersonUsecaseSharedNationality::French,
            NationalityEntity::Vietnamese => PersonUsecaseSharedNationality::Vietnamese,
        }
    }
}

impl ToUsecaseOutput<CreatePersonUsecaseOutput> for PersonDbResponse {
    fn to_usecase_output(self) -> CreatePersonUsecaseOutput {
        let mut personal_id_numbers: Vec<PersonUsecaseSharedIdNumber> = Vec::new();
        if let Some(personal_id_numbers_db_response) = self.personal_id_numbers {
            for personal_id_number in personal_id_numbers_db_response {
                personal_id_numbers.push(personal_id_number.to_usecase_output());
            }
        }

        let mut educational_stages = None;
        if let Some(stages) = self.educational_stages {
            educational_stages = Some(
                stages
                    .into_iter()
                    .map(|stage| stage.to_usecase_output())
                    .collect(),
            )
        }
        CreatePersonUsecaseOutput {
            person_id: self.id,
            first_name: self.first_name.clone(),
            middle_name: self.middle_name.clone(),
            last_name: self.last_name.clone(),
            address: self.address.clone(),
            christian_name: None,
            date_of_birth: self.date_of_birth,
            place_of_birth: self.place_of_birth.clone(),
            email: self.email.clone(),
            phone: self.phone,
            personal_id_numbers: Some(personal_id_numbers),
            nationality: self
                .nationality
                .map(|nationality| nationality.to_usecase_output()),
            race: self.race,
            languages: self.languages.map(|languages| {
                languages
                    .into_iter()
                    .map(|lang| lang.to_usecase_output())
                    .collect()
            }),
            educational_stages,
            position: self.position.map(|p| p.to_usecase_output()),
        }
    }
}

impl ToEntity<PersonEntity> for CreatePersonUsecaseInput {
    fn to_entity(self) -> PersonEntity {
        let mut personal_id_numbers = Vec::new();
        for pin in self.personal_id_numbers.unwrap() {
            personal_id_numbers.push(pin.to_entity());
        }

        let mut nationality: Option<NationalityEntity> = None;
        if let Some(nationality_usecase_input) = self.nationality {
            nationality = Some(nationality_usecase_input.to_entity())
        }

        let mut languages: Option<Vec<LanguageEntity>> = None;
        if let Some(langs) = self.languages {
            languages = Some(langs.into_iter().map(|lang| lang.to_entity()).collect());
        }

        PersonEntity {
            id: Some(Uuid::new_v4()),
            first_name: self.first_name,
            middle_name: self.middle_name,
            last_name: self.last_name,
            date_of_birth: self.date_of_birth,
            place_of_birth: self.place_of_birth,
            email: self.email,
            phone: self.phone,
            nationality,
            race: self.race,
            address: self.address,
            saint_ids: self.saint_ids,
            personal_id_numbers: Some(personal_id_numbers),
            languages,
            educational_stages: self
                .educational_stages
                .map(|stages| stages.into_iter().map(|stage| stage.to_entity()).collect()),
            position: self.position.map(|p| p.to_entity()),
        }
    }
}
