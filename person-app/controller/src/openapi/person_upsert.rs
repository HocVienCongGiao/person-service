// use crate::openapi::ToOpenApi;
// use domain::usecases::student_usecase_shared_models::StudentUsecaseSharedTitle;
// use hvcg_academics_openapi_student::models::StudentTitle as StudentTitleOpenApi;
//
// impl ToOpenApi<StudentTitleOpenApi> for StudentUsecaseSharedTitle {
//     fn to_openapi(self) -> StudentTitleOpenApi {
//         match self {
//             StudentUsecaseSharedTitle::Monk => StudentTitleOpenApi::MONK,
//             StudentUsecaseSharedTitle::Nun => StudentTitleOpenApi::NUN,
//             StudentUsecaseSharedTitle::Priest => StudentTitleOpenApi::PRIEST,
//         }
//     }
// }
