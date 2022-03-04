use crate::common::request_builder;
use hvcg_biography_openapi_person::models::PersonView;
use lambda_http::{Body, Context, IntoResponse};

pub async fn get_one_person_by_id(uuid: String) -> Option<PersonView> {
    let request = request_builder::build_http_request_to_get_one_person(uuid);
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
