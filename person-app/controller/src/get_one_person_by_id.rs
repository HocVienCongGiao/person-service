use crate::openapi::ToOpenApi;
use db_postgres::person_gateway::repository::PersonRepository;
use db_postgres::personal_id_number_gateway::repository::PersonalIdNumberRepository;
use domain::usecases::query_one_person_by_id_usecase::{
    QueryOnePersonByIdUsecase, QueryOnePersonByIdUsecaseInteractor,
};
use hvcg_biography_openapi_person::models::PersonView;
use uuid::Uuid;

pub(crate) async fn from_uuid(id: Uuid) -> Option<PersonView> {
    // Init dependencies
    let client = db_postgres::connect().await;
    let person_repository = PersonRepository { client };

    let personal_id_number_client = db_postgres::connect().await;
    let personal_id_number_repository = PersonalIdNumberRepository {
        client: personal_id_number_client,
    };

    // Inject dependencies to Interactor and invoke func
    let query_one_person_usecase_output =
        QueryOnePersonByIdUsecaseInteractor::new(person_repository, personal_id_number_repository)
            .execute(id)
            .await;

    query_one_person_usecase_output
        .map(|query_one_person_usecase_output| query_one_person_usecase_output.to_openapi())
}
