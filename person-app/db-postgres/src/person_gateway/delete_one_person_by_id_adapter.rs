use crate::person_gateway::repository::PersonRepository;
use async_trait::async_trait;
use domain::ports::person::delete_one_person_by_id_port::DeleteOnePersonByIdPort;
use domain::ports::DbError;
use tokio_postgres::types::ToSql;
use uuid::Uuid;

#[async_trait]
impl DeleteOnePersonByIdPort for PersonRepository {
    async fn delete_one_by_id(&self, id: Uuid) -> Result<(), DbError> {
        let stmt = (*self)
            .client
            .prepare("DELETE FROM public.person__person WHERE id = $1")
            .await
            .unwrap();

        let name_param: &[&(dyn ToSql + Sync)] = &[&id];
        let row = (*self).client.execute(&stmt, name_param).await;
        return match row {
            Ok(_) => Ok(()),
            Err(error) => Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            )),
        };
    }
}
