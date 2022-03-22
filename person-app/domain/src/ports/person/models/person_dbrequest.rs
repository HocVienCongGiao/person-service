use chrono::NaiveDate;
use uuid::Uuid;

pub struct PersonQuery {
    pub id: Option<Uuid>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub place_of_birth: Option<String>,
    // pub sort_request: Option<StudentSort>,
    pub offset: Option<i64>,
    pub count: Option<i64>,
}
