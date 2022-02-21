use domain::usecases::UsecaseError;
use hvcg_biography_openapi_person::models::{
    PersonUpsert as PersonUpsertOpenApi, PersonView as PersonViewOpenApi,
};

mod create_person;
pub mod openapi;

pub async fn create_person(
    person_request: PersonUpsertOpenApi,
) -> Result<PersonViewOpenApi, UsecaseError> {
    create_person::from_openapi(person_request).await
}
