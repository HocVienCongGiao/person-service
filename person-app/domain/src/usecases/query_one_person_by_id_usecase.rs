use async_trait::async_trait;
use uuid::Uuid;

use crate::ports::person_db_gateway::PersonDbGateway;
use crate::ports::person_dbresponse::Person as PersonDbResponse;
use crate::ports::personal_id_number::personal_id_number_db_gateway::PersonalIdNumberGateway;
use crate::usecases::query_one_personal_id_number_usecase::{
    PersonalIdNumberUsecaseOutput, QueryPersonUsecaseOutput,
};
use crate::usecases::ToUsecaseOutput;

pub struct QueryOnePersonByIdUsecaseInteractor<A: PersonDbGateway, B: PersonalIdNumberGateway> {
    person_db_gateway: A,
    personal_id_number_db_gateway: B,
}

#[async_trait]
pub trait QueryOnePersonByIdUsecase {
    // InputBoundary
    async fn execute(&self, id: Uuid) -> Option<QueryPersonUsecaseOutput>;
}

#[async_trait]
impl<A, B> QueryOnePersonByIdUsecase for QueryOnePersonByIdUsecaseInteractor<A, B>
where
    A: PersonDbGateway + Sync + Send,
    B: PersonalIdNumberGateway + Sync + Send,
{
    async fn execute(&self, id: Uuid) -> Option<QueryPersonUsecaseOutput> {
        let usecase_output: Option<QueryPersonUsecaseOutput> = (*self)
            .person_db_gateway
            .find_one_by_id(id)
            .await
            .map(|response| response.to_usecase_output());

        return if let Some(mut usecase_output) = usecase_output {
            let personal_id_numbers = (*self)
                .personal_id_number_db_gateway
                .find_collection_by_person_id(usecase_output.id)
                .await;

            let mut personal_id_numbers_output: Vec<PersonalIdNumberUsecaseOutput> = Vec::new();
            for personal_id_number in personal_id_numbers {
                println!("test {:?}", personal_id_number.id_number);
                personal_id_numbers_output.push(PersonalIdNumberUsecaseOutput {
                    id_number: personal_id_number.id_number,
                    code: personal_id_number.code,
                    date_of_issue: personal_id_number.date_of_issue,
                    place_of_issue: personal_id_number.place_of_issue,
                });
            }
            usecase_output.personal_id_numbers = Some(personal_id_numbers_output);
            Some(usecase_output)
        } else {
            println!("Execution fail");
            None
        };
    }
}

impl<A, B> QueryOnePersonByIdUsecaseInteractor<A, B>
where
    A: PersonDbGateway + Sync + Send,
    B: PersonalIdNumberGateway + Sync + Send,
{
    pub fn new(person_db_gateway: A, personal_id_number_db_gateway: B) -> Self {
        QueryOnePersonByIdUsecaseInteractor {
            person_db_gateway,
            personal_id_number_db_gateway,
        }
    }
}

impl ToUsecaseOutput<QueryPersonUsecaseOutput> for PersonDbResponse {
    fn to_usecase_output(self) -> QueryPersonUsecaseOutput {
        QueryPersonUsecaseOutput {
            id: self.id,
            first_name: self.first_name,
            middle_name: self.middle_name,
            last_name: self.last_name,
            date_of_birth: self.date_of_birth,
            place_of_birth: self.place_of_birth,
            email: self.email,
            phone: self.phone,
            personal_id_numbers: None,
        }
    }
}
