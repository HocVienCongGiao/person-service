use std::fmt::Formatter;

#[derive(PartialEq, Clone)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub enum VowProgress {
    SolemnVow,
    SimpleVow,
    Novice,
    Preparation,
}

impl std::fmt::Display for VowProgress {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            VowProgress::SolemnVow => write!(f, "SOLEMN_VOW"),
            VowProgress::SimpleVow => write!(f, "SIMPLE_VOW"),
            VowProgress::Novice => write!(f, "NOVICE"),
            VowProgress::Preparation => write!(f, "PREPARATION"),
        }
    }
}

impl std::str::FromStr for VowProgress {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "SOLEMN_VOW" => std::result::Result::Ok(VowProgress::SolemnVow),
            "SIMPLE_VOW" => std::result::Result::Ok(VowProgress::SimpleVow),
            "NOVICE" => std::result::Result::Ok(VowProgress::Novice),
            "PREPARATION" => std::result::Result::Ok(VowProgress::Preparation),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}
