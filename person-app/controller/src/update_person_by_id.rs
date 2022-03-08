use db_postgres::person_gateway::repository::PersonRepository;
use domain::usecases::update_one_person_by_id::UpdatePersonUsecaseInteractor;
use domain::usecases::UsecaseError;
use crate::{PersonUpsertOpenApi, PersonViewOpenApi};

pub async fn from_openapi(person: PersonUpsertOpenApi) -> Result<PersonViewOpenApi, UsecaseError> {
    // Init dependencies
    let client = db_postgres::connect().await;
    let person_repository = PersonRepository { client };

    // Inject dependencies to Interactor and invoke func
    let result = UpdatePersonUsecaseInteractor::new(person_repository)
        .execute(person.to_usecase_input())
        .await;
    result.map(|res| res.to_openapi())
}