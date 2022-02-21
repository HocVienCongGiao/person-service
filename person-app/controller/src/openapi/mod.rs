pub mod person_upsert;
pub mod person_view;

pub(crate) trait ToOpenApi<T> {
    fn to_openapi(self) -> T;
}

pub(crate) trait ToUsecaseInput<T> {
    fn to_usecase_input(self) -> T;
}
