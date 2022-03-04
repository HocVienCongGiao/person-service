use async_trait::async_trait;
use domain::ports::personal_id_number::find_personal_id_number_collection_port::FindPersonalIdNumberCollectionPort;
use domain::ports::personal_id_number::models::personal_id_number_db_response::PersonalIdNumberDbResponse;
use domain::ports::personal_id_number::personal_id_number_db_gateway::PersonalIdNumberGateway;
use tokio_postgres::Client;
use uuid::Uuid;

pub struct PersonalIdNumberRepository {
    pub client: Client,
}

#[async_trait]
impl PersonalIdNumberGateway for PersonalIdNumberRepository {}
