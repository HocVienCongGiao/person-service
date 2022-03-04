use crate::entities::person::Person;
use crate::ports::find_one_person_by_id_port::FindOnePersonByIdPort;
use crate::ports::insert_person_port::InsertPersonPort;
use crate::ports::person_mutation_dbrequest::Person as PersonMutationDbRequest;
use crate::ports::person_mutation_dbrequest::PersonalIdNumber;
use async_trait::async_trait;

#[async_trait]
pub trait PersonDbGateway: InsertPersonPort + FindOnePersonByIdPort {}

impl Person {
    pub fn to_mutation_db_request(&self) -> PersonMutationDbRequest {
        let mut personal_id_numbers: Vec<PersonalIdNumber> = Vec::new();
        for personal_id_number in self.personal_id_number.clone().unwrap() {
            personal_id_numbers.push(PersonalIdNumber {
                id: personal_id_number.id,
                id_number: personal_id_number.id_number,
                code: personal_id_number.code.clone().map(|code| code.to_string()),
                date_of_issue: personal_id_number.date_of_issue,
                place_of_issue: personal_id_number.place_of_issue,
            });
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
            nationality: self
                .nationality
                .clone()
                .map(|nationality| nationality.to_string()),
            race: self.race.clone(),
            personal_id_number: Some(personal_id_numbers),
            address: self.address.clone(),
        }
    }
}
