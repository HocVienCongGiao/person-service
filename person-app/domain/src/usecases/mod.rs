pub mod create_person_usecase;
pub mod person_usecase_shared_models;
pub mod query_one_person_by_id_usecase;
pub mod query_one_personal_id_number_usecase;
pub mod update_one_person_by_id;
pub mod delete_person_usecase;

pub(crate) trait ToEntity<T> {
    fn to_entity(self) -> T;
}

pub(crate) trait ToUsecaseOutput<T> {
    fn to_usecase_output(self) -> T;
}

#[derive(Debug)]
pub enum UsecaseError {
    UniqueConstraintViolationError(String),
    IdCollisionError,
    InvalidInput,
    UnknownError(String),
    ResourceNotFound,
}
