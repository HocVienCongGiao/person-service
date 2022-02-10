use lambda_http::{http, Body, Context, IntoResponse, Request};

pub async fn build_http_request_hello_world() -> Option<String> {
    let uri =
        "https://dev-sg.portal.hocvienconggiao.com/query-api/person-service/persons".to_string();
    let response = person::func(build_http_request(uri), Context::default())
        .await
        .expect("expected Ok(_) value")
        .into_response();

    let mut response_data: Option<String> = None;
    if let Body::Text(body) = response.body() {
        // println!("{:?}", serde_json::from_str(body.as_str()));
        response_data = Some(body.to_owned())
    }
    response_data
}

fn build_http_request(uri: String) -> Request {
    http::Request::builder()
        .uri(uri)
        .method("GET")
        .header("Content-Type", "application/json")
        .body(Body::Text("Hello World".to_string()))
        .unwrap()
}
