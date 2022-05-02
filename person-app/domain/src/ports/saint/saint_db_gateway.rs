use crate::ports::saint::find_one_saint_by_id_port::FindOneSaintByIdPort;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait SaintDbGateway: FindOneSaintByIdPort {}

pub struct SaintDbResponse {
    pub id: Uuid,
    pub display_name: Option<String>,
}
