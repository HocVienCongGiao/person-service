use crate::openapi::ToOpenApi;
use chrono::NaiveDate;
use domain::usecases::create_person_usecase::CreatePersonUsecaseOutput;
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
use domain::usecases::query_one_person_by_id_usecase::QueryPersonUsecaseOutput;
use domain::usecases::query_one_personal_id_number_usecase::PersonalIdNumberUsecaseOutput;
use domain::usecases::query_person_collection_usecase::QueryPersonCollectionUsecaseOutput;
use domain::usecases::update_one_person_by_id_usecase::UpdatePersonUsecaseOutput;
use hvcg_biography_openapi_person::models::{
    EducationalLevel, EducationalStage, ForeignLanguageLevel, IdNumberProvider, Language,
    Nationality, PersonUpsertPosition, PersonView, PersonViewCollection, PersonalIdNumber, Title,
    VowProgress,
};
use std::str::FromStr;

impl ToOpenApi<PersonViewCollection> for QueryPersonCollectionUsecaseOutput {
    fn to_openapi(self) -> PersonViewCollection {
        let persons = self.collection;
        let person_views = persons
            .into_iter()
            .map(|person_usecase_output| person_usecase_output.to_openapi())
            .collect::<Vec<PersonView>>()
            .to_vec();

        PersonViewCollection {
            persons: person_views,
            has_more: self.has_more,
            total: Some(self.total),
        }
    }
}

impl ToOpenApi<IdNumberProvider> for PersonUsecaseSharedIdNumberProvider {
    fn to_openapi(self) -> IdNumberProvider {
        match self {
            PersonUsecaseSharedIdNumberProvider::Passport => IdNumberProvider::PASSPORT,
            PersonUsecaseSharedIdNumberProvider::NationalId => IdNumberProvider::NATIONAL_ID,
        }
    }
}

impl ToOpenApi<PersonalIdNumber> for PersonUsecaseSharedIdNumber {
    fn to_openapi(self) -> PersonalIdNumber {
        PersonalIdNumber {
            id_number: self.id_number,
            id_number_provider: self.code.map(|provider| provider.to_openapi()),
            date_of_issue: self.date_of_issue,
            place_of_issue: self.place_of_issue,
        }
    }
}

impl ToOpenApi<PersonalIdNumber> for PersonalIdNumberUsecaseOutput {
    fn to_openapi(self) -> PersonalIdNumber {
        PersonalIdNumber {
            id_number: self.id_number,
            id_number_provider: self.code.as_ref().map(|provider| {
                IdNumberProvider::from_str(provider).expect("Unable to convert to IdNumberProvider")
            }),
            date_of_issue: self.date_of_issue,
            place_of_issue: self.place_of_issue,
        }
    }
}

impl ToOpenApi<Nationality> for PersonUsecaseSharedNationality {
    fn to_openapi(self) -> Nationality {
        match self {
            PersonUsecaseSharedNationality::Vietnamese => Nationality::VIETNAMESE,
            PersonUsecaseSharedNationality::French => Nationality::FRENCH,
            PersonUsecaseSharedNationality::Chinese => Nationality::CHINESE,
            PersonUsecaseSharedNationality::British => Nationality::BRITISH,
            PersonUsecaseSharedNationality::American => Nationality::AMERICAN,
        }
    }
}

impl ToOpenApi<VowProgress> for PersonUsecaseSharedVowProgress {
    fn to_openapi(self) -> VowProgress {
        match self {
            PersonUsecaseSharedVowProgress::Novice => VowProgress::NOVICE,
            PersonUsecaseSharedVowProgress::Preparation => VowProgress::PREPARATION,
            PersonUsecaseSharedVowProgress::SimpleVow => VowProgress::SIMPLE_VOW,
            PersonUsecaseSharedVowProgress::SolemnVow => VowProgress::SOLEMN_VOW,
        }
    }
}

impl ToOpenApi<Title> for PersonUsecaseSharedTitle {
    fn to_openapi(self) -> Title {
        match self {
            PersonUsecaseSharedTitle::Priest => Title::PRIEST,
            PersonUsecaseSharedTitle::Monk => Title::MONK,
            PersonUsecaseSharedTitle::Nun => Title::NUN,
        }
    }
}

impl ToOpenApi<PersonUpsertPosition> for PersonUsecaseSharedPosition {
    fn to_openapi(self) -> PersonUpsertPosition {
        PersonUpsertPosition {
            name: self.title.map(|t| t.to_openapi()).unwrap(),
            period: self.period.map(|p| p.to_openapi()),
            polity_id: self.parish,
        }
    }
}

impl ToOpenApi<EducationalLevel> for PersonUsecaseSharedEducationalLevel {
    fn to_openapi(self) -> EducationalLevel {
        match self {
            PersonUsecaseSharedEducationalLevel::Bachelor => EducationalLevel::BACHELOR,
            PersonUsecaseSharedEducationalLevel::Master => EducationalLevel::MASTER,
            PersonUsecaseSharedEducationalLevel::ElementarySchool => {
                EducationalLevel::ELEMENTARY_SCHOOL
            }
            PersonUsecaseSharedEducationalLevel::HighSchool => EducationalLevel::HIGH_SCHOOL,
            PersonUsecaseSharedEducationalLevel::Doctor => EducationalLevel::DOCTOR,
            PersonUsecaseSharedEducationalLevel::Other => EducationalLevel::OTHER,
            PersonUsecaseSharedEducationalLevel::MiddleSchool => EducationalLevel::MIDDLE_SCHOOL,
        }
    }
}

impl ToOpenApi<EducationalStage> for PersonUsecaseSharedEducationalStage {
    fn to_openapi(self) -> EducationalStage {
        EducationalStage {
            educational_level: self
                .educational_level
                .map(|level| level.to_openapi())
                .unwrap(),
            school_name: self.school_name,
            major: self.major,
            graduate_year: self.graduate_year,
        }
    }
}

impl ToOpenApi<ForeignLanguageLevel> for PersonUsecaseSharedLanguageLevel {
    fn to_openapi(self) -> ForeignLanguageLevel {
        match self {
            PersonUsecaseSharedLanguageLevel::Beginner => ForeignLanguageLevel::BEGINNER,
            PersonUsecaseSharedLanguageLevel::Intermediate => ForeignLanguageLevel::INTERMEDIATE,
            PersonUsecaseSharedLanguageLevel::Advanced => ForeignLanguageLevel::ADVANCED,
        }
    }
}

impl ToOpenApi<Language> for PersonUsecaseSharedLanguage {
    fn to_openapi(self) -> Language {
        Language {
            name: Some(self.language),
            level: Some(self.level.to_openapi()),
        }
    }
}

impl ToOpenApi<PersonView> for CreatePersonUsecaseOutput {
    fn to_openapi(self) -> PersonView {
        let mut personal_id_numbers = match self.personal_id_numbers {
            Some(personal_id_numbers_usecase_output) => personal_id_numbers_usecase_output
                .into_iter()
                .map(|id_number| id_number.to_openapi())
                .collect::<Vec<PersonalIdNumber>>()
                .to_vec(),
            _ => Vec::new(),
        };

        let mut educational_stages = None;
        if let Some(stages) = self.educational_stages {
            educational_stages = Some(stages.into_iter().map(|stage| stage.to_openapi()).collect())
        }

        let languages = match self.languages {
            Some(languages) => Some(
                languages
                    .into_iter()
                    .map(|language| language.to_openapi())
                    .collect::<Vec<Language>>()
                    .to_vec(),
            ),
            _ => None,
        };
        PersonView {
            id: self.person_id,
            date_of_birth: self.date_of_birth,
            place_of_birth: self.place_of_birth,
            email: self.email,
            phone: self.phone,
            name: Some(format!(
                "{} {} {}",
                self.last_name.unwrap_or_default(),
                self.middle_name.unwrap_or_default(),
                self.first_name.unwrap_or_default(),
            )),
            personal_id_numbers: Some(personal_id_numbers),
            christian_name: self.christian_name,
            languages,
            education_stages: educational_stages,
            position: self.position.map(|p| p.to_openapi()),
            nationality: self.nationality.map(|n| n.to_openapi()),
            race: self.race,
            address: self.address,
        }
    }
}

impl ToOpenApi<PersonView> for QueryPersonUsecaseOutput {
    fn to_openapi(self) -> PersonView {
        let mut personal_id_numbers: Vec<PersonalIdNumber> = Vec::new();
        if let Some(personal_id_numbers_usecase_output) = self.personal_id_numbers {
            for personal_id_number in personal_id_numbers_usecase_output {
                personal_id_numbers.push(personal_id_number.to_openapi())
            }
        }
        PersonView {
            id: self.id,
            date_of_birth: self.date_of_birth,
            place_of_birth: self.place_of_birth,
            email: self.email,
            phone: self.phone,
            name: Some(format!(
                "{} {} {}",
                self.last_name.unwrap(),
                self.middle_name.unwrap(),
                self.first_name.unwrap(),
            )),
            personal_id_numbers: Some(personal_id_numbers),
            christian_name: None,
            languages: None,
            education_stages: None,
            position: None,
            nationality: None,
            race: None,
            address: None,
        }
    }
}

impl ToOpenApi<PersonView> for UpdatePersonUsecaseOutput {
    fn to_openapi(self) -> PersonView {
        let mut personal_id_numbers: Vec<PersonalIdNumber> = Vec::new();
        if let Some(personal_id_numbers_usecase_output) = self.personal_id_numbers {
            for personal_id_number in personal_id_numbers_usecase_output {
                personal_id_numbers.push(personal_id_number.to_openapi())
            }
        }
        PersonView {
            id: self.person_id.unwrap(),
            name: Some(format!(
                "{} {} {}",
                self.last_name.unwrap(),
                self.middle_name.unwrap(),
                self.first_name.unwrap(),
            )),
            date_of_birth: self.date_of_birth,
            place_of_birth: self.place_of_birth,
            email: self.email,
            phone: self.phone,
            address: None,
            personal_id_numbers: Some(personal_id_numbers),
            christian_name: None,
            languages: None,
            education_stages: None,
            position: None,
            nationality: None,
            race: None,
        }
    }
}

pub struct PersonCollectionQuery {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub place_of_birth: Option<String>,
    // pub sorts: Option<Vec<PersonSortCriteria>>,
    pub offset: Option<i64>,
    pub count: Option<i64>,
}
