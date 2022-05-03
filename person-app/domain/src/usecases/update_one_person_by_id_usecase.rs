use crate::entities::language::Language as LanguageEntity;
use crate::entities::person::{Nationality, Person as PersonEntity};
use crate::entities::personal_id_number::PersonalIdNumber;
use crate::ports::person::models::person_dbresponse::Person as PersonDbResponse;
use crate::ports::person_db_gateway::PersonDbGateway;
use crate::ports::personal_id_number::personal_id_number_db_gateway::PersonalIdNumberGateway;
use crate::usecases::person_usecase_shared_models::educational_stage::PersonUsecaseSharedEducationalStage;
use crate::usecases::person_usecase_shared_models::language::PersonUsecaseSharedLanguage;
use crate::usecases::person_usecase_shared_models::nationality::PersonUsecaseSharedNationality;
use crate::usecases::person_usecase_shared_models::personal_id_number::PersonUsecaseSharedIdNumber;
use crate::usecases::person_usecase_shared_models::title::PersonUsecaseSharedPosition;
use crate::usecases::query_one_personal_id_number_usecase::PersonalIdNumberUsecaseOutput;
use crate::usecases::{ToEntity, ToUsecaseOutput, UsecaseError};
use async_trait::async_trait;
use chrono::NaiveDate;
use uuid::Uuid;

pub struct UpdatePersonUsecaseInteractor<A: PersonDbGateway, B: PersonalIdNumberGateway> {
    person_db_gateway: A,
    personal_id_number_db_gateway: B,
}

impl<A, B> UpdatePersonUsecaseInteractor<A, B>
where
    A: PersonDbGateway + Sync + Send,
    B: PersonalIdNumberGateway + Sync + Send,
{
    pub fn new(person_db_gateway: A, personal_id_number_db_gateway: B) -> Self {
        UpdatePersonUsecaseInteractor {
            person_db_gateway,
            personal_id_number_db_gateway,
        }
    }
}

#[async_trait]
pub trait UpdatePersonUsecase {
    async fn execute(
        &mut self,
        request: UpdatePersonUsecaseInput,
    ) -> Result<UpdatePersonUsecaseOutput, UsecaseError>;
}

#[async_trait]
impl<A, B> UpdatePersonUsecase for UpdatePersonUsecaseInteractor<A, B>
where
    A: PersonDbGateway + Sync + Send,
    B: PersonalIdNumberGateway + Sync + Send,
{
    async fn execute(
        &mut self,
        request: UpdatePersonUsecaseInput,
    ) -> Result<UpdatePersonUsecaseOutput, UsecaseError> {
        let person = request.to_entity();
        if person.is_valid() {
            println!("This person is valid");

            let person_db_response = (*self)
                .person_db_gateway
                .find_one_by_id(person.id.unwrap())
                .await;
            if person_db_response.is_none() {
                println!("Person ID not found");
                return Err(UsecaseError::ResourceNotFound);
            }

            let usecase_output: Result<UpdatePersonUsecaseOutput, UsecaseError> = (*self)
                .person_db_gateway
                .update_one_by_id(person.to_mutation_db_request())
                .await
                .map(|response| response.to_usecase_output())
                .map_err(|err| err.to_usecase_error());

            return match usecase_output {
                Ok(output) => {
                    println!("Update successfully");
                    // let mut output = output.with_personal_id_numbers()
                    Ok(output)
                }
                Err(error) => {
                    println!("Update fail");
                    Err(error)
                }
            };
        } else {
            println!("This person is not valid");
            Err(UsecaseError::InvalidInput)
        }
    }
}

pub struct UpdatePersonUsecaseInput {
    pub person_id: Option<Uuid>,
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

pub struct UpdatePersonUsecaseOutput {
    pub person_id: Option<Uuid>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub place_of_birth: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub nationality: Option<PersonUsecaseSharedNationality>,
    pub race: Option<String>,
    pub personal_id_numbers: Option<Vec<PersonalIdNumberUsecaseOutput>>,
}

impl ToEntity<PersonEntity> for UpdatePersonUsecaseInput {
    fn to_entity(self) -> PersonEntity {
        let mut nationality: Option<Nationality> = None;
        if let Some(nationality_usecase_input) = self.nationality {
            nationality = Some(nationality_usecase_input.to_entity())
        }

        let mut personal_id_numbers = Vec::new();
        for pin in self.personal_id_numbers.unwrap() {
            personal_id_numbers.push(pin.to_entity());
        }

        let mut educational_stages = Vec::new();
        for stage in self.educational_stages.unwrap() {
            educational_stages.push(stage.to_entity())
        }

        let mut languages: Option<Vec<LanguageEntity>> = None;
        if let Some(langs) = self.languages {
            languages = Some(langs.into_iter().map(|lang| lang.to_entity()).collect());
        }

        PersonEntity {
            id: self.person_id,
            first_name: self.first_name,
            middle_name: self.middle_name,
            last_name: self.last_name,
            date_of_birth: self.date_of_birth,
            place_of_birth: self.place_of_birth,
            email: self.email,
            phone: self.phone,
            nationality,
            race: self.race,
            personal_id_numbers: Some(personal_id_numbers),
            address: self.address,
            saint_ids: self.saint_ids,
            languages,
            educational_stages: Some(educational_stages),
            position: self.position.map(|p| p.to_entity()),
        }
    }
}

impl ToUsecaseOutput<UpdatePersonUsecaseOutput> for PersonDbResponse {
    fn to_usecase_output(self) -> UpdatePersonUsecaseOutput {
        let mut personal_id_numbers = Vec::new();
        if let Some(personal_id_numbers_db_response) = self.personal_id_numbers {
            for personal_id_number in personal_id_numbers_db_response {
                personal_id_numbers.push(personal_id_number.to_usecase_output());
            }
        }
        UpdatePersonUsecaseOutput {
            person_id: Some(self.id),
            first_name: self.first_name.clone(),
            middle_name: self.middle_name.clone(),
            last_name: self.last_name.clone(),
            date_of_birth: self.date_of_birth,
            place_of_birth: self.place_of_birth.clone(),
            email: self.email.clone(),
            phone: self.phone,
            address: None,
            nationality: None,
            race: None,
            personal_id_numbers: Some(personal_id_numbers),
        }
    }
}

impl UpdatePersonUsecaseInput {
    pub fn with_person_id(mut self, id: Uuid) -> UpdatePersonUsecaseInput {
        self.person_id = Some(id);
        self
    }
}
