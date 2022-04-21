use crate::entities::educational_stage::EducationalStage;
use crate::entities::language::Language;
use crate::entities::title::Position;
use crate::ports::personal_id_number::models::personal_id_number_db_request::PersonalIdNumber as PersonalIdNumberDbRequest;
use chrono::NaiveDate;
use uuid::Uuid;

pub struct Person {
    pub id: Option<Uuid>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub place_of_birth: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub nationality: Option<String>,
    pub race: Option<String>,
    pub personal_id_numbers: Option<Vec<PersonalIdNumberDbRequest>>,
    pub address: Option<String>,
    pub saint_ids: Option<Vec<Uuid>>,
    pub languages: Option<Vec<Language>>,
    pub educational_stages: Option<Vec<EducationalStage>>,
    pub position: Option<Position>,
}
