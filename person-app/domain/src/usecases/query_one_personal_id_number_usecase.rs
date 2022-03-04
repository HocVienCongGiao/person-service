use chrono::NaiveDate;
use uuid::Uuid;

pub struct PersonalIdNumberUsecaseOutput {
    pub id_number: Option<String>,
    pub code: Option<String>,
    pub date_of_issue: Option<NaiveDate>,
    pub place_of_issue: Option<String>,
}

pub struct QueryPersonUsecaseOutput {
    pub id: Uuid,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub place_of_birth: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub personal_id_numbers: Option<Vec<PersonalIdNumberUsecaseOutput>>,
}
