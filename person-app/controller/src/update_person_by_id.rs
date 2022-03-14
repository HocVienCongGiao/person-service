use crate::openapi::ToOpenApi;
use crate::openapi::ToUsecaseInput;
use crate::{PersonUpsertOpenApi, PersonViewOpenApi};
use db_postgres::person_gateway::repository::PersonRepository;
use db_postgres::personal_id_number_gateway::repository::PersonalIdNumberRepository;
use domain::usecases::person_usecase_shared_models::PersonUsecaseSharedNationality;
use domain::usecases::update_one_person_by_id_usecase::{
    UpdatePersonUsecase, UpdatePersonUsecaseInput, UpdatePersonUsecaseInteractor,
};
use domain::usecases::UsecaseError;
use hvcg_biography_openapi_person::models::Nationality;
use uuid::Uuid;

pub async fn from_openapi(
    person_id: Uuid,
    person: PersonUpsertOpenApi,
) -> Result<PersonViewOpenApi, UsecaseError> {
    // Init dependencies
    let client = db_postgres::connect().await;
    let person_repository = PersonRepository { client };

    let personal_id_number_client = db_postgres::connect().await;
    let personal_id_number_repository = PersonalIdNumberRepository {
        client: personal_id_number_client,
    };

    let mut usecase_input: UpdatePersonUsecaseInput = person.to_usecase_input();
    usecase_input = usecase_input.with_person_id(person_id);

    // Inject dependencies to Interactor and invoke func
    let result =
        UpdatePersonUsecaseInteractor::new(person_repository, personal_id_number_repository)
            .execute(person_id, usecase_input)
            .await;
    result.map(|res| res.to_openapi())
}

impl ToUsecaseInput<UpdatePersonUsecaseInput> for PersonUpsertOpenApi {
    fn to_usecase_input(self) -> UpdatePersonUsecaseInput {
        let mut nationality: Option<PersonUsecaseSharedNationality> = None;
        if let Some(nationality_openapi) = self.nationality {
            nationality = Some(match nationality_openapi {
                Nationality::VIETNAMESE => PersonUsecaseSharedNationality::Vietnamese,
                Nationality::CHINESE => PersonUsecaseSharedNationality::Chinese,
                Nationality::AMERICAN => PersonUsecaseSharedNationality::American,
                // TODO: FRECH -> FRENCH
                Nationality::FRECH => PersonUsecaseSharedNationality::French,
                Nationality::BRITISH => PersonUsecaseSharedNationality::British,
            })
        }
        UpdatePersonUsecaseInput {
            person_id: None, // update later
            first_name: self.first_name.clone(),
            middle_name: self.middle_name.clone(),
            last_name: self.last_name.clone(),
            date_of_birth: self.date_of_birth,
            place_of_birth: self.place_of_birth.clone(),
            email: self.email.clone(),
            phone: self.phone,
            address: None,
            nationality,
            race: None,
            personal_id_number: None,
        }
    }
}
