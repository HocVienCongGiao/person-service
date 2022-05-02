use async_trait::async_trait;
use domain::ports::saint::saint_db_gateway::SaintDbGateway;
use tokio_postgres::Client;

pub struct SaintRepository {
    pub client: Client,
}

#[async_trait]
impl SaintDbGateway for SaintRepository {}
