use crate::entities::person::Person;
use crate::ports::find_one_person_by_id_port::FindOnePersonByIdPort;
use crate::ports::person::delete_one_person_by_id_port::DeleteOnePersonByIdPort;
use crate::ports::person::find_person_collection_port::FindPersonCollectionPort;
use crate::ports::person::insert_person_port::InsertPersonPort;
use crate::ports::person::models::person_dbrequest::PersonQuery as PersonQueryDbRequest;
use crate::ports::person::models::person_mutation_dbrequest::Person as PersonMutationDbRequest;
use crate::ports::person::update_person_by_id_port::UpdateOnePersonByIdPort;
use crate::ports::personal_id_number::models::personal_id_number_db_request::PersonalIdNumber as PersonalIdNumberDbRequest;
use crate::usecases::query_person_collection_usecase::QueryPersonCollectionUsecaseInput;
use async_trait::async_trait;

#[async_trait]
pub trait PersonDbGateway:
    InsertPersonPort
    + FindOnePersonByIdPort
    + DeleteOnePersonByIdPort
    + UpdateOnePersonByIdPort
    + FindPersonCollectionPort
{
}

impl Person {
    pub fn to_mutation_db_request(&self) -> PersonMutationDbRequest {
        let mut personal_id_numbers: Vec<PersonalIdNumberDbRequest> = Vec::new();
        if let Some(personal_id_numbers_entity) = self.personal_id_numbers.clone() {
            for personal_id_number in personal_id_numbers_entity {
                personal_id_numbers.push(PersonalIdNumberDbRequest {
                    id: personal_id_number.id,
                    id_number: personal_id_number.id_number,
                    code: personal_id_number.code.clone().map(|code| code.to_string()),
                    date_of_issue: personal_id_number.date_of_issue,
                    place_of_issue: personal_id_number.place_of_issue,
                });
            }
        }

        PersonMutationDbRequest {
            id: self.id,
            first_name: self.first_name.clone(),
            middle_name: self.middle_name.clone(),
            last_name: self.last_name.clone(),
            date_of_birth: self.date_of_birth,
            place_of_birth: self.place_of_birth.clone(),
            email: self.email.clone(),
            phone: self.phone.clone(),
            nationality: self.nationality.clone(),
            race: self.race.clone(),
            personal_id_numbers: Some(personal_id_numbers),
            address: self.address.clone(),
            saint_ids: self.saint_ids.clone(),
            languages: self.languages.clone(),
            educational_stages: self.educational_stages.clone(),
            position: self.position.clone(),
        }
    }
}

impl QueryPersonCollectionUsecaseInput {
    pub fn to_query_db_request(&self) -> PersonQueryDbRequest {
        PersonQueryDbRequest {
            id: self.id,
            name: self.name.clone(),
            email: self.email.clone(),
            phone: self.phone.clone(),
            date_of_birth: self.date_of_birth,
            place_of_birth: self.place_of_birth.clone(),
            offset: self.offset,
            count: self.count,
        }
    }
}
