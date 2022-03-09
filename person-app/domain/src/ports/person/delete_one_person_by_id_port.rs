use async_trait::async_trait;
use uuid::Uuid;
use crate::ports::DbError;

#[async_trait]
pub trait DeleteOnePersonByIdPort {
    async fn delete_one_by_id(&self, id: Uuid) -> Result<(), DbError>;
}