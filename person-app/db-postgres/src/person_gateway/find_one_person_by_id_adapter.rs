use crate::db_column;
use crate::person_gateway::repository::PersonRepository;
use async_trait::async_trait;
use domain::ports::find_one_person_by_id_port::FindOnePersonByIdPort;
use domain::ports::person::models::person_dbresponse::Person as PersonDbResponse;
use tokio_postgres::types::ToSql;
use tokio_postgres::Row;
use uuid::Uuid;

#[async_trait]
impl FindOnePersonByIdPort for PersonRepository {
    async fn find_one_by_id(&self, id: Uuid) -> Option<PersonDbResponse> {
        let stmt = (*self)
            .client
            .prepare("SELECT * FROM person__person_view WHERE id = $1")
            .await
            .unwrap();

        // let stmt = block_on(stmt_future).unwrap();
        let name_param: &[&(dyn ToSql + Sync)] = &[&id];
        let row = (*self).client.query_one(&stmt, name_param).await;
        return match row {
            Ok(row) => Some(from_pg_row_to_person_db_response(row)),
            Err(e) => {
                eprintln!("Error: {:?}", e);
                None
            }
        };
    }
}

pub(crate) fn from_pg_row_to_person_db_response(row: Row) -> PersonDbResponse {
    PersonDbResponse {
        id: db_column::get_uuid(&row, "id"),
        first_name: Some(db_column::get_string(&row, "first_name")),
        middle_name: Some(db_column::get_string(&row, "middle_name")),
        last_name: Some(db_column::get_string(&row, "last_name")),
        date_of_birth: Some(db_column::get_date(&row, "date_of_birth")),
        place_of_birth: Some(db_column::get_string(&row, "place_of_birth")),
        email: Some(db_column::get_string(&row, "email")),
        phone: Some(db_column::get_string(&row, "phone")),
        personal_id_numbers: None,
    }
}
