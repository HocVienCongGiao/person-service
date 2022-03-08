use crate::ports::person_db_gateway::PersonDbGateway;
use async_trait::async_trait;
use crate::usecases::UsecaseError;

pub struct UpdatePersonUsecaseInteractor<A: PersonDbGateway> {
    person_db_gateway: A
}

impl<A> UpdatePersonUsecaseInteractor<A>
where
A: PersonDbGateway + Sync + Send {
    pub fn new(person_db_gateway: A) -> Self { UpdatePersonUsecaseInteractor {person_db_gateway}}
}

#[async_trait]
pub trait UpdatePersonUsecase {
    async fn execute(
        &mut self,
        request: UpdatePersonUsecaseInput
    ) -> Result<UpdatePersonUsecaseOutput, UsecaseError>;
}

#[async_trait]
impl<A> UpdatePersonUsecase for UpdatePersonUsecaseInteractor<A>
where
A: PersonDbGateway + Sync + Send,
{
    async fn execute(&mut self, request: UpdatePersonUsecaseInput) -> Result<UpdatePersonUsecaseOutput, UsecaseError> {
        let person = request.to_entity();
        if person.is_valid() {
            println!("This person is valid");
            let usecase_output: Result<UpdatePersonUsecaseOutput, UsecaseError> = (*self)
                .person_db_gateway
                .insert(person.to_mutation_db_request())
                .await
                .map(|response| response.to_usecase_output())
                .map_err(|err| err.to_usecase_error());

            return match usecase_output {
                Ok(output) => {
                    println!("Create successfully");
                    // let mut output = output.with_personal_id_numbers()
                    Ok(output)
                }
                Err(error) => {
                    println!("Create fail");
                    Err(error)
                }
            };
        } else {
            println!("This person is not valid");
            Err(UsecaseError::InvalidInput)
        }
    }
}