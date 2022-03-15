use crate::build_response;
use crate::parse_request::from_request_to_id;
use lambda_http::http::header::{
    ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN,
    CONTENT_TYPE,
};
use lambda_http::http::HeaderValue;
use lambda_http::{Body, Request, RequestExt, Response};
use std::intrinsics::offset;
use uuid::Uuid;

pub async fn execute(request: Request) -> Response<Body> {
    println!("Handle get method.");
    let response: Response<Body>;

    return if let Some(id) = from_request_to_id(&request) {
        get_person_by_id(id).await
    } else {
        get_persons(request).await
    };
}

async fn get_person_by_id(id: Uuid) -> Response<Body> {
    let person_response = controller::get_one_person_by_id(id).await;
    build_response::execute(200, person_response, None)
}

async fn get_persons(request: Request) -> Response<Body> {
    let query = from_request_to_id()
    let person_response = controller::get_person_collection(offset, count).await;
    build_response::execute(200, None, None)
}
