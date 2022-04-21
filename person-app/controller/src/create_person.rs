use crate::openapi::ToOpenApi;
use crate::openapi::ToUsecaseInput;
use crate::PersonViewOpenApi;
use db_postgres::person_gateway::repository::PersonRepository;
use domain::usecases::create_person_usecase::CreatePersonUsecase;
use domain::usecases::create_person_usecase::{
    CreatePersonUsecaseInput, CreatePersonUsecaseInteractor,
};
use domain::usecases::UsecaseError;
use hvcg_biography_openapi_person::models::PersonUpsert as PersonUpsertOpenApi;

pub async fn from_openapi(person: PersonUpsertOpenApi) -> Result<PersonViewOpenApi, UsecaseError> {
    // Init dependencies
    let client = db_postgres::connect().await;
    let person_repository = PersonRepository { client };

    // Inject dependencies to Interactor and invoke func
    let result = CreatePersonUsecaseInteractor::new(person_repository)
        .execute(person.to_usecase_input())
        .await;
    result.map(|res| res.to_openapi())
}

impl ToUsecaseInput<CreatePersonUsecaseInput> for PersonUpsertOpenApi {
    fn to_usecase_input(self) -> CreatePersonUsecaseInput {
        CreatePersonUsecaseInput {
            first_name: self.first_name.clone(),
            middle_name: self.middle_name.clone(),
            last_name: self.last_name.clone(),
            saint_ids: self.saint_id_array.clone(),
            date_of_birth: self.date_of_birth,
            place_of_birth: self.place_of_birth.clone(),
            email: self.email.clone(),
            phone: self.phone,
            address: self.address,
            nationality: self.nationality.map(|n| n.to_usecase_input()),
            race: self.race,
            personal_id_numbers: self.personal_id_numbers.map(|id_numbders| {
                id_numbders
                    .into_iter()
                    .map(|id_number| id_number.to_usecase_input())
                    .collect()
            }),
            languages: self.language.map(|languages| {
                languages
                    .into_iter()
                    .map(|lang| lang.to_usecase_input())
                    .collect()
            }),
            educational_stages: self.education_stage.map(|stages| {
                stages
                    .into_iter()
                    .map(|stage| stage.to_usecase_input())
                    .collect()
            }),
            position: self.position.map(|p| p.to_usecase_input()),
        }
    }
}
