use crate::common::request_builder;
use hvcg_biography_openapi_person::models::{PersonUpsert, PersonView};
use lambda_http::{Body, Context, IntoResponse};

pub async fn post_person_upsert(given_person_upsert: PersonUpsert) -> Option<PersonView> {
    let request = request_builder::build_http_request_to_post_person_upsert(given_person_upsert);
    // When
    let response = person::func(request, Context::default())
        .await
        .expect("expected Ok(_) value")
        .into_response();

    let mut person_view_openapi: Option<PersonView> = None;
    if let Body::Text(body) = response.body() {
        person_view_openapi =
            Some(serde_json::from_str(body).expect("Unable to deserialize response body"));
    }
    person_view_openapi
}
