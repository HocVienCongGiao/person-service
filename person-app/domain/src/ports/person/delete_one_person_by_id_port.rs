use crate::ports::DbError;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait DeleteOnePersonByIdPort {
    async fn delete_one_by_id(&self, id: Uuid) -> Result<(), DbError>;
}
