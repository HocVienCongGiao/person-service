use async_trait::async_trait;
use tokio_postgres::types::ToSql;
use tokio_postgres::Row;
use uuid::Uuid;

use crate::db_column;
use domain::ports::personal_id_number::find_personal_id_number_collection_port::FindPersonalIdNumberCollectionPort;
use domain::ports::personal_id_number::models::personal_id_number_db_response::PersonalIdNumberDbResponse;

use crate::personal_id_number_gateway::repository::PersonalIdNumberRepository;

#[async_trait]
impl FindPersonalIdNumberCollectionPort for PersonalIdNumberRepository {
    async fn find_collection_by_person_id(
        &self,
        person_id: Uuid,
    ) -> Vec<PersonalIdNumberDbResponse> {
        let stmt = (*self)
            .client
            .prepare("SELECT * FROM person__personal_id_number_view WHERE person_id = $1")
            .await
            .unwrap();

        let name_param: &[&(dyn ToSql + Sync)] = &[&person_id];
        let rows = (*self).client.query(&stmt, name_param).await;
        let collection: Vec<PersonalIdNumberDbResponse>;
        if let Ok(result) = rows {
            println!("{:?}", result.len());
            collection = result
                .into_iter()
                .map(from_pg_row_personal_id_number_db_response)
                .collect();
        } else {
            collection = vec![];
        }
        collection
    }
}

pub(crate) fn from_pg_row_personal_id_number_db_response(row: Row) -> PersonalIdNumberDbResponse {
    println!("turn to db response {:?}", db_column::get_uuid(&row, "id"));
    PersonalIdNumberDbResponse {
        id: db_column::get_uuid(&row, "id"),
        person_id: Some(db_column::get_uuid(&row, "person_id")),
        id_number: Some(db_column::get_string(&row, "person_id_number")),
        code: Some(db_column::get_string(&row, "code")),
        date_of_issue: Some(db_column::get_date(&row, "date_of_issue")),
        place_of_issue: Some(db_column::get_string(&row, "place_of_issue")),
    }
}
