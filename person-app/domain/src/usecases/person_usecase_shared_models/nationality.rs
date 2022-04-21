use crate::entities::person::Nationality as NationalityEntity;
use crate::usecases::ToEntity;

#[derive(PartialEq, Clone)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub enum PersonUsecaseSharedNationality {
    Vietnamese,
    Chinese,
    British,
    American,
    French,
}

impl std::str::FromStr for PersonUsecaseSharedNationality {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "VIETNAMESE" => std::result::Result::Ok(PersonUsecaseSharedNationality::Vietnamese),
            "CHINESE" => std::result::Result::Ok(PersonUsecaseSharedNationality::Chinese),
            "BRITISH" => std::result::Result::Ok(PersonUsecaseSharedNationality::British),
            "AMERICAN" => std::result::Result::Ok(PersonUsecaseSharedNationality::American),
            "FRENCH" => std::result::Result::Ok(PersonUsecaseSharedNationality::French),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

impl ToEntity<NationalityEntity> for PersonUsecaseSharedNationality {
    fn to_entity(self) -> NationalityEntity {
        match self {
            PersonUsecaseSharedNationality::Vietnamese => NationalityEntity::Vietnamese,
            PersonUsecaseSharedNationality::Chinese => NationalityEntity::Chinese,
            PersonUsecaseSharedNationality::American => NationalityEntity::American,
            PersonUsecaseSharedNationality::French => NationalityEntity::French,
            PersonUsecaseSharedNationality::British => NationalityEntity::British,
        }
    }
}
