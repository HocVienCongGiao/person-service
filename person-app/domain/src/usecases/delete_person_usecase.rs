use uuid::Uuid;
use crate::ports::person_db_gateway::PersonDbGateway;
use crate::ports::personal_id_number::personal_id_number_db_gateway::PersonalIdNumberGateway;
use crate::usecases::UsecaseError;
use async_trait::async_trait;

pub struct DeleteOnePersonByIdUsecaseInteractor<A: PersonDbGateway, B: PersonalIdNumberGateway> {
    person_db_gateway: A,
    personal_id_number_db_gateway: B
}

impl <A, B> DeleteOnePersonByIdUsecaseInteractor<A, B>
where
    A: PersonDbGateway + Sync + Send,
B: PersonalIdNumberGateway + Sync + Send, {
    pub fn new(person_db_gateway: A, personal_id_number_db_gateway: B) -> Self {
        DeleteOnePersonByIdUsecaseInteractor {
            person_db_gateway,
            personal_id_number_db_gateway,
        }
    }
}

#[async_trait]
pub trait CreatePersonUsecase {
    async fn execute(
        &mut self,
        id: Uuid,
    ) -> Result<(), UsecaseError>;
}

impl <A, B> DeleteOnePersonByIdUsecaseInteractor<A, B>
where
    A: PersonDbGateway,
    B: PersonalIdNumberGateway
{
    pub async fn execute(&self, id: Uuid) -> Result<(), UsecaseError> {
        (*self).person_db_gateway.delete_one_by_id(id).await;
        Ok(())
    }
}