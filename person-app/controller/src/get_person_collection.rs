use crate::openapi::ToOpenApi;
use crate::PersonCollectionQuery;
use db_postgres::person_gateway::repository::PersonRepository;
use db_postgres::personal_id_number_gateway::repository::PersonalIdNumberRepository;
use domain::usecases::query_person_collection_usecase::{
    QueryPersonCollectionUsecase, QueryPersonCollectionUsecaseInput,
    QueryPersonCollectionUsecaseInteractor,
};
use hvcg_biography_openapi_person::models::PersonViewCollection;
pub(crate) async fn from_usecase_input(
    request: QueryPersonCollectionUsecaseInput,
) -> PersonViewCollection {
    let client = db_postgres::connect().await;
    let person_repo = PersonRepository { client };

    let personal_id_number_client = db_postgres::connect().await;
    let personal_id_number_repo = PersonalIdNumberRepository {
        client: personal_id_number_client,
    };

    let query_persons_usecase_output =
        QueryPersonCollectionUsecaseInteractor::new(person_repo, personal_id_number_repo)
            .execute(request)
            .await;

    query_persons_usecase_output.to_openapi()
    // PersonViewCollection {
    //     persons: vec![],
    //     has_more: None,
    //     total: None,
    // }
}

impl PersonCollectionQuery {
    pub fn to_usecase_input(&self) -> QueryPersonCollectionUsecaseInput {
        QueryPersonCollectionUsecaseInput {
            id: None,
            name: self.name.clone(),
            email: self.email.clone(),
            phone: self.phone.clone(),
            date_of_birth: self.date_of_birth,
            place_of_birth: self.place_of_birth.clone(),
            offset: self.offset,
            count: self.count,
        }
    }
}
