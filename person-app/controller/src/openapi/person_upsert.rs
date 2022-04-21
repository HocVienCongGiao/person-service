use crate::openapi::ToUsecaseInput;
use domain::usecases::person_usecase_shared_models::educational_stage::{
    PersonUsecaseSharedEducationalLevel, PersonUsecaseSharedEducationalStage,
};
use domain::usecases::person_usecase_shared_models::language::{
    PersonUsecaseSharedLanguage, PersonUsecaseSharedLanguageLevel,
};
use domain::usecases::person_usecase_shared_models::nationality::PersonUsecaseSharedNationality;
use domain::usecases::person_usecase_shared_models::personal_id_number::{
    PersonUsecaseSharedIdNumber, PersonUsecaseSharedIdNumberProvider,
};
use domain::usecases::person_usecase_shared_models::title::{
    PersonUsecaseSharedPosition, PersonUsecaseSharedTitle,
};
use domain::usecases::person_usecase_shared_models::vow_progress::PersonUsecaseSharedVowProgress;
use hvcg_biography_openapi_person::models::{
    EducationalLevel, EducationalStage, ForeignLanguageLevel, IdNumberProvider, Language,
    Nationality, PersonUpsertPosition, PersonalIdNumber, Title, VowProgress,
};

impl ToUsecaseInput<PersonUsecaseSharedVowProgress> for VowProgress {
    fn to_usecase_input(self) -> PersonUsecaseSharedVowProgress {
        match self {
            VowProgress::NOVICE => PersonUsecaseSharedVowProgress::Novice,
            VowProgress::PREPARATION => PersonUsecaseSharedVowProgress::Preparation,
            VowProgress::SIMPLE_VOW => PersonUsecaseSharedVowProgress::SimpleVow,
            VowProgress::SOLEMN_VOW => PersonUsecaseSharedVowProgress::SolemnVow,
        }
    }
}

impl ToUsecaseInput<PersonUsecaseSharedTitle> for Title {
    fn to_usecase_input(self) -> PersonUsecaseSharedTitle {
        match self {
            Title::MONK => PersonUsecaseSharedTitle::Monk,
            Title::NUN => PersonUsecaseSharedTitle::Nun,
            Title::PRIEST => PersonUsecaseSharedTitle::Priest,
        }
    }
}

impl ToUsecaseInput<PersonUsecaseSharedPosition> for PersonUpsertPosition {
    fn to_usecase_input(self) -> PersonUsecaseSharedPosition {
        PersonUsecaseSharedPosition {
            title: Some(self.name.to_usecase_input()),
            period: self.period.map(|v| v.to_usecase_input()),
            parish: self.parish,
        }
    }
}

impl ToUsecaseInput<PersonUsecaseSharedNationality> for Nationality {
    fn to_usecase_input(self) -> PersonUsecaseSharedNationality {
        match self {
            Nationality::VIETNAMESE => PersonUsecaseSharedNationality::Vietnamese,
            Nationality::AMERICAN => PersonUsecaseSharedNationality::American,
            Nationality::CHINESE => PersonUsecaseSharedNationality::Chinese,
            Nationality::FRENCH => PersonUsecaseSharedNationality::French,
            Nationality::BRITISH => PersonUsecaseSharedNationality::British,
        }
    }
}

impl ToUsecaseInput<PersonUsecaseSharedLanguageLevel> for ForeignLanguageLevel {
    fn to_usecase_input(self) -> PersonUsecaseSharedLanguageLevel {
        match self {
            ForeignLanguageLevel::BEGINNER => PersonUsecaseSharedLanguageLevel::Beginner,
            ForeignLanguageLevel::INTERMEDIATE => PersonUsecaseSharedLanguageLevel::Intermediate,
            ForeignLanguageLevel::ADVANCED => PersonUsecaseSharedLanguageLevel::Advanced,
        }
    }
}

impl ToUsecaseInput<PersonUsecaseSharedLanguage> for Language {
    fn to_usecase_input(self) -> PersonUsecaseSharedLanguage {
        PersonUsecaseSharedLanguage {
            language: self.name.map(|name| name).unwrap(),
            level: self.level.map(|level| level.to_usecase_input()).unwrap(),
        }
    }
}

impl ToUsecaseInput<PersonUsecaseSharedEducationalLevel> for EducationalLevel {
    fn to_usecase_input(self) -> PersonUsecaseSharedEducationalLevel {
        match self {
            EducationalLevel::BACHELOR => PersonUsecaseSharedEducationalLevel::Bachelor,
            EducationalLevel::MASTER => PersonUsecaseSharedEducationalLevel::Master,
            EducationalLevel::ELEMENTARY_SCHOOL => {
                PersonUsecaseSharedEducationalLevel::ElementarySchool
            }
            EducationalLevel::MIDDLE_SCHOOL => PersonUsecaseSharedEducationalLevel::MiddleSchool,
            EducationalLevel::HIGH_SCHOOL => PersonUsecaseSharedEducationalLevel::HighSchool,
            EducationalLevel::DOCTOR => PersonUsecaseSharedEducationalLevel::Doctor,
            EducationalLevel::OTHER => PersonUsecaseSharedEducationalLevel::Other,
        }
    }
}

impl ToUsecaseInput<PersonUsecaseSharedEducationalStage> for EducationalStage {
    fn to_usecase_input(self) -> PersonUsecaseSharedEducationalStage {
        PersonUsecaseSharedEducationalStage {
            educational_level: Some(self.educational_level.to_usecase_input()),
            school_name: self.school_name,
            major: self.major,
            graduate_year: self.graduate_year,
        }
    }
}

impl ToUsecaseInput<PersonUsecaseSharedIdNumberProvider> for IdNumberProvider {
    fn to_usecase_input(self) -> PersonUsecaseSharedIdNumberProvider {
        match self {
            IdNumberProvider::NATIONAL_ID => PersonUsecaseSharedIdNumberProvider::NationalId,
            IdNumberProvider::PASSPORT => PersonUsecaseSharedIdNumberProvider::Passport,
        }
    }
}

impl ToUsecaseInput<PersonUsecaseSharedIdNumber> for PersonalIdNumber {
    fn to_usecase_input(self) -> PersonUsecaseSharedIdNumber {
        PersonUsecaseSharedIdNumber {
            id_number: self.id_number,
            code: self
                .id_number_provider
                .map(|provider| provider.to_usecase_input()),
            date_of_issue: self.date_of_issue,
            place_of_issue: self.place_of_issue,
        }
    }
}
