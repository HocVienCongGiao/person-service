use crate::ports::person_dbresponse::Person as PersonDbResponse;
use crate::ports::person_mutation_dbrequest::Person as PersonMutationDbRequest;
use crate::ports::DbError;
use async_trait::async_trait;

#[async_trait]
pub trait UpdateOnePersonByIdPort {
    async fn update_one_by_id(
        &mut self,
        db_request: PersonMutationDbRequest,
    ) -> Result<PersonDbResponse, DbError>;
}
