use crate::ports::personal_id_number::models::personal_id_number_db_response::PersonalIdNumberDbResponse;
use uuid::Uuid;

pub mod educational_stage;
pub mod language;
pub mod nationality;
pub mod personal_id_number;
pub mod title;
pub mod vow_progress;

pub(crate) trait WithPolity<T> {
    fn with_polity(
        self,
        name: Option<String>,
        location_name: Option<String>,
        location_address: Option<String>,
        location_email: Option<String>,
    ) -> T;
}

pub(crate) trait WithChristianName<T> {
    fn with_christian_name(self, name: Option<String>) -> T;
}

pub(crate) trait WithPersonId<T> {
    fn with_person_id(self, person_id: Option<Uuid>) -> T;
}

pub(crate) trait WithPersonalIdNumbers<T> {
    fn with_personal_id_numbers(self, personal_id_numbers: Vec<PersonalIdNumberDbResponse>) -> T;
}
