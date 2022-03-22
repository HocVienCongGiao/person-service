use crate::ports::person::models::person_dbrequest::PersonQuery as PersonQueryDbRequest;
use crate::ports::person::models::person_dbresponse::PersonCollection as PersonCollectionDbResponse;
use async_trait::async_trait;

#[async_trait]
pub trait FindPersonCollectionPort {
    async fn find_collection_by(
        &self,
        db_request: PersonQueryDbRequest,
    ) -> PersonCollectionDbResponse;
}
