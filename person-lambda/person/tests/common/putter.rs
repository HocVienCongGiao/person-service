use crate::common::request_builder;
use hvcg_biography_openapi_person::models::{PersonUpsert, PersonView};
use lambda_http::{http, Body, Context, IntoResponse, RequestExt, Response};

pub async fn put_person(person_upsert: PersonUpsert, uuid: String) -> Option<PersonView> {
    let request = request_builder::build_http_request_to_put_person(person_upsert, uuid);
    // When
    let response = person::func(request, Context::default())
        .await
        .expect("expected Ok(_) value")
        .into_response();

    let mut person_view_openapi: Option<PersonView> = None;
    if let Body::Text(body) = response.body() {
        person_view_openapi =
            Some(serde_json::from_str(body).expect("Unable to deserialise response body"));
    }
    person_view_openapi
}
