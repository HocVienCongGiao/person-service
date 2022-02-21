pub mod create_person_usecase;
pub mod person_usecase_shared_models;

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
