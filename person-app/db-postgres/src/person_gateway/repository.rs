use async_trait::async_trait;
use domain::ports::person_db_gateway::PersonDbGateway;
use tokio_postgres::Client;

pub struct PersonRepository {
    pub client: Client,
}

#[async_trait]
impl PersonDbGateway for PersonRepository {}
