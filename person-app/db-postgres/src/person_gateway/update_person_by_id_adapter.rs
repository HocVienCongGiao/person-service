use crate::person_gateway::repository::PersonRepository;
use async_trait::async_trait;
use chrono::NaiveDate;
use domain::ports::person::update_person_by_id_port::UpdateOnePersonByIdPort;
use domain::ports::person_dbresponse::Person as PersonDbResponse;
use domain::ports::person_mutation_dbrequest::Person as PersonMutationDbRequest;
use domain::ports::DbError;
use tokio_postgres::types::ToSql;
use tokio_postgres::{Error, Transaction};
use uuid::Uuid;
use domain::ports::personal_id_number::models::personal_id_number_db_response::PersonalIdNumberDbResponse;
use domain::ports::personal_id_number::models::personal_id_number_db_response::PersonalIdNumberDbResponse;

pub(crate) async fn update_date_of_birth(
    transaction: &Transaction<'_>,
    id: Uuid,
    date_of_birth: NaiveDate,
) -> Result<u64, Error> {
    let stmt = (*transaction)
        .prepare("UPDATE public.person__person_date_of_birth SET date_of_birth = $2 WHERE id = $1")
        .await
        .unwrap();

    let params: &[&(dyn ToSql + Sync)] = &[&id, &date_of_birth];
    transaction.execute(&stmt, params).await
}

pub(crate) async fn update_person_info(
    transaction: &Transaction<'_>,
    id: Uuid,
    entity_name: String,
    field_name: String,
    value: String,
) -> Result<u64, Error> {
    let statement = format!(
        "UPDATE public.person__{}_{} SET {} = $2 WHERE id = $1",
        entity_name, field_name, field_name
    );
    let stmt = (*transaction).prepare(&statement).await.unwrap();

    let params: &[&(dyn ToSql + Sync)] = &[&id, &value];
    transaction.execute(&stmt, params).await
}

pub(crate) async fn update_personal_id_number_info(
    transaction: &Transaction<'_>,
    personal_id_number_id: Uuid,
    table_name: String,
    field_name: String,
    value: String,
) -> Result<u64, Error> {
    let statement = format!(
        "UPDATE public.person__person_id_number_{} (id, {}) VAlUES ($1, $2)",
        table_name, field_name
    );
    let stmt = (*transaction).prepare(&statement).await.unwrap();

    let params: &[&(dyn ToSql + Sync)] = &[&personal_id_number_id, &value];
    transaction.execute(&stmt, params).await
}

// add personal_id_number id to mutation db request to update
pub(crate) async fn save_personal_id_number(
    transaction: &Transaction<'_>,
    person_id: Uuid,
    personal_id_number_id: Uuid,
    personal_id_number: String,
) -> Result<u64, Error> {
    let stmt = (*transaction)
        .prepare(
            "UPDATE public.person__person_id_number (id, person_id, person_id_number) VAlUES ($1, $2, $3)",
        )
        .await
        .unwrap();

    let params: &[&(dyn ToSql + Sync)] = &[&personal_id_number_id, &person_id, &personal_id_number];
    transaction.execute(&stmt, params).await
}

#[async_trait]
impl UpdateOnePersonByIdPort for PersonRepository {
    async fn update_one_by_id(
        &mut self,
        db_request: PersonMutationDbRequest,
    ) -> Result<PersonDbResponse, DbError> {
        let mut result: Result<u64, Error> = Ok(1_u64);

        let transaction = (*self)
            .client
            .transaction()
            .await
            .map_err(|error| DbError::UnknownError(error.into_source().unwrap().to_string()))?;

        let person_id = db_request.id.unwrap();

        // update first name
        let first_name = db_request.first_name.unwrap();
        result = update_person_info(
            &transaction,
            person_id,
            "person".to_string(),
            "first_name".to_string(),
            first_name.clone(),
        )
        .await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }

        // update last name
        let last_name = db_request.last_name.unwrap();
        result = update_person_info(
            &transaction,
            person_id,
            "person".to_string(),
            "last_name".to_string(),
            last_name.clone(),
        )
        .await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }

        // update middle name
        let middle_name = db_request.middle_name.unwrap();
        result = update_person_info(
            &transaction,
            person_id,
            "person".to_string(),
            "middle_name".to_string(),
            middle_name.clone(),
        )
        .await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }

        // update date of birth
        let date_of_birth = db_request.date_of_birth.unwrap();
        result = update_date_of_birth(&transaction, person_id, date_of_birth).await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }

        // update email
        let email = db_request.email.unwrap();
        result = update_person_info(
            &transaction,
            person_id,
            "person".to_string(),
            "email".to_string(),
            email.clone(),
        )
        .await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }

        // update phone
        let phone = db_request.phone.unwrap();
        result = update_person_info(
            &transaction,
            person_id,
            "person".to_string(),
            "phone".to_string(),
            phone.clone(),
        )
        .await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }

        // update place of birth
        let place_of_birth = db_request.place_of_birth.unwrap();
        result = update_person_info(
            &transaction,
            person_id,
            "person".to_string(),
            "place_of_birth".to_string(),
            place_of_birth.clone(),
        )
        .await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }

        let mut personal_id_numbers: Vec<PersonalIdNumberDbResponse> = Vec::new();
        for person_id_number in db_request.personal_id_number.unwrap() {
            // insert id for personal id number
            let id_number_id = person_id_number.id.unwrap();
            let id_number = person_id_number.id_number.unwrap();
            result =
                save_personal_id_number(&transaction, id, id_number_id, id_number.clone()).await;
            if let Err(error) = result {
                return Err(DbError::UnknownError(
                    error.into_source().unwrap().to_string(),
                ));
            }

            // insert date of issue
            let date_of_issue = person_id_number.date_of_issue.unwrap();
            result = update_person_info(
                &transaction,
                person_id,
                "person_id_number".to_string(),
                "date_of_issue".to_string(),
                place_of_birth.clone(),
            ).await;
            if let Err(error) = result {
                return Err(DbError::UnknownError(
                    error.into_source().unwrap().to_string(),
                ));
            }
            // insert place of issue
            let place_of_issue = person_id_number.place_of_issue.unwrap();
            result = update_person_info(
                &transaction,
                person_id,
                "person_id_number".to_string(),
                "place_of_issue".to_string(),
                place_of_birth.clone(),
            ).await;
            if let Err(error) = result {
                return Err(DbError::UnknownError(
                    error.into_source().unwrap().to_string(),
                ));
            }

            // insert id number provider
            let id_number_provider = person_id_number.code.unwrap();
            result = update_person_info(
                &transaction,
                person_id,
                "person_id_number".to_string(),
                "place_of_issue".to_string(),
                place_of_birth.clone(),
            ).await;
            if let Err(error) = result {
                return Err(DbError::UnknownError(
                    error.into_source().unwrap().to_string(),
                ));
            }
            personal_id_numbers.push(PersonalIdNumberDbResponse {
                id: id_number_id,
                person_id: Some(id),
                id_number: Some(id_number),
                code: Some(id_number_provider),
                date_of_issue: Some(date_of_issue),
                place_of_issue: Some(place_of_issue),
            })
        }

        transaction
            .commit()
            .await
            .map(|_| PersonDbResponse {
                id: person_id,
                first_name: Some(first_name.clone()),
                middle_name: Some(middle_name.clone()),
                last_name: Some(last_name.clone()),
                date_of_birth: Some(date_of_birth),
                place_of_birth: Some(place_of_birth.clone()),
                email: Some(email.clone()),
                phone: Some(phone.clone()),
                personal_id_numbers: None,
            })
            .map_err(|error| DbError::UnknownError(error.into_source().unwrap().to_string()))
    }
}
