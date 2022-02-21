use crate::openapi::ToOpenApi;
use domain::usecases::create_person_usecase::CreatePersonUsecaseOutput;
use hvcg_biography_openapi_person::models::PersonView;

impl ToOpenApi<PersonView> for CreatePersonUsecaseOutput {
    fn to_openapi(self) -> PersonView {
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
            // TODO
            personal_id_numbers: None,
        }
    }
}
