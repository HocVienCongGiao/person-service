use hvcg_biography_openapi_person::models::PersonUpsert;
use lambda_http::http::Request;
use lambda_http::{http, Body, Context, IntoResponse, RequestExt};
use std::collections::HashMap;

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
    let test = "{
    'lastName': 'Nguyen',
    'saintName': 'Giuse',
    'dateOfBirth': {},
    'placeOfBirth': 'Tra Vinh',
    'email': 'binh@sunrise.vn',
    'phone': '+84 1228019700',
    'address': '1000 CMT8 p5 q.Tân Bình, TP.HCM',
    'nationality': 'Vietnamese',
    'race': 'Kinh',
    'personalIdNumbers': [
    {
        'idNumber': '99005014079',
        'idNumberProvider': 'NATIONAL_ID',
        'dateOfIssue': '2022-03-05',
        'placeOfIssue': 'string'
    }
    ],
    'language': 'Vietnamse',
    'completedProgram': 'University',
    'position': {
        'pastor': {
            'diocese': 'Xuan loc',
            'order': 'Da minh'
        },
        'seminarian': {
            'diocese': 'Xuan loc',
            'order': 'Da minh'
        },
        'monk': 'Khấn trọn',
        'parishioner': {
            'diocese': 'Xuan loc',
            'parish': 'Doc mo'
        }
    },
    'program': 'Cử Nhân'
}".to_string();
    let mut request_body = Body::Empty;
    if let Some(body) = body {
        request_body = Body::from(test)
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
