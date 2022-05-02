use chrono::NaiveDate;
use tokio_postgres::{Error, Row};
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

pub fn get_boolean(row: &Row, col: &str) -> bool {
    row.get::<&str, bool>(col)
}

pub fn get_result_of_string(row: &Row, col: &str) -> Option<String> {
    match row.try_get::<&str, String>(col) {
        Ok(string) => Some(string),
        _ => None,
    }
}

pub fn get_date(row: &Row, col: &str) -> NaiveDate {
    row.get::<&str, NaiveDate>(col)
}
