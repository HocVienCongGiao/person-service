use chrono::NaiveDate;

pub struct PersonalIdNumberUsecaseOutput {
    pub id_number: Option<String>,
    pub code: Option<String>,
    pub date_of_issue: Option<NaiveDate>,
    pub place_of_issue: Option<String>,
}
