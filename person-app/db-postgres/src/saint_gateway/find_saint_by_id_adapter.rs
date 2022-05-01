use crate::db_column;
use crate::saint_gateway::repository::SaintRepository;
use async_trait::async_trait;
use domain::ports::saint::find_one_saint_by_id_port::FindOneSaintByIdPort;
use domain::ports::saint::saint_db_gateway::SaintDbResponse;
use tokio_postgres::types::ToSql;
use uuid::Uuid;

#[async_trait]
impl FindOneSaintByIdPort for SaintRepository {
    async fn find_one_by_id(&self, id: Uuid) -> Option<SaintDbResponse> {
        let stmt = (*self)
            .client
            .prepare("SELECT * FROM saint__saint_display_name WHERE id = $1")
            .await
            .unwrap();

        // let stmt = block_on(stmt_future).unwrap();
        let name_param: &[&(dyn ToSql + Sync)] = &[&id];
        let row = (*self).client.query_one(&stmt, name_param).await.unwrap();

        Some(SaintDbResponse {
            id,
            display_name: Some(db_column::get_string(&row, "display_name")),
        })
    }
}
