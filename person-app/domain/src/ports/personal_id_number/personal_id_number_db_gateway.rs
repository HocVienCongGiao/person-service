use crate::ports::personal_id_number::find_personal_id_number_collection_port::FindPersonalIdNumberCollectionPort;
use async_trait::async_trait;

#[async_trait]
pub trait PersonalIdNumberGateway: FindPersonalIdNumberCollectionPort {}
