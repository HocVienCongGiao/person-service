use crate::ports::personal_id_number::models::personal_id_number_db_response::PersonalIdNumberDbResponse;
use chrono::NaiveDate;
use uuid::Uuid;

pub struct Person {
    pub id: Uuid,
    pub personal_id_numbers: Option<Vec<PersonalIdNumberDbResponse>>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub place_of_birth: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}
