use chrono::NaiveDate;
use uuid::Uuid;

pub struct PersonalIdNumberDbResponse {
    pub id: Uuid,
    pub person_id: Option<Uuid>,
    pub id_number: Option<String>,
    pub code: Option<String>,
    pub date_of_issue: Option<NaiveDate>,
    pub place_of_issue: Option<String>,
}
