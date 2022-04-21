use crate::entities::vow_progress::VowProgress;
use std::fmt::Formatter;
use uuid::Uuid;

#[derive(Clone)]
pub struct Position {
    pub title: Option<Title>,
    pub period: Option<VowProgress>,
    pub parish: Option<Uuid>,
}

#[derive(PartialEq, Clone)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub enum Title {
    Priest,
    Monk,
    Nun,
}

impl std::fmt::Display for Title {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Title::Priest => write!(f, "PRIEST"),
            Title::Monk => write!(f, "MONK"),
            Title::Nun => write!(f, "NUN"),
        }
    }
}

impl std::str::FromStr for Title {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "PRIEST" => std::result::Result::Ok(Title::Priest),
            "MONK" => std::result::Result::Ok(Title::Monk),
            "NUN" => std::result::Result::Ok(Title::Nun),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}
