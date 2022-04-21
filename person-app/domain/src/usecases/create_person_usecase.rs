use crate::entities::language::Language as LanguageEntity;
use crate::entities::person::{Nationality, Person as PersonEntity};
use crate::ports::person::models::person_dbresponse::Person as PersonDbResponse;
use crate::ports::person_db_gateway::PersonDbGateway;
use crate::ports::personal_id_number::models::personal_id_number_db_response::PersonalIdNumberDbResponse;
use crate::usecases::person_usecase_shared_models::educational_stage::PersonUsecaseSharedEducationalStage;
use crate::usecases::person_usecase_shared_models::language::PersonUsecaseSharedLanguage;
use crate::usecases::person_usecase_shared_models::nationality::PersonUsecaseSharedNationality;
use crate::usecases::person_usecase_shared_models::personal_id_number::PersonUsecaseSharedIdNumber;
use crate::usecases::person_usecase_shared_models::title::PersonUsecaseSharedPosition;
use crate::usecases::{ToEntity, ToUsecaseOutput, UsecaseError};
use async_trait::async_trait;
use chrono::NaiveDate;
use uuid::Uuid;

pub struct CreatePersonUsecaseInteractor<A: PersonDbGateway> {
    person_db_gateway: A,
}

impl<A> CreatePersonUsecaseInteractor<A>
where
    A: PersonDbGateway + Sync + Send,
{
    pub fn new(person_db_gateway: A) -> Self {
        CreatePersonUsecaseInteractor { person_db_gateway }
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
impl<A> CreatePersonUsecase for CreatePersonUsecaseInteractor<A>
where
    A: PersonDbGateway + Sync + Send,
{
    async fn execute(
        &mut self,
        request: CreatePersonUsecaseInput,
    ) -> Result<CreatePersonUsecaseOutput, UsecaseError> {
        let person = request.to_entity();
        if person.is_valid() {
            println!("This person is valid");
            let usecase_output: Result<CreatePersonUsecaseOutput, UsecaseError> = (*self)
                .person_db_gateway
                .insert(person.to_mutation_db_request())
                .await
                .map(|response| response.to_usecase_output())
                .map_err(|err| err.to_usecase_error());

            return match usecase_output {
                Ok(output) => {
                    println!("Create successfully");
                    // let mut output = output.with_personal_id_numbers()
                    Ok(output)
                }
                Err(error) => {
                    println!("Create fail");
                    Err(error)
                }
            };
        } else {
            println!("This person is not valid");
            Err(UsecaseError::InvalidInput)
        }
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

#[derive(Clone)]
pub struct CreatePersonUsecaseOutput {
    pub person_id: Uuid,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub place_of_birth: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub personal_id_numbers: Option<Vec<PersonUsecaseSharedIdNumber>>,
    // TODO: add more
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

impl ToUsecaseOutput<CreatePersonUsecaseOutput> for PersonDbResponse {
    fn to_usecase_output(self) -> CreatePersonUsecaseOutput {
        let mut personal_id_numbers: Vec<PersonUsecaseSharedIdNumber> = Vec::new();
        if let Some(personal_id_numbers_db_response) = self.personal_id_numbers {
            for personal_id_number in personal_id_numbers_db_response {
                personal_id_numbers.push(personal_id_number.to_usecase_output());
            }
        }
        CreatePersonUsecaseOutput {
            person_id: self.id,
            first_name: self.first_name.clone(),
            middle_name: self.middle_name.clone(),
            last_name: self.last_name.clone(),
            date_of_birth: self.date_of_birth,
            place_of_birth: self.place_of_birth.clone(),
            email: self.email.clone(),
            phone: self.phone,
            personal_id_numbers: Some(personal_id_numbers),
        }
    }
}

impl ToEntity<PersonEntity> for CreatePersonUsecaseInput {
    fn to_entity(self) -> PersonEntity {
        let mut personal_id_numbers = Vec::new();
        for pin in self.personal_id_numbers.unwrap() {
            personal_id_numbers.push(pin.to_entity());
        }

        let mut nationality: Option<Nationality> = None;
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
