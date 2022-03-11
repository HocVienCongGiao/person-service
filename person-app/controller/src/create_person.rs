use crate::openapi::ToOpenApi;
use crate::openapi::ToUsecaseInput;
use crate::PersonViewOpenApi;
use db_postgres::person_gateway::repository::PersonRepository;
use domain::usecases::create_person_usecase::CreatePersonUsecase;
use domain::usecases::create_person_usecase::{
    CreatePersonUsecaseInput, CreatePersonUsecaseInteractor,
};
use domain::usecases::person_usecase_shared_models::{
    PersonUsecaseSharedIdNumber, PersonUsecaseSharedIdNumberProvider,
    PersonUsecaseSharedNationality,
};
use domain::usecases::UsecaseError;
use hvcg_biography_openapi_person::models::{IdNumberProvider, Nationality, PersonUpsert as PersonUpsertOpenApi, PersonView};
use domain::usecases::update_one_person_by_id_usecase::UpdatePersonUsecaseOutput;

pub async fn from_openapi(person: PersonUpsertOpenApi) -> Result<PersonViewOpenApi, UsecaseError> {
    // Init dependencies
    let client = db_postgres::connect().await;
    let person_repository = PersonRepository { client };

    // Inject dependencies to Interactor and invoke func
    let result = CreatePersonUsecaseInteractor::new(person_repository)
        .execute(person.to_usecase_input())
        .await;
    result.map(|res| res.to_openapi())
}

impl ToUsecaseInput<CreatePersonUsecaseInput> for PersonUpsertOpenApi {
    fn to_usecase_input(self) -> CreatePersonUsecaseInput {
        let mut nationality: Option<PersonUsecaseSharedNationality> = None;
        let nationality_openapi = self.nationality;
        if let Some(nationality_openapi) = nationality_openapi {
            nationality = Some(match nationality_openapi {
                Nationality::VIETNAMESE => PersonUsecaseSharedNationality::Vietnamese,
                Nationality::AMERICAN => PersonUsecaseSharedNationality::American,
                Nationality::CHINESE => PersonUsecaseSharedNationality::Chinese,
                Nationality::FRECH => PersonUsecaseSharedNationality::French,
                Nationality::BRITISH => PersonUsecaseSharedNationality::British,
            })
        }

        // convert openapi personal id numbers list to shared models id numbers list list
        let mut personal_id_numbers: Vec<PersonUsecaseSharedIdNumber> = Vec::new();
        let mut provider: Option<PersonUsecaseSharedIdNumberProvider>;
        for pin in self.personal_id_numbers.unwrap() {
            provider = Some(match pin.id_number_provider.unwrap() {
                IdNumberProvider::NATIONAL_ID => PersonUsecaseSharedIdNumberProvider::NationalId,
                IdNumberProvider::PASSPORT => PersonUsecaseSharedIdNumberProvider::Passport,
            });
            personal_id_numbers.push(PersonUsecaseSharedIdNumber {
                id_number: pin.id_number,
                code: provider,
                date_of_issue: pin.date_of_issue,
                place_of_issue: pin.place_of_issue,
            })
        }

        CreatePersonUsecaseInput {
            first_name: self.first_name.clone(),
            middle_name: self.middle_name.clone(),
            last_name: self.last_name.clone(),
            date_of_birth: self.date_of_birth,
            place_of_birth: self.place_of_birth.clone(),
            email: self.email.clone(),
            phone: self.phone,
            address: self.address,
            nationality,
            race: self.race,
            personal_id_number: Some(personal_id_numbers),
        }
    }
}
