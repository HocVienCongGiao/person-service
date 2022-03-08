use uuid::Uuid;
use crate::ports::person_db_gateway::PersonDbGateway;
use crate::usecases::UsecaseError;
use async_trait::async_trait;

pub struct DeleteOnePersonByIdUsecaseInteractor<A: PersonDbGateway> {
    person_db_gateway: A,
}

impl <A> DeleteOnePersonByIdUsecaseInteractor<A>
where
    A: PersonDbGateway + Sync + Send,
{
    pub fn new(person_db_gateway: A) -> Self {
        DeleteOnePersonByIdUsecaseInteractor {
            person_db_gateway,
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

impl <A> DeleteOnePersonByIdUsecaseInteractor<A>
where
    A: PersonDbGateway,
{
    pub async fn execute(&self, id: Uuid) -> Result<(), UsecaseError> {
        let result = (*self).person_db_gateway.delete_one_by_id(id).await;
        match result {
            Err(error) => Err(error.to_usecase_error()),
            _ => Ok(())
        }
    }
}