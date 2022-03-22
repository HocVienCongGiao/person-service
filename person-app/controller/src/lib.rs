use crate::openapi::person_view::PersonCollectionQuery;
use domain::usecases::UsecaseError;
use hvcg_biography_openapi_person::models::{
    PersonUpsert as PersonUpsertOpenApi, PersonView as PersonViewOpenApi, PersonViewCollection,
};
use uuid::Uuid;

mod create_person;
mod delete_one_person_by_id;
mod get_one_person_by_id;
mod get_person_collection;
pub mod openapi;
mod update_person_by_id;

pub async fn get_one_person_by_id(id: Uuid) -> Option<PersonViewOpenApi> {
    get_one_person_by_id::from_uuid(id).await
}

pub async fn get_person_collection(query: PersonCollectionQuery) -> PersonViewCollection {
    get_person_collection::from_usecase_input(query.to_usecase_input()).await
}

pub async fn delete_one_person_by_id(id: Uuid) -> Result<(), UsecaseError> {
    delete_one_person_by_id::from_uuid(id).await
}

pub async fn create_person(
    person_request: PersonUpsertOpenApi,
) -> Result<PersonViewOpenApi, UsecaseError> {
    create_person::from_openapi(person_request).await
}

pub async fn update_person_by_id(
    person_id: Uuid,
    person_request: PersonUpsertOpenApi,
) -> Result<PersonViewOpenApi, UsecaseError> {
    update_person_by_id::from_openapi(person_id, person_request).await
}
