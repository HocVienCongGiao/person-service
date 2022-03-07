use hvcg_biography_openapi_person::models::{PersonView, PersonViewCollection};
use lambda_http::http::header::{
    ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN,
    CONTENT_TYPE,
};
use lambda_http::{Body, Request, Response};

pub fn execute(
    status_code: u16,
    person_response: Option<PersonView>,
    person_collection: Option<PersonViewCollection>,
) -> Response<Body> {
    let mut is_get_persons = false;
    if person_collection.is_some() {
        is_get_persons = true;
    }

    let mut content_type: String = "application/json".to_string();
    if status_code == 204 {
        content_type = "".to_string();
        println!("status code is 204, removing application/json in Content_type header");
    }

    let response: Response<Body> = Response::builder()
        .header(CONTENT_TYPE, content_type)
        .header(ACCESS_CONTROL_ALLOW_HEADERS, "*")
        .header(ACCESS_CONTROL_ALLOW_METHODS, "*")
        .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .status(status_code)
        .body(
            if person_response.is_none() && person_collection.is_none() {
                Body::Empty
            } else {
                if is_get_persons {
                    serde_json::to_string(&person_collection)
                } else {
                    serde_json::to_string(&person_response)
                }
                .expect("unable to serialize serde_json::Value")
                .into()
            },
        )
        .expect("unable to build http:Response");

    println!(
        "final user response{:?}",
        serde_json::to_string(&person_collection)
    );
    response
}

pub fn default_response(request: Request) -> Response<Body> {
    Response::builder()
        .header(CONTENT_TYPE, "")
        .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .header(ACCESS_CONTROL_ALLOW_HEADERS, "*")
        .header(ACCESS_CONTROL_ALLOW_METHODS, "*")
        .status(200)
        .body(Body::Empty)
        .unwrap()
}
