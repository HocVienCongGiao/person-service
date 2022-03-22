use crate::usecases::UsecaseError;

pub mod find_one_person_by_id_port;
pub mod insert_person_port;
pub mod person;
pub mod person_db_gateway;
pub mod personal_id_number;

#[derive(Debug)]
pub enum DbError {
    UniqueConstraintViolationError(String),
    UnknownError(String),
}

impl DbError {
    pub(crate) fn to_usecase_error(&self) -> UsecaseError {
        match self {
            DbError::UniqueConstraintViolationError(field) => {
                UsecaseError::UniqueConstraintViolationError(field.to_string())
            }
            DbError::UnknownError(msg) => UsecaseError::UnknownError(msg.to_string()),
        }
    }
}
