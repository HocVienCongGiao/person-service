use crate::ports::person::models::person_dbresponse::PersonCollection as PersonCollectionDbResponse;
use crate::ports::person_db_gateway::PersonDbGateway;
use crate::ports::personal_id_number::personal_id_number_db_gateway::PersonalIdNumberGateway;
use crate::usecases::query_one_person_by_id_usecase::QueryPersonUsecaseOutput;
use crate::usecases::query_one_personal_id_number_usecase::PersonalIdNumberUsecaseOutput;
use crate::usecases::ToUsecaseOutput;
use async_trait::async_trait;
use chrono::NaiveDate;
use uuid::Uuid;

pub struct QueryPersonCollectionUsecaseInteractor<A: PersonDbGateway, B: PersonalIdNumberGateway> {
    person_db_gateway: A,
    personal_id_number_db_gateway: B,
}

impl<A, B> QueryPersonCollectionUsecaseInteractor<A, B>
where
    A: PersonDbGateway + Sync + Send,
    B: PersonalIdNumberGateway + Sync + Send,
{
    pub fn new(person_db_gateway: A, personal_id_number_db_gateway: B) -> Self {
        QueryPersonCollectionUsecaseInteractor {
            person_db_gateway,
            personal_id_number_db_gateway,
        }
    }
}

#[async_trait]
pub trait QueryPersonCollectionUsecase {
    async fn execute(
        &self,
        request: QueryPersonCollectionUsecaseInput,
    ) -> QueryPersonCollectionUsecaseOutput;
}

#[async_trait]
impl<A, B> QueryPersonCollectionUsecase for QueryPersonCollectionUsecaseInteractor<A, B>
where
    A: PersonDbGateway + Sync + Send,
    B: PersonalIdNumberGateway + Sync + Send,
{
    async fn execute(
        &self,
        request: QueryPersonCollectionUsecaseInput,
    ) -> QueryPersonCollectionUsecaseOutput {
        let usecase_output = (*self)
            .person_db_gateway
            .find_collection_by(request.to_query_db_request())
            .await
            .to_usecase_output();

        let mut persons: Vec<QueryPersonUsecaseOutput> = Vec::new();
        for item in usecase_output.collection {
            let mut person: QueryPersonUsecaseOutput = item;
            let personal_id_numbers = (*self)
                .personal_id_number_db_gateway
                .find_collection_by_person_id(person.id)
                .await;

            let mut personal_id_numbers_output: Vec<PersonalIdNumberUsecaseOutput> = Vec::new();
            for personal_id_number in personal_id_numbers {
                personal_id_numbers_output.push(PersonalIdNumberUsecaseOutput {
                    id_number: personal_id_number.id_number,
                    code: personal_id_number.code,
                    date_of_issue: personal_id_number.date_of_issue,
                    place_of_issue: personal_id_number.place_of_issue,
                });
            }
            person.personal_id_numbers = Some(personal_id_numbers_output);

            persons.push(person);
        }

        // handle personal id number
        QueryPersonCollectionUsecaseOutput {
            collection: persons,
            has_more: usecase_output.has_more,
            total: usecase_output.total,
        }
    }
}

pub struct QueryPersonCollectionUsecaseInput {
    pub id: Option<Uuid>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub place_of_birth: Option<String>,
    // pub sort_request: Option<QueryPersonCollectionUsecaseInputSort>,
    pub offset: Option<i64>,
    pub count: Option<i64>,
}

pub struct QueryPersonCollectionUsecaseOutput {
    pub collection: Vec<QueryPersonUsecaseOutput>,
    pub has_more: Option<bool>,
    pub total: i64,
}

impl ToUsecaseOutput<QueryPersonCollectionUsecaseOutput> for PersonCollectionDbResponse {
    fn to_usecase_output(self) -> QueryPersonCollectionUsecaseOutput {
        let collection = self
            .collection
            .into_iter()
            .map(|person_db_response| person_db_response.to_usecase_output()) // fn in query_one_person_by_id_usecase
            .collect();
        QueryPersonCollectionUsecaseOutput {
            collection,
            has_more: self.has_more,
            total: self.total,
        }
    }
}
