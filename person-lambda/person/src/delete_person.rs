use crate::build_response;
use crate::parse_request::from_request_to_id;
use domain::usecases::UsecaseError;
use lambda_http::http::StatusCode;
use lambda_http::{Body, Request, RequestExt, Response};
use uuid::Uuid;

pub async fn execute(request: Request) -> Response<Body> {
    println!("Handle delete method");
    let mut status_code = StatusCode::INTERNAL_SERVER_ERROR;
    let result = match from_request_to_id(&request) {
        Some(id) => {
            status_code = StatusCode::NO_CONTENT;
            controller::delete_one_person_by_id(id).await
        }
        _ => {
            status_code = StatusCode::NOT_FOUND;
            // UsecaseError::ResourceNotFound
            Ok(())
        }
    };
    // TODO: refactor
    build_response::execute(status_code.as_u16(), None, None)
}
