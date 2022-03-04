use chrono::NaiveDate;
use tokio_postgres::Row;
use uuid::Uuid;

pub fn get_uuid(row: &Row, col: &str) -> Uuid {
    row.get::<&str, Uuid>(col)
}

pub fn get_uuid_collection(row: &Row, col: &str) -> Vec<Uuid> {
    row.get::<&str, Vec<Uuid>>(col)
}

pub fn get_string(row: &Row, col: &str) -> String {
    row.get::<&str, String>(col)
}

pub fn get_date(row: &Row, col: &str) -> NaiveDate {
    row.get::<&str, NaiveDate>(col)
}
