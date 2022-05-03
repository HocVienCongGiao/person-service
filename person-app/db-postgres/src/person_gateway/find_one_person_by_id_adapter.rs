use crate::db_column;
use crate::person_gateway::repository::PersonRepository;
use async_trait::async_trait;
use domain::entities::educational_stage::{EducationalLevel, EducationalStage};
use domain::entities::language::{Language, LanguageLevel};
use domain::entities::person::Nationality;
use domain::entities::title::{Position, Title};
use domain::entities::vow_progress::VowProgress;
use domain::ports::find_one_person_by_id_port::FindOnePersonByIdPort;
use domain::ports::person::models::person_dbresponse::Person as PersonDbResponse;
use std::str::FromStr;
use tokio_postgres::types::ToSql;
use tokio_postgres::Row;
use uuid::Uuid;

#[async_trait]
impl FindOnePersonByIdPort for PersonRepository {
    async fn find_one_by_id(&self, id: Uuid) -> Option<PersonDbResponse> {
        let stmt = (*self)
            .client
            .prepare("SELECT * FROM person__person_view WHERE id = $1")
            .await
            .unwrap();

        // let stmt = block_on(stmt_future).unwrap();
        let name_param: &[&(dyn ToSql + Sync)] = &[&id];
        let row = (*self).client.query_one(&stmt, name_param).await;
        return match row {
            Ok(row) => {
                let mut person = from_pg_row_to_person_db_response(row);

                let languages_pg_row = (*self)
                    .client
                    .query(
                        "SELECT * FROM person__person_languages WHERE person_id = $1",
                        &[&id],
                    )
                    .await;

                person.languages = match languages_pg_row {
                    Ok(rows) => Some(
                        rows.into_iter()
                            .map(from_pg_row_to_language)
                            .rev()
                            .collect(),
                    ),
                    _ => None,
                };

                // fixme
                let educational_stages_pg_row = (*self)
                    .client
                    .query(
                        "SELECT * FROM person__person_education_stage_view WHERE person_id = $1",
                        &[&id],
                    )
                    .await;

                person.educational_stages = match educational_stages_pg_row {
                    Ok(rows) => Some(
                        rows.into_iter()
                            .map(from_pg_row_to_educational_stage)
                            .collect(),
                    ),
                    _ => None,
                };

                Some(person)
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
                None
            }
        };
    }
}

pub(crate) fn from_pg_row_to_educational_stage(row: Row) -> EducationalStage {
    EducationalStage {
        id: db_column::get_uuid(&row, "educational_stage_id"),
        educational_level: EducationalLevel::from_str(&db_column::get_string(
            &row,
            "educational_level",
        ))
        .ok(),
        school_name: db_column::get_result_of_string(&row, "school_name"),
        major: db_column::get_result_of_string(&row, "major"),
        graduate_year: Some(db_column::get_result_of_int(&row, "graduate_year").unwrap() as f64),
    }
}

pub(crate) fn from_pg_row_to_language(row: Row) -> Language {
    Language {
        language: db_column::get_result_of_string(&row, "language"),
        level: LanguageLevel::from_str(&db_column::get_string(&row, "level")).ok(),
    }
}

pub(crate) fn from_pg_row_to_person_db_response(row: Row) -> PersonDbResponse {
    PersonDbResponse {
        id: db_column::get_uuid(&row, "id"),
        first_name: db_column::get_result_of_string(&row, "first_name"),
        middle_name: db_column::get_result_of_string(&row, "middle_name"),
        last_name: db_column::get_result_of_string(&row, "last_name"),
        date_of_birth: Some(db_column::get_date(&row, "date_of_birth")),
        place_of_birth: db_column::get_result_of_string(&row, "place_of_birth"),
        email: db_column::get_result_of_string(&row, "email"),
        phone: db_column::get_result_of_string(&row, "phone"),
        address: db_column::get_result_of_string(&row, "address"),
        saint_ids: None, // don't need this
        christian_name: db_column::get_result_of_string(&row, "christian_name"),
        languages: None,
        personal_id_numbers: None,
        position: Some(Position {
            title: Title::from_str(&db_column::get_string(&row, "title")).ok(),
            period: VowProgress::from_str(&db_column::get_string(&row, "progress")).ok(),
            parish: Some(db_column::get_uuid(&row, "polity_id")),
        }),
        nationality: Nationality::from_str(&db_column::get_string(&row, "nationality")).ok(),
        educational_stages: None,
        race: db_column::get_result_of_string(&row, "race"),
    }
}
