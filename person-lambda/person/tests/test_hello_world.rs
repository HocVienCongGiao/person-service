use crate::common::{getter, poster, test_data};
use hvcg_biography_openapi_person::models::{PersonUpsert, PersonView};
use std::path::PathBuf;
use std::sync::Once;
use uuid::Uuid;

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
async fn crud_should_work() {
    initialise();
    when_post_a_person_upsert_then_person_is_correctly_saved_and_person_view_returned().await;
    given_a_student_when_get_one_by_id_then_return_correct_student_view_openapi().await;

}

async fn given_a_student_when_get_one_by_id_then_return_correct_student_view_openapi() {
    // Given
    let expected_person_view_openapi: PersonView = test_data::prepare_person_view_openapi(None);
    let given_uuid = expected_person_view_openapi.id.to_string();

    // When
    let actual_person_view_openapi = getter::get_one_person_by_id(given_uuid).await;

    // Then
    assert_eq!(
        expected_person_view_openapi,
        actual_person_view_openapi.unwrap()
    );
}

async fn when_post_a_person_upsert_then_person_is_correctly_saved_and_person_view_returned() {
    // Given
    let given_person_upsert_openapi: PersonUpsert = test_data::prepare_person_upsert_openapi();

    // When
    let actual_person_view_openapi = poster::post_person_upsert(given_person_upsert_openapi).await;

    // Then
    let actual_id: Option<Uuid> = actual_person_view_openapi.clone().map(|t| t.id);
    let expected_person_view_openapi = test_data::prepare_person_view_openapi(actual_id);
    assert_eq!(
        expected_person_view_openapi,
        actual_person_view_openapi.unwrap()
    );
}
