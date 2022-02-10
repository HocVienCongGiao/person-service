use std::path::PathBuf;
use std::sync::Once;

mod common;

static INIT: Once = Once::new();

fn initialise() {
    INIT.call_once(|| {
        let my_path = PathBuf::new().join(".env.test");
        dotenv::from_path(my_path.as_path()).ok();
        // println!("testing env {}", std::env::var("HELLO").unwrap());
    });
}

#[tokio::test]
async fn hello_world() {
    initialise();
    let actual_response_data = common::request_builder::build_http_request_hello_world().await;
    assert_eq!(
        "Hello World".to_string(),
        actual_response_data.unwrap(),
        "Some reason"
    )
}
