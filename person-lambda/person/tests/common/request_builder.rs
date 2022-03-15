use hvcg_biography_openapi_person::models::PersonUpsert;
use lambda_http::http::Request;
use lambda_http::{http, Body, Context, IntoResponse, RequestExt};
use std::collections::HashMap;

pub fn build_http_request_to_get_person_collection(offset: i32, count: i32) -> Request<Body> {
    let mut query_param = HashMap::new();
    query_param.insert("count".to_string(), vec![count.to_string()]);
    query_param.insert("offset".to_string(), vec![offset.to_string()]);
    let mut path_param = HashMap::new();

    let uri =
        "https://dev-sg.portal.hocvienconggiao.com/query-api/person-service/persons".to_string();
    build_http_get_request(uri, query_param, path_param)
}

pub fn build_http_request_to_get_one_person(uuid: String) -> Request<Body> {
    let mut query_param = HashMap::new();
    let mut path_param = HashMap::new();

    let uri = format!(
        "https://dev-sg.portal.hocvienconggiao.com/query-api/person-service/persons/{}",
        uuid
    );

    path_param.insert("id".to_string(), vec![uuid]);
    build_http_get_request(uri, query_param, path_param)
}

pub fn build_http_get_request(
    uri: String,
    query_param: HashMap<String, Vec<String>>,
    path_param: HashMap<String, Vec<String>>,
) -> Request<Body> {
    build_http_request("GET".to_string(), uri, None, query_param, path_param)
}

pub fn build_http_request_to_post_person_upsert(
    given_person_upsert: PersonUpsert,
) -> Request<Body> {
    let query_param = HashMap::new();
    let path_param = HashMap::new();
    let uri =
        "https://dev-sg.portal.hocvienconggiao.com/mutation-api/person-service/persons".to_string();

    let serialized = serde_json::to_string(&given_person_upsert).unwrap();
    build_http_post_request(uri, query_param, path_param, Some(serialized))
}

fn build_http_post_request(
    uri: String,
    query_param: HashMap<String, Vec<String>>,
    path_param: HashMap<String, Vec<String>>,
    body: Option<String>,
) -> Request<Body> {
    build_http_request("POST".to_string(), uri, body, query_param, path_param)
}

fn build_http_request(
    method: String,
    uri: String,
    body: Option<String>,
    query_param: HashMap<String, Vec<String>>,
    path_param: HashMap<String, Vec<String>>,
) -> Request<Body> {
    let mut request_body = Body::Empty;
    if let Some(body) = body {
        request_body = Body::from(body)
    }
    let request = http::Request::builder()
        .uri(uri)
        .method(method.as_str())
        .header("Content-Type", "application/json")
        .body(request_body)
        .unwrap()
        .with_query_string_parameters(query_param)
        .with_path_parameters(path_param);
    request
}

pub(crate) fn build_http_request_to_delete_one_person(uuid: String) -> Request<Body> {
    let mut query_param = HashMap::new();
    let mut path_param = HashMap::new();

    let uri = format!(
        "https://dev-sg.portal.hocvienconggiao.com/mutation-api/person-service/persons/{}",
        uuid
    );

    path_param.insert("id".to_string(), vec![uuid]);
    build_http_delete_request(uri, query_param, path_param)
}

fn build_http_delete_request(
    uri: String,
    query_param: HashMap<String, Vec<String>>,
    path_param: HashMap<String, Vec<String>>,
) -> Request<Body> {
    build_http_request("DELETE".to_string(), uri, None, query_param, path_param)
}

pub fn build_http_request_to_put_person(
    person_upsert: PersonUpsert,
    uuid: String,
) -> Request<Body> {
    let mut query_param = HashMap::new();
    let mut path_param = HashMap::new();

    let uri = format!(
        "https://dev-sg.portal.hocvienconggiao.com/mutation-api/person-service/persons/{}",
        uuid
    );

    path_param.insert("id".to_string(), vec![uuid]);
    let serialized = serde_json::to_string(&person_upsert).unwrap();

    build_http_put_request(uri, query_param, path_param, Some(serialized))
}

fn build_http_put_request(
    uri: String,
    query_param: HashMap<String, Vec<String>>,
    path_param: HashMap<String, Vec<String>>,
    body: Option<String>,
) -> Request<Body> {
    build_http_request("PUT".to_string(), uri, body, query_param, path_param)
}
