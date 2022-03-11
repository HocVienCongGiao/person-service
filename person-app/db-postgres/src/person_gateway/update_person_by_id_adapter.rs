use domain::ports::person_dbresponse::Person as PersonDbResponse;
use domain::ports::person_mutation_dbrequest::Person as PersonMutationDbRequest;use domain::ports::DbError;
use async_trait::async_trait;
use uuid::Uuid;
use domain::ports::person::update_person_by_id_port::UpdateOnePersonByIdPort;
use crate::person_gateway::repository::PersonRepository;

#[async_trait]
impl UpdateOnePersonByIdPort for PersonRepository {
    async fn update_one_by_id(
        &self,
        id: Uuid,
        db_request: PersonMutationDbRequest,
    ) -> Result<PersonDbResponse, DbError> {
        Ok(PersonDbResponse {
            id,
            personal_id_numbers: None,
            first_name: None,
            middle_name: None,
            last_name: None,
            date_of_birth: None,
            place_of_birth: None,
            email: None,
            phone: None
        })
    }
}
