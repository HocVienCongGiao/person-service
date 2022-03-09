use crate::ports::person_db_gateway::PersonDbGateway;
use async_trait::async_trait;
use chrono::NaiveDate;
use uuid::Uuid;
use crate::entities::person::{Nationality, Person as PersonEntity};
use crate::entities::personal_id_number::PersonalIdNumber;
use crate::ports::personal_id_number::personal_id_number_db_gateway::PersonalIdNumberGateway;
use crate::usecases::person_usecase_shared_models::{PersonUsecaseSharedIdNumber, PersonUsecaseSharedNationality};
use crate::usecases::{ToEntity, ToUsecaseOutput, UsecaseError};
use crate::ports::person_dbresponse::Person as PersonDbResponse;
pub struct UpdatePersonUsecaseInteractor<A: PersonDbGateway, B: PersonalIdNumberGateway> {
    person_db_gateway: A,
    personal_id_number_db_gateway: B,
}

impl<A, B> UpdatePersonUsecaseInteractor<A, B>
    where
        A: PersonDbGateway + Sync + Send,
        B: PersonalIdNumberGateway + Sync + Send
{
    pub fn new(
        person_db_gateway: A,
        personal_id_number_db_gateway: B,
    ) -> Self { UpdatePersonUsecaseInteractor { person_db_gateway, personal_id_number_db_gateway } }
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
        B: PersonalIdNumberGateway + Sync + Send
{
    async fn execute(&mut self, request: UpdatePersonUsecaseInput) -> Result<UpdatePersonUsecaseOutput, UsecaseError> {
        let person = request.to_entity();
        if person.is_valid() {
            println!("This person is valid");
            let usecase_output: Result<UpdatePersonUsecaseOutput, UsecaseError> = (*self)
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

pub struct UpdatePersonUsecaseInput {
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
    pub personal_id_number: Option<Vec<PersonUsecaseSharedIdNumber>>,
}

#[derive(Clone)]
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
    pub personal_id_number: Option<Vec<PersonUsecaseSharedIdNumber>>,
}

impl ToEntity<PersonEntity> for UpdatePersonUsecaseInput {
    fn to_entity(self) -> PersonEntity {
        let mut nationality: Option<Nationality> = None;
        if let Some(nationality_usecase_input) = self.nationality {
            nationality = Some(nationality.to_entity())
        }

        let mut personal_id_numbers: Vec<PersonalIdNumber> = Vec::new();
        if let Some(personal_id_numbers_request) = self.personal_id_number {
            for pin in personal_id_numbers_request {
                personal_id_numbers.push(PersonalIdNumber{
                    id: None,
                    id_number: pin.id_number,
                    // code: pin.code, TODO: refactor
                    code: None,
                    date_of_issue: pin.date_of_issue,
                    place_of_issue: pin.place_of_issue
                })
            }
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
            race: None,
            personal_id_numbers: None,
            address: self.address
        }
    }
}

impl ToUsecaseOutput<UpdatePersonUsecaseOutput> for PersonDbResponse {
    fn to_usecase_output(self) -> UpdatePersonUsecaseOutput {
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
            personal_id_number: None
        }
    }
}