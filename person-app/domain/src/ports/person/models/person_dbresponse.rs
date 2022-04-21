use crate::entities::educational_stage::EducationalStage;
use crate::entities::language::Language;
use crate::entities::title::Position;
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
    pub languages: Vec<Language>,
    pub educational_stages: Vec<EducationalStage>,
    pub position: Option<Position>,
}

pub struct PersonCollection {
    pub collection: Vec<Person>,
    pub has_more: Option<bool>,
    pub total: i64,
}
