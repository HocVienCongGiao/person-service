use async_trait::async_trait;
use chrono::NaiveDate;
use domain::entities::educational_stage::EducationalStage as EducationalStageEntity;
use tokio_postgres::types::ToSql;
use tokio_postgres::{Error, Transaction};
use uuid::Uuid;

use domain::entities::language::Language as LanguageEntity;
use domain::entities::person::Nationality;
use domain::entities::title::Position;

use crate::person_gateway::repository::PersonRepository;
use domain::ports::person::insert_person_port::InsertPersonPort;
use domain::ports::person::models::person_dbresponse::Person as PersonDbResponse;
use domain::ports::person::models::person_mutation_dbrequest::Person as PersonMutationDbRequest;
use domain::ports::personal_id_number::models::personal_id_number_db_response::PersonalIdNumberDbResponse;
use domain::ports::DbError;
use std::ops::Add;

pub(crate) async fn save_id(
    transaction: &Transaction<'_>,
    id: Uuid,
    object_name: String,
) -> Result<u64, Error> {
    let statement = format!(
        "INSERT into public.person__{} (id) VAlUES ($1)",
        object_name
    );
    let stmt = (*transaction).prepare(&statement).await.unwrap();

    let params: &[&(dyn ToSql + Sync)] = &[&id];
    transaction.execute(&stmt, params).await
}

pub(crate) async fn save_person_info(
    transaction: &Transaction<'_>,
    id: Uuid,
    field_name: String,
    value: String,
) -> Result<u64, Error> {
    let statement = format!(
        "INSERT into public.person__person_{} (id, {}) VAlUES ($1, $2)",
        field_name, field_name
    );
    let stmt = (*transaction).prepare(&statement).await.unwrap();

    let params: &[&(dyn ToSql + Sync)] = &[&id, &value];
    transaction.execute(&stmt, params).await
}

pub(crate) async fn save_personal_id_number(
    transaction: &Transaction<'_>,
    person_id: Uuid,
    personal_id_number_id: Uuid,
    personal_id_number: String,
) -> Result<u64, Error> {
    let stmt = (*transaction)
        .prepare(
            "INSERT INTO public.person__person_id_number (id, person_id, person_id_number) VAlUES ($1, $2, $3)",
        )
        .await
        .unwrap();

    let params: &[&(dyn ToSql + Sync)] = &[&personal_id_number_id, &person_id, &personal_id_number];
    transaction.execute(&stmt, params).await
}

pub(crate) async fn save_polity(
    transaction: &Transaction<'_>,
    person_id: Uuid,
    polity_id: Uuid,
) -> Result<u64, Error> {
    let stmt = (*transaction)
        .prepare("INSERT INTO public.person__person_polity (id, polity_id) VAlUES ($1, $2)")
        .await
        .unwrap();

    let params: &[&(dyn ToSql + Sync)] = &[&person_id, &polity_id];
    transaction.execute(&stmt, params).await
}

pub(crate) async fn save_educational_stage(
    transaction: &Transaction<'_>,
    person_id: Uuid,
    stage_id: Uuid,
    graduated_year: i32,
) -> Result<u64, Error> {
    let stmt = (*transaction)
        .prepare(
            "INSERT INTO public.person__person_educational_stages (person_id, educational_stage_id, graduate_year) VAlUES ($1, $2, $3)",
        )
        .await
        .unwrap();

    let params: &[&(dyn ToSql + Sync)] = &[&person_id, &stage_id, &graduated_year];
    transaction.execute(&stmt, params).await
}

pub(crate) async fn save_date(
    transaction: &Transaction<'_>,
    id: Uuid,
    object_name: String,
    field_name: String,
    date: NaiveDate,
) -> Result<u64, Error> {
    let statement = format!(
        "INSERT into public.person__{}_{} (id, {}) VAlUES ($1, $2)",
        object_name, field_name, field_name
    );
    let stmt = (*transaction).prepare(&statement).await.unwrap();

    let params: &[&(dyn ToSql + Sync)] = &[&id, &date];
    transaction.execute(&stmt, params).await
}

pub(crate) async fn save_personal_extra_data(
    transaction: &Transaction<'_>,
    object_id: Uuid,
    object_name: String,
    table_name: String,
    field_name: String,
    value: String,
) -> Result<u64, Error> {
    let statement = format!(
        "INSERT INTO public.person__{}_{} (id, {}) VAlUES ($1, $2)",
        object_name, table_name, field_name
    );
    let stmt = (*transaction).prepare(&statement).await.unwrap();

    let params: &[&(dyn ToSql + Sync)] = &[&object_id, &value];
    transaction.execute(&stmt, params).await
}

pub(crate) async fn save_language(
    transaction: &Transaction<'_>,
    personal_id: Uuid,
    language: String,
    level: String,
) -> Result<u64, Error> {
    let stmt = (*transaction)
        .prepare(
            "INSERT INTO public.person__person_languages (person_id, language, level) VAlUES ($1, $2, $3)",
        )
        .await
        .unwrap();

    let params: &[&(dyn ToSql + Sync)] = &[&personal_id, &language, &level];
    transaction.execute(&stmt, params).await
}

pub(crate) async fn save_christian_names(
    transaction: &Transaction<'_>,
    id: Uuid,
    christian_names: Vec<Uuid>,
) -> Result<u64, Error> {
    // TODO: refactor this into 1 query
    // TODO: result and then
    let mut result: Result<u64, Error> = Ok(1_u64);
    let mut ordering: i16 = 1;
    // TODO: use enumerate instead
    for christian_name in christian_names {
        let params: &[&(dyn ToSql + Sync)] = &[&id, &christian_name, &ordering];
        let stmt = (*transaction)
            .prepare("INSERT into public.person__person_christian_names (person_id, saint_id, ordering) VAlUES ($1, $2, $3)")
            .await
            .unwrap();
        result = transaction.execute(&stmt, params).await;
        ordering = ordering.add(1);
    }
    result
}

#[async_trait]
impl InsertPersonPort for PersonRepository {
    async fn insert(
        &mut self,
        db_request: PersonMutationDbRequest,
    ) -> Result<PersonDbResponse, DbError> {
        let transaction = (*self)
            .client
            .transaction()
            .await
            .map_err(|error| DbError::UnknownError(error.into_source().unwrap().to_string()))?;

        // insert id
        let id = db_request.id.unwrap();
        let mut result: Result<u64, Error> = save_id(&transaction, id, "person".to_string()).await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }

        // insert first name
        let first_name = db_request.first_name.unwrap();
        result = save_person_info(
            &transaction,
            id,
            "first_name".to_string(),
            first_name.clone(),
        )
        .await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }

        // insert last name
        let last_name = db_request.last_name.unwrap();
        result =
            save_person_info(&transaction, id, "last_name".to_string(), last_name.clone()).await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }

        // insert middle name
        let middle_name = db_request.middle_name.unwrap();
        result = save_person_info(
            &transaction,
            id,
            "middle_name".to_string(),
            middle_name.clone(),
        )
        .await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }

        // insert address
        let address = db_request.address.unwrap();
        result = save_person_info(&transaction, id, "address".to_string(), address.clone()).await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }

        // insert date of birth
        let date_of_birth = db_request.date_of_birth.unwrap();
        result = save_date(
            &transaction,
            id,
            "person".to_string(),
            "date_of_birth".to_string(),
            date_of_birth,
        )
        .await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }

        // insert email
        let email = db_request.email.unwrap();
        result = save_person_info(&transaction, id, "email".to_string(), email.clone()).await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }

        // insert phone
        let phone = db_request.phone.unwrap();
        result = save_person_info(&transaction, id, "phone".to_string(), phone.clone()).await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }

        // insert place of birth
        let place_of_birth = db_request.place_of_birth.unwrap();
        result = save_person_info(
            &transaction,
            id,
            "place_of_birth".to_string(),
            place_of_birth.clone(),
        )
        .await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }

        // insert nationality
        let nationality = db_request.nationality.unwrap();
        result = save_person_info(
            &transaction,
            id,
            "nationality".to_string(),
            nationality.to_string().clone(),
        )
        .await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }

        // insert christian names
        let christian_names = db_request.saint_ids.unwrap();
        result = save_christian_names(&transaction, id, christian_names.clone()).await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }

        // insert race
        let race = db_request.race.unwrap();
        result = save_person_info(&transaction, id, "race".to_string(), race.clone()).await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }

        let mut personal_id_numbers: Vec<PersonalIdNumberDbResponse> = Vec::new();
        for person_id_number in db_request.personal_id_numbers.unwrap() {
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
            result = save_date(
                &transaction,
                id_number_id,
                "person_id_number".to_string(),
                "date_of_issue".to_string(),
                date_of_issue,
            )
            .await;
            if let Err(error) = result {
                return Err(DbError::UnknownError(
                    error.into_source().unwrap().to_string(),
                ));
            }

            // insert place of issue
            let place_of_issue = person_id_number.place_of_issue.unwrap();
            result = save_personal_extra_data(
                &transaction,
                id_number_id,
                "person_id_number".to_string(),
                "place_of_issue".to_string(),
                "place_of_issue".to_string(),
                place_of_issue.clone(),
            )
            .await;
            if let Err(error) = result {
                return Err(DbError::UnknownError(
                    error.into_source().unwrap().to_string(),
                ));
            }

            // insert id number provider
            let id_number_provider = person_id_number.code.unwrap();
            result = save_personal_extra_data(
                &transaction,
                id_number_id,
                "person_id_number".to_string(),
                "provider".to_string(),
                "code".to_string(),
                id_number_provider.clone(),
            )
            .await;
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

        // insert language
        let languages: Vec<LanguageEntity> = match db_request.languages {
            Some(languages) => {
                let mut r = Vec::new();
                for lang in languages {
                    let language = lang.clone();
                    let name = language.language.unwrap();
                    let level = language.level.unwrap();
                    result = save_language(&transaction, id, name, level.to_string()).await;
                    if let Err(error) = result {
                        return Err(DbError::UnknownError(
                            error.into_source().unwrap().to_string(),
                        ));
                    }
                    r.push(lang.clone())
                }
                r
            }
            _ => vec![],
        };

        // insert educational stage
        let educational_stages: Vec<EducationalStageEntity> = match db_request.educational_stages {
            Some(stages) => {
                let mut r = Vec::new();
                for stage in stages {
                    let stage_id = stage.id;
                    // insert id for new stage of a person
                    result = save_id(&transaction, stage_id, "educational_stage".to_string()).await;
                    if let Err(error) = result {
                        return Err(DbError::UnknownError(
                            error.into_source().unwrap().to_string(),
                        ));
                    }

                    // insert educational stage level
                    // TODO: too much argument
                    if let Some(educational_level) = stage.educational_level.clone() {
                        result = save_personal_extra_data(
                            &transaction,
                            stage_id,
                            "educational_stage".to_string(),
                            "educational_level".to_string(),
                            "level".to_string(),
                            educational_level.to_string(),
                        )
                        .await;
                    }

                    // insert major
                    if let Some(major) = stage.major.clone() {
                        result = save_personal_extra_data(
                            &transaction,
                            stage_id,
                            "educational_stage".to_string(),
                            "major".to_string(),
                            "major".to_string(),
                            major.to_string(),
                        )
                        .await;
                    }

                    // insert school name
                    if let Some(school_name) = stage.school_name.clone() {
                        result = save_personal_extra_data(
                            &transaction,
                            stage_id,
                            "educational_stage".to_string(),
                            "school_name".to_string(),
                            "school_name".to_string(),
                            school_name.to_string(),
                        )
                        .await;
                    }

                    // attach a stage to a person
                    result = save_educational_stage(
                        &transaction,
                        id,
                        stage_id,
                        stage.graduate_year.unwrap() as i32,
                    )
                    .await;
                    if let Err(error) = result {
                        return Err(DbError::UnknownError(
                            error.into_source().unwrap().to_string(),
                        ));
                    }
                    r.push(stage.clone())
                }
                r
            }
            _ => vec![],
        };

        let position: Option<Position> = match db_request.position {
            Some(position) => {
                // insert title
                if let Some(title) = position.title.clone() {
                    result = save_personal_extra_data(
                        &transaction,
                        id,
                        "person".to_string(),
                        "title".to_string(),
                        "title".to_string(),
                        title.to_string(),
                    )
                    .await;
                }
                if let Err(error) = result {
                    return Err(DbError::UnknownError(
                        error.into_source().unwrap().to_string(),
                    ));
                }
                // insert vow progress
                if let Some(vow_progress) = position.period.clone() {
                    result = save_personal_extra_data(
                        &transaction,
                        id,
                        "person".to_string(),
                        "vow_progress".to_string(),
                        "progress".to_string(),
                        vow_progress.to_string(),
                    )
                    .await;
                }
                if let Err(error) = result {
                    return Err(DbError::UnknownError(
                        error.into_source().unwrap().to_string(),
                    ));
                }
                // insert title
                if let Some(parish) = position.parish {
                    result = save_polity(&transaction, id, parish).await;
                }
                if let Err(error) = result {
                    return Err(DbError::UnknownError(
                        error.into_source().unwrap().to_string(),
                    ));
                }
                Some(position)
            }
            _ => None,
        };

        transaction
            .commit()
            .await
            .map(|_| PersonDbResponse {
                id,
                first_name: Some(first_name.clone()),
                middle_name: Some(middle_name.clone()),
                last_name: Some(last_name.clone()),
                date_of_birth: Some(date_of_birth),
                place_of_birth: Some(place_of_birth.clone()),
                email: Some(email.clone()),
                phone: Some(phone.clone()),
                address: Some(address),
                personal_id_numbers: Some(personal_id_numbers),
                languages: Some(languages),
                educational_stages: Some(educational_stages),
                position,
                nationality: Some(nationality),
                saint_ids: Some(christian_names),
                race: Some(race),
            })
            .map_err(|error| DbError::UnknownError(error.into_source().unwrap().to_string()))
    }
}
