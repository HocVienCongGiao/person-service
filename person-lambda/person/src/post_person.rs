use crate::build_response;
use domain::usecases::UsecaseError;
use hvcg_biography_openapi_person::models::PersonUpsert;
use lambda_http::http::StatusCode;
use lambda_http::{Body, Request, RequestExt, Response};

pub async fn execute(request: Request) -> Response<Body> {
    println!("Handle post method.");
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
    // Create person
    return post_request(payload.unwrap()).await;
}

async fn post_request(value: PersonUpsert) -> Response<Body> {
    let lambda_person_request = value;
    let result = controller::create_person(lambda_person_request).await;
    let status_code = match result {
        Ok(_) => 200,
        Err(UsecaseError::UniqueConstraintViolationError(..)) => 503,
        Err(UsecaseError::InvalidInput) => 405,
        _ => 500,
    };

    let person_response = result.map(Some).unwrap_or_else(|e| {
        println!("error: {:?}", e);
        None
    });

    build_response::execute(status_code, person_response, None)
}
