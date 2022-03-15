use crate::Error;
use chrono::NaiveDate;
use domain::usecases::UsecaseError;
use lambda_http::http::header::{
    ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN,
    CONTENT_TYPE,
};
use lambda_http::http::{method, uri::Uri, HeaderValue};
use lambda_http::{handler, Body, Context, IntoResponse, Request, RequestExt, Response};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::env;
use std::str::FromStr;

fn from_query_param_to_string(request: &Request, param: &str) -> Option<String> {
    let query = request.query_string_parameters();
    query.get(param).map(|str| str.parse().unwrap())
}

pub fn from_request_to_id(req: &Request) -> Option<uuid::Uuid> {
    let path_parameters = req.path_parameters();
    let id_param = path_parameters.get("id");
    if let Some(id) = id_param {
        println!("id found");
        Some(uuid::Uuid::parse_str(id).unwrap())
    } else {
        println!("id not found");
        None
    }
}

pub fn from_request_to_collection_query
