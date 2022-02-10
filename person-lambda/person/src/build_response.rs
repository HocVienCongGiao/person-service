use chrono::DateTime;
use jsonwebtoken::TokenData;
use lambda_http::http::header::{
    ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN,
    CONTENT_TYPE,
};
use lambda_http::{Body, Request, Response};
use std::env;

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

pub fn default_response(request: Request) -> Response<Body> {
    Response::builder()
        .header(CONTENT_TYPE, "")
        .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .header(ACCESS_CONTROL_ALLOW_HEADERS, "*")
        .header(ACCESS_CONTROL_ALLOW_METHODS, "*")
        .status(200)
        .body(Body::Text("Hello World".to_string()))
        .unwrap()
}
