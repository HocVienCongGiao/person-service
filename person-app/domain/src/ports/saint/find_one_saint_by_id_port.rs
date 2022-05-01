use crate::ports::saint::saint_db_gateway::SaintDbResponse;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait FindOneSaintByIdPort {
    async fn find_one_by_id(&self, id: Uuid) -> Option<SaintDbResponse>;
}
