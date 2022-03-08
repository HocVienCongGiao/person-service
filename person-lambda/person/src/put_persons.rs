use crate::build_response;
use hvcg_biography_openapi_person::models::PersonUpsert;
use lambda_http::http::StatusCode;
use lambda_http::{Body, Request, RequestExt, Response};

pub async fn execute(request: Request) -> Response<Body> {
    println!("Handle put method");
    let payload: Option<PersonUpsert> = match request.payload() {
        Ok(Some(body)) => body,
        Err(e) => {
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(e.to_string().into())
                .expect("Invalid payload")
        }
        _ => None,
    };
    if payload.is_none() {
        return build_response::execute(400, None, None);
    }
    build_response::execute(200, None, None)
}

async fn put_request(value: PersonUpsert) -> Response<Body> {
    let lambda_person_request = value;
    let result = controller::
}
