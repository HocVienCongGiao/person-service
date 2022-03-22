use crate::person_gateway::find_one_person_by_id_adapter::from_pg_row_to_person_db_response;
use crate::person_gateway::repository::PersonRepository;
use async_trait::async_trait;
use chrono::NaiveDate;
use domain::ports::person::find_person_collection_port::FindPersonCollectionPort;
use domain::ports::person::models::person_dbrequest::PersonQuery as PersonQueryDbRequest;
use domain::ports::person::models::person_dbresponse::Person as PersonDbResponse;
use domain::ports::person::models::person_dbresponse::PersonCollection as PersonCollectionDbResponse;
use tokio_postgres::types::ToSql;
use tokio_postgres::{Client, Error, Row};

pub struct PersonFilter {
    name: String,
    email: String,
    phone: String,
    date_of_birth: Option<NaiveDate>,
    place_of_birth: String,
}

#[async_trait]
impl FindPersonCollectionPort for PersonRepository {
    async fn find_collection_by(
        &self,
        db_request: PersonQueryDbRequest,
    ) -> PersonCollectionDbResponse {
        let name = db_request.name.unwrap_or_else(|| "".to_string());
        let email = db_request.email.unwrap_or_else(|| "".to_string());
        let phone = db_request.phone.unwrap_or_else(|| "".to_string());
        let date_of_birth = db_request.date_of_birth;
        let place_of_birth = db_request.place_of_birth.unwrap_or_else(|| "".to_string());
        let offset = db_request.offset.unwrap_or(0);
        let count = db_request.count.unwrap_or(20);

        let filter = PersonFilter {
            name,
            email,
            phone,
            date_of_birth,
            place_of_birth,
        };

        let filtering_string = build_filtering_query_statement_string();

        let result = find_by(
            &(*self).client,
            &filter,
            count,
            offset,
            filtering_string.clone(),
        )
        .await;

        let collection: Vec<PersonDbResponse>;
        if let Ok(result) = result {
            collection = result
                .into_iter()
                .map(from_pg_row_to_person_db_response)
                .collect();
        } else {
            collection = vec![];
        }

        let has_more: Option<bool>;
        let total_from_offset =
            count_without_limit(&(*self).client, &filter, offset, filtering_string.clone())
                .await
                .unwrap();

        if total_from_offset > count {
            has_more = Some(true);
        } else {
            has_more = Some(false);
        }

        let total = count_total(&(*self).client, &filter, filtering_string)
            .await
            .unwrap();

        PersonCollectionDbResponse {
            collection,
            has_more,
            total,
        }
    }
}

fn build_filtering_query_statement_string() -> String {
    "(unaccent(last_name) LIKE ('%' || unaccent($1) || '%') \
        OR unaccent(middle_name) LIKE ('%' || unaccent($1) || '%') \
        OR unaccent(first_name) LIKE ('%' || unaccent($1) || '%')) \
        AND (unaccent(email) LIKE ('%' || unaccent($2) || '%') OR email is NULL) \
        AND (unaccent(phone) LIKE ('%' || unaccent($3) || '%') OR email is NULL) \
        AND ($4::DATE IS NULL OR date_of_birth = $4::DATE) \
        AND (unaccent(place_of_birth) LIKE ('%' || unaccent($5) || '%') OR email is NULL) "
        .to_string()
}

async fn find_by(
    client: &Client,
    filter: &PersonFilter,
    count: i64,
    offset: i64,
    filtering_string: String,
) -> Result<Vec<Row>, Error> {
    let statement = format!(
        "SELECT * FROM person__person_view \
        WHERE {} \
        LIMIT $6 OFFSET $7",
        filtering_string
    );

    println!("statement = {}", statement);

    let stmt = (*client).prepare(&statement).await.unwrap();

    let name_param: &[&(dyn ToSql + Sync)] = &[
        &filter.name,
        &filter.email,
        &filter.phone,
        &filter.date_of_birth,
        &filter.place_of_birth,
        &count,
        &offset,
    ];

    client.query(&stmt, name_param).await
}

async fn count_without_limit(
    client: &Client,
    filter: &PersonFilter,
    offset: i64,
    filter_string: String,
) -> Result<i64, Error> {
    let statement = format!(
        "SELECT COUNT(*) FROM (\
        SELECT * FROM person__person_view \
        WHERE {} \
        LIMIT ALL OFFSET $6 ) as persons",
        filter_string
    );

    println!("statement = {}", statement);
    let stmt = (*client).prepare(&statement).await.unwrap();
    let name_param: &[&(dyn ToSql + Sync)] = &[
        &filter.name,
        &&filter.email,
        &filter.phone,
        &filter.date_of_birth,
        &filter.place_of_birth,
        &offset,
    ];
    Ok(client.query_one(&stmt, name_param).await?.get("count"))
}

async fn count_total(
    client: &Client,
    filter: &PersonFilter,
    filtering_string: String,
) -> Result<i64, Error> {
    let statement = format!(
        "SELECT COUNT(*) FROM person__person_view \
        WHERE {}",
        filtering_string
    );

    println!("statement = {}", statement);
    let stmt = (*client).prepare(&statement).await.unwrap();
    let name_param: &[&(dyn ToSql + Sync)] = &[
        &filter.name,
        &filter.email,
        &filter.phone,
        &filter.date_of_birth,
        &filter.place_of_birth,
    ];
    Ok(client.query_one(&stmt, name_param).await?.get("count"))
}
