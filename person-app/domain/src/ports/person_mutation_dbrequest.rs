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
    pub personal_id_number: Option<Vec<PersonalIdNumber>>,
    pub address: Option<String>,
}

pub struct PersonalIdNumber {
    pub id: Option<Uuid>,
    pub id_number: Option<String>,
    pub code: Option<String>,
    pub date_of_issue: Option<NaiveDate>,
    pub place_of_issue: Option<String>,
}
