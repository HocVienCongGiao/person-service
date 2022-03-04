use crate::ports::personal_id_number::models::personal_id_number_db_response::PersonalIdNumberDbResponse;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait FindPersonalIdNumberCollectionPort {
    async fn find_collection_by_person_id(
        &self,
        person_id: Uuid,
    ) -> Vec<PersonalIdNumberDbResponse>;
}
