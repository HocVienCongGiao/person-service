use lambda_http::{Body, Request, Response};
use crate::build_response;

pub async fn execute(request: Request) -> Response<Body> {
    build_response::execute(200, None, None)
}