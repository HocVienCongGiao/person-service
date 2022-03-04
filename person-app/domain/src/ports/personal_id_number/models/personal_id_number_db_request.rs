use chrono::NaiveDate;
use uuid::Uuid;

pub struct PersonalIdNumberDbRequest {
    pub id: Option<Uuid>,
    pub id_number: Option<String>,
    pub code: Option<String>,
    pub date_of_issue: Option<NaiveDate>,
    pub place_of_issue: Option<String>,
    pub offset: Option<i64>,
    pub count: Option<i64>,
}
