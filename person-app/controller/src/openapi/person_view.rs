use crate::openapi::ToOpenApi;
use domain::usecases::create_person_usecase::CreatePersonUsecaseOutput;
use domain::usecases::person_usecase_shared_models::{
    PersonUsecaseSharedIdNumber, PersonUsecaseSharedIdNumberProvider,
};
use domain::usecases::query_one_personal_id_number_usecase::PersonalIdNumberUsecaseOutput;
use domain::usecases::query_one_personal_id_number_usecase::QueryPersonUsecaseOutput;
use hvcg_biography_openapi_person::models::{IdNumberProvider, PersonView, PersonalIdNumber};
use std::str::FromStr;

impl ToOpenApi<IdNumberProvider> for PersonUsecaseSharedIdNumberProvider {
    fn to_openapi(self) -> IdNumberProvider {
        match self {
            PersonUsecaseSharedIdNumberProvider::Passport => IdNumberProvider::PASSPORT,
            PersonUsecaseSharedIdNumberProvider::NationalId => IdNumberProvider::NATIONAL_ID,
        }
    }
}

impl ToOpenApi<PersonalIdNumber> for PersonUsecaseSharedIdNumber {
    fn to_openapi(self) -> PersonalIdNumber {
        PersonalIdNumber {
            id_number: self.id_number,
            id_number_provider: self.code.map(|provider| provider.to_openapi()),
            date_of_issue: self.date_of_issue,
            place_of_issue: self.place_of_issue,
        }
    }
}

impl ToOpenApi<PersonalIdNumber> for PersonalIdNumberUsecaseOutput {
    fn to_openapi(self) -> PersonalIdNumber {
        println!("test {:?}", self.id_number);
        PersonalIdNumber {
            id_number: self.id_number,
            id_number_provider: self.code.as_ref().map(|provider| {
                IdNumberProvider::from_str(provider).expect("Unable to convert to IdNumberProvider")
            }),
            date_of_issue: self.date_of_issue,
            place_of_issue: self.place_of_issue,
        }
    }
}

impl ToOpenApi<PersonView> for CreatePersonUsecaseOutput {
    fn to_openapi(self) -> PersonView {
        let mut personal_id_numbers: Vec<PersonalIdNumber> = Vec::new();
        if let Some(personal_id_numbers_usecase_output) = self.personal_id_numbers {
            for personal_id_number in personal_id_numbers_usecase_output {
                personal_id_numbers.push(personal_id_number.to_openapi())
            }
        }
        PersonView {
            id: self.person_id,
            date_of_birth: self.date_of_birth,
            place_of_birth: self.place_of_birth,
            email: self.email,
            phone: self.phone,
            name: Some(format!(
                "{} {} {}",
                self.last_name.unwrap_or_default(),
                self.middle_name.unwrap_or_default(),
                self.first_name.unwrap_or_default(),
            )),
            personal_id_numbers: Some(personal_id_numbers),
        }
    }
}

impl ToOpenApi<PersonView> for QueryPersonUsecaseOutput {
    fn to_openapi(self) -> PersonView {
        let mut personal_id_numbers: Vec<PersonalIdNumber> = Vec::new();
        if let Some(personal_id_numbers_usecase_output) = self.personal_id_numbers {
            for personal_id_number in personal_id_numbers_usecase_output {
                personal_id_numbers.push(personal_id_number.to_openapi())
            }
        }
        PersonView {
            id: self.id,
            date_of_birth: self.date_of_birth,
            place_of_birth: self.place_of_birth,
            email: self.email,
            phone: self.phone,
            name: Some(format!(
                "{} {} {}",
                self.first_name.unwrap(),
                self.middle_name.unwrap(),
                self.last_name.unwrap(),
            )),
            personal_id_numbers: Some(personal_id_numbers),
        }
    }
}
