use chrono::NaiveDate;
use hvcg_biography_openapi_person::models::{
    EducationalLevel, EducationalStage, ForeignLanguageLevel, IdNumberProvider, Language,
    Nationality, PersonUpsert, PersonUpsertPosition, PersonView, PersonViewCollection,
    PersonalIdNumber, Title, VowProgress,
};
use std::str::FromStr;
use uuid::Uuid;

pub fn prepare_person_view_openapi(
    person_uuid: Option<Uuid>,
    personal_id_number: Option<String>,
) -> PersonView {
    let personal_id_numbers_openapi = vec![PersonalIdNumber {
        id_number: Some(personal_id_number.unwrap_or_else(|| "837837655555".to_string())),
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
        id_number: Some("837837655555".to_string()),
        id_number_provider: Some(IdNumberProvider::NATIONAL_ID),
        date_of_issue: Some(NaiveDate::from_str("2011-05-05").unwrap()),
        place_of_issue: Some("TP.HCM".to_string()),
    }];
    let languages = vec![Language {
        name: Some("ENGLISH".to_string()),
        level: Some(ForeignLanguageLevel::BEGINNER),
    }];
    let educational_stages = vec![EducationalStage {
        educational_level: EducationalLevel::HIGH_SCHOOL,
        school_name: "Nguyễn Du".to_string(),
        major: None,
        graduate_year: Some(2000_f64),
    }];
    let position = PersonUpsertPosition {
        name: Title::PRIEST,
        period: Some(VowProgress::SIMPLE_VOW),
        parish: Some(Uuid::from_str("369769b1-96ee-4e11-95e9-a9ed1409c043").unwrap()),
    };
    PersonUpsert {
        first_name: Some("Chiến".to_string()),
        middle_name: Some("Hữu".to_string()),
        last_name: Some("Nguyễn".to_string()),
        saint_id_array: Some(vec![
            Uuid::from_str("40e6215d-b5c6-4896-987c-f30f3678f608").unwrap()
        ]),
        date_of_birth: Some(NaiveDate::from_str("1983-05-16").unwrap()),
        place_of_birth: Some("Trà Vinh".to_string()),
        email: Some("binh@sunrise.vn".to_string()),
        phone: Some("+84 1228019700".to_string()),
        address: Some("1000 CMT8 phường 3 quận Tân Bình, TP HCM".to_string()),
        nationality: Some(Nationality::VIETNAMESE),
        race: Some("Kinh".to_string()),
        personal_id_numbers: Some(personal_id_numbers_openapi),
        language: Some(languages),
        education_stage: Some(educational_stages),
        position: Some(position),
    }
}
