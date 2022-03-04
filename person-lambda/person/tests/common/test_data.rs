use chrono::NaiveDate;
use hvcg_biography_openapi_person::models::{
    IdNumberProvider, Nationality, PersonUpsert, PersonView, PersonalIdNumber,
};
use std::str::FromStr;
use uuid::Uuid;

pub fn prepare_person_view_openapi(person_uuid: Option<Uuid>) -> PersonView {
    let personal_id_numbers_openapi = vec![PersonalIdNumber {
        id_number: Some("837837655558".to_string()),
        id_number_provider: Some(IdNumberProvider::NATIONAL_ID),
        date_of_issue: Some(NaiveDate::from_str("2011-05-05").unwrap()),
        place_of_issue: Some("TP.HCM".to_string()),
    }];
    PersonView {
        id: person_uuid
            .unwrap_or_else(|| Uuid::from_str("53f549b9-99bf-4e12-88e3-c2f868953283").unwrap()),
        name: Some("Nguyễn Hữu Chiến".to_string()),
        date_of_birth: Some(NaiveDate::from_str("1983-05-16").unwrap()),
        place_of_birth: Some("Trà Vinh".to_string()),
        email: Some("binh@sunrise.vn".to_string()),
        phone: Some("+84 1228019700".to_string()),
        personal_id_numbers: Some(personal_id_numbers_openapi),
    }
}

pub fn prepare_person_upsert_openapi() -> PersonUpsert {
    let personal_id_numbers_openapi = vec![PersonalIdNumber {
        id_number: Some("837837655558".to_string()),
        id_number_provider: Some(IdNumberProvider::NATIONAL_ID),
        date_of_issue: Some(NaiveDate::from_str("2011-05-05").unwrap()),
        place_of_issue: Some("TP.HCM".to_string()),
    }];
    PersonUpsert {
        first_name: Some("Chiến".to_string()),
        middle_name: Some("Hữu".to_string()),
        last_name: Some("Nguyễn".to_string()),
        date_of_birth: Some(NaiveDate::from_str("1983-05-16").unwrap()),
        place_of_birth: Some("Trà Vinh".to_string()),
        email: Some("binh@sunrise.vn".to_string()),
        phone: Some("+84 1228019700".to_string()),
        address: Some("1000 CMT8 phường 3 quận Tân Bình, TP HCM".to_string()),
        nationality: Some(Nationality::VIETNAMESE),
        race: Some("Kinh".to_string()),
        personal_id_numbers: Some(personal_id_numbers_openapi),
    }
}
