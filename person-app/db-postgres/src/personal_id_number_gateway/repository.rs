use async_trait::async_trait;
use domain::ports::personal_id_number::personal_id_number_db_gateway::PersonalIdNumberGateway;
use tokio_postgres::Client;

pub struct PersonalIdNumberRepository {
    pub client: Client,
}

#[async_trait]
impl PersonalIdNumberGateway for PersonalIdNumberRepository {}
