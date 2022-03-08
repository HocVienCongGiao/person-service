use domain::usecases::UsecaseError;
use hvcg_biography_openapi_person::models::{
    PersonUpsert as PersonUpsertOpenApi, PersonView as PersonViewOpenApi,
};
use uuid::Uuid;

mod create_person;
mod get_one_person_by_id;
pub mod openapi;
mod delete_one_person_by_id;

pub async fn get_one_person_by_id(id: Uuid) -> Option<PersonViewOpenApi> {
    get_one_person_by_id::from_uuid(id).await
}

pub async fn delete_one_person_by_id(id: Uuid) -> Result<(), UsecaseError> {
    delete_one_person_by_id::from_uuid(id).await
}

pub async fn create_person(
    person_request: PersonUpsertOpenApi,
) -> Result<PersonViewOpenApi, UsecaseError> {
    create_person::from_openapi(person_request).await
}
