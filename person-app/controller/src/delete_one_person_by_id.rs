use uuid::Uuid;
use db_postgres::person_gateway::repository::PersonRepository;
use db_postgres::personal_id_number_gateway::repository::PersonalIdNumberRepository;
use domain::usecases::delete_person_usecase::DeleteOnePersonByIdUsecaseInteractor;
use domain::usecases::UsecaseError;

pub async fn from_uuid(id: Uuid) -> Result<(), UsecaseError> {
    // Init dependencies
    let client = db_postgres::connect().await;
    let person_repository = PersonRepository { client };

    let personal_id_number_client = db_postgres::connect().await;
    let personal_id_number_repository = PersonalIdNumberRepository {
        client: personal_id_number_client,
    };

    let delete_one_person_usecase_output = DeleteOnePersonByIdUsecaseInteractor::new(person_repository).execute(id).await;
    Ok(())
}